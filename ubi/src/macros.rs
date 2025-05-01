// Internal logger macros
#[macro_export]
macro_rules! ubiinfo {
    ($($arg:tt)*) => {
        ::log::info!("[Ubi] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! ubiwarn {
    ($($arg:tt)*) => {
        ::log::warn!("[Ubi] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! ubierror {
    ($($arg:tt)*) => {
        ::log::error!("[Ubi] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! ubidebug {
    ($($arg:tt)*) => {
        ::log::debug!("[Ubi] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! ubitrace {
    ($($arg:tt)*) => {
        ::log::trace!("[Ubi] {}", format_args!($($arg)*));
    };
}

// External logger macros to be used by app

#[macro_export]
macro_rules! appinfo {
    ($($arg:tt)*) => {
        ::log::info!("[App] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! appwarn {
    ($($arg:tt)*) => {
        ::log::warn!("[App] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! apperror {
    ($($arg:tt)*) => {
        ::log::error!("[App] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! appdebug {
    ($($arg:tt)*) => {
        ::log::debug!("[App] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! apptrace {
    ($($arg:tt)*) => {
        ::log::trace!("[App] {}", format_args!($($arg)*));
    };
}
