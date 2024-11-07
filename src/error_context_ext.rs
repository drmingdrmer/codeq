use std::fmt::Display;
use std::io;

pub trait ErrorContextExt {
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
