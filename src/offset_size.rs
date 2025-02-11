mod offset;
mod size;

pub use offset::Offset;
pub use size::Size;

pub trait Span {
    fn offset(&self) -> u64;
    fn size(&self) -> u64;
    fn end(&self) -> u64 {
        self.offset() + self.size()
    }
}

impl<T> Span for &T
where T: Span
{
    fn offset(&self) -> u64 {
        (*self).offset()
    }

    fn size(&self) -> u64 {
        (*self).size()
    }
}
