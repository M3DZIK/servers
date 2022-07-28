/// Update value in the Mutex
///
/// ```
/// use std::sync::Mutex;
/// use servers::update_mutex;
///
/// // create a new Mutex
/// let mutex: Mutex<String> = Mutex::new(String::new());
///
/// // update the value in the Mutex
/// update_mutex!(mutex, "new value".to_string());
/// ```
#[macro_export]
macro_rules! update_mutex {
    ($mutex: expr, $($new_value:tt)+) => {
        *$crate::lock_mutex!($mutex) = $($new_value)+;
    };
}

/// Lock value in the Mutex
///
/// ```
/// use std::sync::Mutex;
/// use servers::lock_mutex;
///
/// // create a new Mutex
/// let mutex: Mutex<String> = Mutex::new("value".to_string());
///
/// // lock the Mutex
/// let value = lock_mutex!(mutex);
///
/// println!("{}", value);
/// ```
#[macro_export]
macro_rules! lock_mutex {
    ($mutex: expr) => {
        $mutex.lock().expect("failed to lock mutex")
    };
}
