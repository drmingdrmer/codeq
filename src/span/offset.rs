use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::Sub;
use std::ops::SubAssign;

use derive_more::Display;
use derive_more::From;

use crate::span::Size;

#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(From)]
#[derive(Display)]
#[display("Offset({_0})")]
pub struct Offset(pub u64);

impl Add<Size> for Offset {
    type Output = Offset;

    fn add(self, rhs: Size) -> Self::Output {
        Offset(*self + *rhs)
    }
}

impl AddAssign<Size> for Offset {
    fn add_assign(&mut self, rhs: Size) {
        *self = Offset(self.0 + *rhs);
    }
}

impl Sub<Size> for Offset {
    type Output = Offset;

    fn sub(self, rhs: Size) -> Self::Output {
        Offset(*self - *rhs)
    }
}

impl SubAssign<Size> for Offset {
    fn sub_assign(&mut self, rhs: Size) {
        *self = Offset(self.0 - *rhs);
    }
}

impl Sub for Offset {
    type Output = Size;

    fn sub(self, rhs: Self) -> Self::Output {
        Size(*self - *rhs)
    }
}

impl Deref for Offset {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Offset;
    use crate::span::Size;

    #[test]
    fn test_display() {
        assert_eq!(format!("{:?}", Offset(42)), "Offset(42)");
        assert_eq!(format!("{}", Offset(42)), "Offset(42)");
    }

    #[allow(clippy::clone_on_copy)]
    #[test]
    fn test_calculation() {
        let mut a = Offset(42);
        let b = Offset(10);

        // Add Sub

        assert_eq!(a + Size(3), Offset(45));
        assert_eq!(a - Size(3), Offset(39));

        a += Size(3);
        assert_eq!(a, Offset(45));
        a -= Size(3);
        assert_eq!(a, Offset(42));

        // Eq, Ord

        assert_eq!(a, Offset(42));
        assert!(a > b);

        // From

        let c = Offset::from(1u64);

        // Clone Copy

        let d = c.clone();
        assert_eq!(d, c);

        let d = c;
        assert_eq!(d, c);

        // Default

        assert_eq!(Offset::default(), Offset(0));

        // Deref

        assert_eq!(*Offset(1u64), 1u64);
    }
}
