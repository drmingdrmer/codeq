mod offset;
mod size;

pub use offset::Offset;
pub use size::Size;

pub trait OffsetSize {
    fn offset(&self) -> u64;
    fn size(&self) -> u64;
    fn end(&self) -> u64 {
        self.offset() + self.size()
    }
}

impl<T> OffsetSize for &T
where T: OffsetSize
{
    fn offset(&self) -> u64 {
        (*self).offset()
    }

    fn size(&self) -> u64 {
        (*self).size()
    }
}
