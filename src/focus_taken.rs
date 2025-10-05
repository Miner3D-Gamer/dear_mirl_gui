
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// How much focus has been took
pub enum FocusTaken {
    /// No one ha taken focus previously, continue
    FocusFree,
    /// Something took visual focus -> E.g. hovering over a button
    VisuallyTaken,
    /// Something took visual focus -> E.g. clicking a button
    FunctionallyTaken,
}
impl FocusTaken {
    /// If the focus is not free
    #[must_use]
    pub fn is_focus_taken(self) -> bool {
        self != Self::FocusFree
    }
}
impl std::ops::BitOr for FocusTaken {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        if self == Self::FunctionallyTaken || rhs == Self::FunctionallyTaken {
            Self::FunctionallyTaken
        } else if self == Self::VisuallyTaken || rhs == Self::VisuallyTaken {
            Self::VisuallyTaken
        } else {
            Self::FocusFree
        }
    }
}
impl std::ops::BitOrAssign for FocusTaken {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}
impl std::ops::Not for FocusTaken {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.is_focus_taken()
    }
}
impl PartialOrd for FocusTaken {
    fn ge(&self, other: &Self) -> bool {
        match self {
            Self::FocusFree => *other == Self::FocusFree,
            Self::VisuallyTaken => *other != Self::FocusFree,
            Self::FunctionallyTaken => *other == Self::FunctionallyTaken,
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match self {
            Self::FocusFree => false,
            Self::VisuallyTaken => *other == Self::FocusFree,
            Self::FunctionallyTaken => *other != Self::FunctionallyTaken,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self {
            Self::FocusFree => *other == Self::FocusFree,
            Self::VisuallyTaken => *other != Self::FunctionallyTaken,
            Self::FunctionallyTaken => *other == Self::FunctionallyTaken,
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match self {
            Self::FocusFree => *other != Self::FocusFree,
            Self::VisuallyTaken => *other == Self::FunctionallyTaken,
            Self::FunctionallyTaken => false,
        }
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if *self == Self::FocusFree {
            Some(std::cmp::Ordering::Less)
        } else if *self == Self::FunctionallyTaken || *other == Self::FocusFree
        {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}