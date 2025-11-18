//! ChangeNotifier - Base trait for observable objects
//!
//! ChangeNotifier provides a flexible way to create custom observable objects
//! that can notify listeners when their state changes. This is similar to
//! Flutter's ChangeNotifier.

use std::sync::{Arc, Mutex};

type ChangeListener = Box<dyn Fn() + Send + Sync>;

/// Trait for objects that can notify listeners when they change
///
/// Implement this trait to create custom observable objects that can
/// notify multiple listeners when their internal state changes.
///
/// # Example
///
/// ```
/// use kobalt_core::state::ChangeNotifier;
///
/// struct Counter {
///     notifier: ChangeNotifier,
///     count: i32,
/// }
///
/// impl Counter {
///     fn new() -> Self {
///         Self {
///             notifier: ChangeNotifier::new(),
///             count: 0,
///         }
///     }
///
///     fn increment(&mut self) {
///         self.count += 1;
///         self.notifier.notify();
///     }
///
///     fn get_count(&self) -> i32 {
///         self.count
///     }
///
///     fn add_listener<F>(&self, listener: F)
///     where
///         F: Fn() + Send + Sync + 'static,
///     {
///         self.notifier.add_listener(listener);
///     }
/// }
/// ```
pub struct ChangeNotifier {
    listeners: Arc<Mutex<Vec<ChangeListener>>>,
}

impl ChangeNotifier {
    /// Creates a new ChangeNotifier
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a listener that will be called when notify() is invoked
    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.push(Box::new(listener));
    }

    /// Notifies all listeners that a change has occurred
    pub fn notify(&self) {
        let listeners = self.listeners.lock().unwrap();
        for listener in listeners.iter() {
            listener();
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

impl Default for ChangeNotifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ChangeNotifier {
    fn clone(&self) -> Self {
        Self {
            listeners: Arc::clone(&self.listeners),
        }
    }
}

impl std::fmt::Debug for ChangeNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChangeNotifier")
            .field("listener_count", &self.listener_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    struct TestCounter {
        notifier: ChangeNotifier,
        count: i32,
    }

    impl TestCounter {
        fn new() -> Self {
            Self {
                notifier: ChangeNotifier::new(),
                count: 0,
            }
        }

        fn increment(&mut self) {
            self.count += 1;
            self.notifier.notify();
        }

        fn get_count(&self) -> i32 {
            self.count
        }

        fn add_listener<F>(&self, listener: F)
        where
            F: Fn() + Send + Sync + 'static,
        {
            self.notifier.add_listener(listener);
        }
    }

    #[test]
    fn test_change_notifier_basic() {
        let notifier = ChangeNotifier::new();
        let counter = Arc::new(AtomicI32::new(0));

        let c = Arc::clone(&counter);
        notifier.add_listener(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });

        notifier.notify();
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        notifier.notify();
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_change_notifier_multiple_listeners() {
        let notifier = ChangeNotifier::new();
        let counter1 = Arc::new(AtomicI32::new(0));
        let counter2 = Arc::new(AtomicI32::new(0));

        let c1 = Arc::clone(&counter1);
        notifier.add_listener(move || {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let c2 = Arc::clone(&counter2);
        notifier.add_listener(move || {
            c2.fetch_add(10, Ordering::SeqCst);
        });

        notifier.notify();
        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_change_notifier_clear() {
        let notifier = ChangeNotifier::new();
        let counter = Arc::new(AtomicI32::new(0));

        let c = Arc::clone(&counter);
        notifier.add_listener(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });

        notifier.notify();
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        notifier.clear_listeners();
        notifier.notify();
        // Counter should still be 1 since listeners were cleared
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_change_notifier_listener_count() {
        let notifier = ChangeNotifier::new();
        assert_eq!(notifier.listener_count(), 0);

        notifier.add_listener(|| {});
        assert_eq!(notifier.listener_count(), 1);

        notifier.add_listener(|| {});
        assert_eq!(notifier.listener_count(), 2);

        notifier.clear_listeners();
        assert_eq!(notifier.listener_count(), 0);
    }

    #[test]
    fn test_change_notifier_with_struct() {
        let mut counter = TestCounter::new();
        let call_count = Arc::new(AtomicI32::new(0));

        let c = Arc::clone(&call_count);
        counter.add_listener(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(counter.get_count(), 0);
        assert_eq!(call_count.load(Ordering::SeqCst), 0);

        counter.increment();
        assert_eq!(counter.get_count(), 1);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        counter.increment();
        assert_eq!(counter.get_count(), 2);
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_change_notifier_clone() {
        let notifier1 = ChangeNotifier::new();
        let notifier2 = notifier1.clone();
        let counter = Arc::new(AtomicI32::new(0));

        let c = Arc::clone(&counter);
        notifier1.add_listener(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });

        notifier2.notify();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
