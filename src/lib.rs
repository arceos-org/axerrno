#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

pub use axerrno::{AxErrorKind as AxError, LinuxError};

/// A specialized [`Result`] type with [`AxError`] as the error type.
pub type AxResult<T = ()> = Result<T, AxError>;

/// A specialized [`Result`] type with [`LinuxError`] as the error type.
pub type LinuxResult<T = ()> = Result<T, LinuxError>;

/// Convenience method to construct an [`AxError`] type while printing a warning
/// message.
///
/// # Examples
///
/// ```
/// # use axerrno::{ax_err_type, AxError};
/// #
/// // Also print "[AxError::AlreadyExists]" if the `log` crate is enabled.
/// assert_eq!(
///     ax_err_type!(AlreadyExists),
///     AxError::AlreadyExists,
/// );
///
/// // Also print "[AxError::BadAddress] the address is 0!" if the `log` crate
/// // is enabled.
/// assert_eq!(
///     ax_err_type!(BadAddress, "the address is 0!"),
///     AxError::BadAddress,
/// );
/// ```
#[macro_export]
macro_rules! ax_err_type {
    ($err: ident) => {{
        use $crate::AxError::*;
        $crate::__priv::warn!("[AxError::{:?}]", $err);
        $err
    }};
    ($err: ident, $msg: expr) => {{
        use $crate::AxError::*;
        $crate::__priv::warn!("[AxError::{:?}] {}", $err, $msg);
        $err
    }};
}

/// Ensure a condition is true. If it is not, return from the function
/// with an error.
///
/// ## Examples
///
/// ```rust
/// # use axerrno::{ensure, ax_err, AxError, AxResult};
///
/// fn example(user_id: i32) -> AxResult {
///     ensure!(user_id > 0, ax_err!(InvalidInput));
///     // After this point, we know that `user_id` is positive.
///     let user_id = user_id as u32;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($predicate:expr, $context_selector:expr $(,)?) => {
        if !$predicate {
            return $context_selector;
        }
    };
}

/// Convenience method to construct an [`Err(AxError)`] type while printing a
/// warning message.
///
/// # Examples
///
/// ```
/// # use axerrno::{ax_err, AxResult, AxError};
/// #
/// // Also print "[AxError::AlreadyExists]" if the `log` crate is enabled.
/// assert_eq!(
///     ax_err!(AlreadyExists),
///     AxResult::<()>::Err(AxError::AlreadyExists),
/// );
///
/// // Also print "[AxError::BadAddress] the address is 0!" if the `log` crate is enabled.
/// assert_eq!(
///     ax_err!(BadAddress, "the address is 0!"),
///     AxResult::<()>::Err(AxError::BadAddress),
/// );
/// ```
/// [`Err(AxError)`]: Err
#[macro_export]
macro_rules! ax_err {
    ($err: ident) => {
        Err($crate::ax_err_type!($err))
    };
    ($err: ident, $msg: expr) => {
        Err($crate::ax_err_type!($err, $msg))
    };
}

/// Throws an error of type [`AxError`] with the given error code, optionally
/// with a message.
#[macro_export]
macro_rules! ax_bail {
    ($($t:tt)*) => {
        return $crate::ax_err!($($t)*);
    };
}

#[doc(hidden)]
pub mod __priv {
    pub use log::warn;
}
