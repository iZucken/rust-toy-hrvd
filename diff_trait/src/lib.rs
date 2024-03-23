use std::fmt::Display;

pub trait Diffable {
    fn diff_to(self: &Self, other: &Self) -> String {
        "uncomparable".to_string()
    }
}

impl<T: Display + PartialEq> Diffable for T {
    fn diff_to(self: &Self, other: &Self) -> String {
        if self == other {
            self.to_string()
        } else {
            format!("{} -> {}", self, other)
        }
    }
}
