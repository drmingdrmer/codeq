use std::ops::Deref;

use derive_more::Add;
use derive_more::AddAssign;
use derive_more::Display;
use derive_more::From;
use derive_more::Sub;
use derive_more::SubAssign;

#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(From)]
#[derive(Add, AddAssign, Sub, SubAssign)]
#[derive(Display)]
#[display("Size({_0})")]
pub struct Size(pub u64);

impl Deref for Size {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    fn test_display() {
        assert_eq!(format!("{:?}", Size(42)), "Size(42)");
        assert_eq!(format!("{}", Size(42)), "Size(42)");
    }

    #[allow(clippy::clone_on_copy)]
    #[test]
    fn test_calculation() {
        let mut a = Size(42);
        let b = Size(10);

        // Add Sub

        assert_eq!(a + b, Size(52));
        assert_eq!(a - b, Size(32));

        a += b;
        assert_eq!(a, Size(52));
        a -= b;
        assert_eq!(a, Size(42));

        // Eq, Ord

        assert_eq!(a, Size(42));
        assert!(a > b);

        // From

        let c = Size::from(1u64);

        // Clone Copy

        let d = c.clone();
        assert_eq!(d, c);

        let d = c;
        assert_eq!(d, c);

        // Default

        assert_eq!(Size::default(), Size(0));

        // Deref

        assert_eq!(*Size(1u64), 1u64);
    }
}
