use std::cmp::Ordering;
use std::ops::Deref;
/* The default ordering of `Option`s is `None` being less than `Some`. The purpose of this struct is
to reverse that. */
#[derive(PartialEq)]
pub(crate) struct ReverseOrdOption<'a, T>(&'a Option<T>);

impl<T> Deref for ReverseOrdOption<'_, T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Ord> Eq for ReverseOrdOption<'_, T> {}

impl<T: Ord> PartialOrd<Self> for ReverseOrdOption<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for ReverseOrdOption<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.as_ref(), other.as_ref()) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(self_time), Some(other_time)) => self_time.cmp(other_time),
        }
    }
}

impl<'a, T> From<&'a Option<T>> for ReverseOrdOption<'a, T> {
    fn from(value: &'a Option<T>) -> Self {
        ReverseOrdOption(value)
    }
}
