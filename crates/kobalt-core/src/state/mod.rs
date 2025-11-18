//! State management system
//!
//! This module provides reactive state management:
//! - `LiveData<T>`: Observable data container with thread-safe listeners
//! - `ValueNotifier<T>`: Simple value notifier
//! - `ChangeNotifier`: Base trait for notifying changes

mod live_data;
mod value_notifier;
mod change_notifier;

pub use live_data::LiveData;
pub use value_notifier::ValueNotifier;
