//! LiveData - Thread-safe observable data container
//!
//! LiveData is inspired by Android's LiveData and provides a way to hold
//! observable data that notifies listeners when the value changes.

use std::sync::{Arc, Mutex};

type Listener<T> = Box<dyn Fn(&T) + Send + Sync>;

/// Thread-safe observable data container that notifies listeners on value changes
///
/// # Example
///
/// ```
/// use kobalt_core::state::LiveData;
/// use std::sync::Arc;
///
/// let live_data = Arc::new(LiveData::new(0));
/// let live_data_clone = live_data.clone();
///
/// // Add observer
/// live_data.observe(move |value| {
///     println!("Value changed to: {}", value);
/// });
///
/// // Update value (will trigger observer)
/// live_data_clone.set(42);
/// ```
pub struct LiveData<T> {
    value: Arc<Mutex<T>>,
    listeners: Arc<Mutex<Vec<Listener<T>>>>,
}

impl<T> LiveData<T>
where
    T: Clone + Send + Sync,
{
    /// Creates a new LiveData with the given initial value
    pub fn new(initial_value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial_value)),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Gets the current value
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    /// Sets a new value and notifies all observers
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned
    pub fn set(&self, new_value: T) {
        {
            let mut value = self.value.lock().unwrap();
            *value = new_value;
        }
        self.notify();
    }

    /// Updates the value using a closure and notifies observers
    ///
    /// # Example
    ///
    /// ```
    /// use kobalt_core::state::LiveData;
    ///
    /// let live_data = LiveData::new(5);
    /// live_data.update(|v| *v += 10);
    /// assert_eq!(live_data.get(), 15);
    /// ```
    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.lock().unwrap();
            updater(&mut *value);
        }
        self.notify();
    }

    /// Adds an observer that will be called when the value changes
    ///
    /// Returns the index of the observer, which can be used to remove it later
    pub fn observe<F>(&self, observer: F) -> usize
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.push(Box::new(observer));
        listeners.len() - 1
    }

    /// Notifies all observers with the current value
    fn notify(&self) {
        let value = self.value.lock().unwrap();
        let listeners = self.listeners.lock().unwrap();

        for listener in listeners.iter() {
            listener(&*value);
        }
    }

    /// Removes all observers
    pub fn clear_observers(&self) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.clear();
    }

    /// Returns the number of active observers
    pub fn observer_count(&self) -> usize {
        self.listeners.lock().unwrap().len()
    }
}

impl<T> Clone for LiveData<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            listeners: Arc::clone(&self.listeners),
        }
    }
}

impl<T> std::fmt::Debug for LiveData<T>
where
    T: std::fmt::Debug + Clone + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiveData")
            .field("value", &self.get())
            .field("observer_count", &self.observer_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn test_live_data_basic() {
        let live_data = LiveData::new(10);
        assert_eq!(live_data.get(), 10);

        live_data.set(20);
        assert_eq!(live_data.get(), 20);
    }

    #[test]
    fn test_live_data_observer() {
        let live_data = Arc::new(LiveData::new(0));
        let counter = Arc::new(AtomicI32::new(0));

        let counter_clone = Arc::clone(&counter);
        live_data.observe(move |value| {
            counter_clone.store(*value, Ordering::SeqCst);
        });

        live_data.set(42);
        assert_eq!(counter.load(Ordering::SeqCst), 42);

        live_data.set(100);
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_live_data_multiple_observers() {
        let live_data = Arc::new(LiveData::new(0));
        let counter1 = Arc::new(AtomicI32::new(0));
        let counter2 = Arc::new(AtomicI32::new(0));

        let c1 = Arc::clone(&counter1);
        live_data.observe(move |value| {
            c1.store(*value, Ordering::SeqCst);
        });

        let c2 = Arc::clone(&counter2);
        live_data.observe(move |value| {
            c2.store(*value * 2, Ordering::SeqCst);
        });

        live_data.set(10);
        assert_eq!(counter1.load(Ordering::SeqCst), 10);
        assert_eq!(counter2.load(Ordering::SeqCst), 20);
    }

    #[test]
    fn test_live_data_update() {
        let live_data = LiveData::new(5);

        live_data.update(|v| *v += 10);
        assert_eq!(live_data.get(), 15);

        live_data.update(|v| *v *= 2);
        assert_eq!(live_data.get(), 30);
    }

    #[test]
    fn test_live_data_clear_observers() {
        let live_data = Arc::new(LiveData::new(0));
        let counter = Arc::new(AtomicI32::new(0));

        let c = Arc::clone(&counter);
        live_data.observe(move |value| {
            c.store(*value, Ordering::SeqCst);
        });

        live_data.set(10);
        assert_eq!(counter.load(Ordering::SeqCst), 10);

        live_data.clear_observers();
        live_data.set(20);
        // Counter should still be 10 since observer was cleared
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_live_data_observer_count() {
        let live_data = LiveData::new(0);
        assert_eq!(live_data.observer_count(), 0);

        live_data.observe(|_| {});
        assert_eq!(live_data.observer_count(), 1);

        live_data.observe(|_| {});
        assert_eq!(live_data.observer_count(), 2);

        live_data.clear_observers();
        assert_eq!(live_data.observer_count(), 0);
    }

    #[test]
    fn test_live_data_clone() {
        let live_data1 = Arc::new(LiveData::new(10));
        let live_data2 = live_data1.clone();

        live_data1.set(20);
        assert_eq!(live_data2.get(), 20);
    }

    #[test]
    fn test_live_data_string() {
        let live_data = Arc::new(LiveData::new(String::from("Hello")));
        let result = Arc::new(Mutex::new(String::new()));

        let r = Arc::clone(&result);
        live_data.observe(move |value| {
            *r.lock().unwrap() = value.clone();
        });

        live_data.set(String::from("World"));
        assert_eq!(*result.lock().unwrap(), "World");
    }
}
