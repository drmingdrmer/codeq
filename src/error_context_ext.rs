use std::io;

pub trait ErrorContextExt {
    fn context(self, ctx: impl ToString) -> Self;
}

impl ErrorContextExt for io::Error {
    fn context(self, ctx: impl ToString) -> Self {
        io::Error::new(self.kind(), format!("{}; when:({})", self, ctx.to_string()))
    }
}

impl<T> ErrorContextExt for Result<T, io::Error> {
    fn context(self, ctx: impl ToString) -> Self {
        self.map_err(|e| e.context(ctx))
    }
}
