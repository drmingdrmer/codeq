use std::fmt::Display;
use std::io;

/// Trait for adding context to error messages.
///
/// This trait allows adding additional context to error messages by providing
/// a closure that returns a displayable context string.
///
/// Example of adding context to an error:
/// ```rust
/// # use std::io;
/// # use codeq::error_context_ext::ErrorContextExt;
///
/// let err = io::Error::new(io::ErrorKind::Other, "some error");
/// let err = err.context(|| "some context");
/// assert_eq!(err.to_string(), "some error; when:(some context)");
/// ```
///
/// Example of adding context to a result:
/// ```rust
/// # use std::io;
/// # use codeq::error_context_ext::ErrorContextExt;
///
/// let res = Result::<(), io::Error>::Err(io::Error::new(io::ErrorKind::Other, "some error"));
/// let res = res.context(|| "some context");
/// assert_eq!(res.unwrap_err().to_string(), "some error; when:(some context)");
/// ```
pub trait ErrorContextExt {
    /// Adds context to the error message.
    ///
    /// # Arguments
    /// * `ctx` - A closure that returns a displayable context string
    fn context<D: Display>(self, ctx: impl FnOnce() -> D) -> Self;
}

impl ErrorContextExt for io::Error {
    fn context<D: Display>(self, ctx: impl FnOnce() -> D) -> Self {
        io::Error::new(self.kind(), format!("{}; when:({})", self, ctx()))
    }
}

impl<T> ErrorContextExt for Result<T, io::Error> {
    fn context<D: Display>(self, ctx: impl FnOnce() -> D) -> Self {
        self.map_err(|e| e.context(ctx))
    }
}
