//! ValueNotifier - Simple observable value container
//!
//! ValueNotifier is a lightweight observable similar to Flutter's ValueNotifier.
//! It's simpler than LiveData and suitable for local state management.

use std::sync::{Arc, Mutex};

type Listener<T> = Box<dyn Fn(&T)>;

/// Simple observable value container that notifies listeners on changes
///
/// ValueNotifier is lighter than LiveData and doesn't require Send + Sync bounds
/// on the listener closures, making it more suitable for single-threaded scenarios.
///
/// # Example
///
/// ```
/// use kobalt_core::state::ValueNotifier;
/// use std::sync::Arc;
/// use std::cell::RefCell;
///
/// let notifier = ValueNotifier::new(0);
/// let captured = Arc::new(RefCell::new(0));
///
/// let c = captured.clone();
/// notifier.add_listener(move |value| {
///     *c.borrow_mut() = *value;
/// });
///
/// notifier.set(42);
/// assert_eq!(*captured.borrow(), 42);
/// ```
pub struct ValueNotifier<T> {
    value: Arc<Mutex<T>>,
    listeners: Arc<Mutex<Vec<Listener<T>>>>,
}

impl<T> ValueNotifier<T>
where
    T: Clone,
{
    /// Creates a new ValueNotifier with the given initial value
    pub fn new(initial_value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial_value)),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Gets the current value
    pub fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    /// Sets a new value and notifies all listeners
    pub fn set(&self, new_value: T) {
        {
            let mut value = self.value.lock().unwrap();
            *value = new_value;
        }
        self.notify_listeners();
    }

    /// Updates the value using a closure and notifies listeners
    ///
    /// # Example
    ///
    /// ```
    /// use kobalt_core::state::ValueNotifier;
    ///
    /// let notifier = ValueNotifier::new(5);
    /// notifier.update(|v| *v += 10);
    /// assert_eq!(notifier.get(), 15);
    /// ```
    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.lock().unwrap();
            updater(&mut *value);
        }
        self.notify_listeners();
    }

    /// Adds a listener that will be called when the value changes
    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(&T) + 'static,
    {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.push(Box::new(listener));
    }

    /// Notifies all listeners with the current value
    fn notify_listeners(&self) {
        let value = self.value.lock().unwrap();
        let listeners = self.listeners.lock().unwrap();

        for listener in listeners.iter() {
            listener(&*value);
        }
    }

    /// Removes all listeners
    pub fn clear_listeners(&self) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.clear();
    }

    /// Returns the number of active listeners
    pub fn listener_count(&self) -> usize {
        self.listeners.lock().unwrap().len()
    }
}

impl<T> Clone for ValueNotifier<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            listeners: Arc::clone(&self.listeners),
        }
    }
}

impl<T> std::fmt::Debug for ValueNotifier<T>
where
    T: std::fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueNotifier")
            .field("value", &self.get())
            .field("listener_count", &self.listener_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_value_notifier_basic() {
        let notifier = ValueNotifier::new(10);
        assert_eq!(notifier.get(), 10);

        notifier.set(20);
        assert_eq!(notifier.get(), 20);
    }

    #[test]
    fn test_value_notifier_listener() {
        let notifier = ValueNotifier::new(0);
        let captured = Arc::new(RefCell::new(0));

        let c = captured.clone();
        notifier.add_listener(move |value| {
            *c.borrow_mut() = *value;
        });

        notifier.set(42);
        assert_eq!(*captured.borrow(), 42);

        notifier.set(100);
        assert_eq!(*captured.borrow(), 100);
    }

    #[test]
    fn test_value_notifier_multiple_listeners() {
        let notifier = ValueNotifier::new(0);
        let captured1 = Arc::new(RefCell::new(0));
        let captured2 = Arc::new(RefCell::new(0));

        let c1 = captured1.clone();
        notifier.add_listener(move |value| {
            *c1.borrow_mut() = *value;
        });

        let c2 = captured2.clone();
        notifier.add_listener(move |value| {
            *c2.borrow_mut() = *value * 2;
        });

        notifier.set(10);
        assert_eq!(*captured1.borrow(), 10);
        assert_eq!(*captured2.borrow(), 20);
    }

    #[test]
    fn test_value_notifier_update() {
        let notifier = ValueNotifier::new(5);

        notifier.update(|v| *v += 10);
        assert_eq!(notifier.get(), 15);

        notifier.update(|v| *v *= 2);
        assert_eq!(notifier.get(), 30);
    }

    #[test]
    fn test_value_notifier_clear_listeners() {
        let notifier = ValueNotifier::new(0);
        let captured = Arc::new(RefCell::new(0));

        let c = captured.clone();
        notifier.add_listener(move |value| {
            *c.borrow_mut() = *value;
        });

        notifier.set(10);
        assert_eq!(*captured.borrow(), 10);

        notifier.clear_listeners();
        notifier.set(20);
        // Captured should still be 10 since listener was cleared
        assert_eq!(*captured.borrow(), 10);
    }

    #[test]
    fn test_value_notifier_listener_count() {
        let notifier = ValueNotifier::new(0);
        assert_eq!(notifier.listener_count(), 0);

        notifier.add_listener(|_| {});
        assert_eq!(notifier.listener_count(), 1);

        notifier.add_listener(|_| {});
        assert_eq!(notifier.listener_count(), 2);

        notifier.clear_listeners();
        assert_eq!(notifier.listener_count(), 0);
    }

    #[test]
    fn test_value_notifier_clone() {
        let notifier1 = ValueNotifier::new(10);
        let notifier2 = notifier1.clone();

        notifier1.set(20);
        assert_eq!(notifier2.get(), 20);
    }

    #[test]
    fn test_value_notifier_string() {
        let notifier = ValueNotifier::new(String::from("Hello"));
        let captured = Arc::new(RefCell::new(String::new()));

        let c = captured.clone();
        notifier.add_listener(move |value| {
            *c.borrow_mut() = value.clone();
        });

        notifier.set(String::from("World"));
        assert_eq!(*captured.borrow(), "World");
    }
}
