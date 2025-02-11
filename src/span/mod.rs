mod offset;
mod size;

pub use offset::Offset;
pub use size::Size;

/// A trait for types that span a range with an offset and size
pub trait Span {
    /// Returns the starting offset of the span
    fn offset(&self) -> Offset;

    /// Returns the size of the span
    fn size(&self) -> Size;

    /// Returns the starting offset of the span.
    /// This is an alias for [`offset()`](Self::offset).
    fn start(&self) -> Offset {
        self.offset()
    }

    /// Returns the end offset of the span (offset + size)
    fn end(&self) -> Offset {
        self.offset() + self.size()
    }
}

impl<T> Span for &T
where T: Span
{
    fn offset(&self) -> Offset {
        (*self).offset()
    }

    fn size(&self) -> Size {
        (*self).size()
    }
}
