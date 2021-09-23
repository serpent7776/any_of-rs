struct Tuple<Tuple> {
    tuple: Tuple,
}

impl<X, T0> PartialEq<X> for Tuple<(T0,)>
where
    T0: std::cmp::PartialEq<X>,
{
    fn eq(&self, value: &X) -> bool {
        self.tuple.0 == *value
    }
}
impl<X, T0, T1> PartialEq<X> for Tuple<(T0, T1)>
where
    T0: std::cmp::PartialEq<X>,
    T1: std::cmp::PartialEq<X>,
{
    fn eq(&self, value: &X) -> bool {
        self.tuple.0 == *value || self.tuple.1 == *value
    }
}
impl<X, T0, T1, T2> PartialEq<X> for Tuple<(T0, T1, T2)>
where
    T0: std::cmp::PartialEq<X>,
    T1: std::cmp::PartialEq<X>,
    T2: std::cmp::PartialEq<X>,
{
    fn eq(&self, value: &X) -> bool {
        self.tuple.0 == *value || self.tuple.1 == *value || self.tuple.2 == *value
    }
}

macro_rules! any_of {
    ($($value: literal),+) => {
        Tuple {
            tuple : ($($value, )+)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_of_with_single_int_matches_its_int() {
        assert!(any_of!(1) == 1);
    }

    #[test]
    fn any_of_with_single_int_doesnt_match_other_int() {
        assert!(!(any_of!(1) == 7));
    }

    #[test]
    fn any_of_with_two_ints_matches_first_int() {
        assert!(any_of!(1, 2) == 1);
    }

    #[test]
    fn any_of_with_two_ints_matches_second_int() {
        assert!(any_of!(1, 2) == 2);
    }

    #[test]
    fn any_of_with_two_ints_doesnt_match_other_int() {
        assert!(!(any_of!(1, 2) == 7));
    }

    #[test]
    fn any_of_with_three_ints_matches_first_int() {
        assert!(any_of!(1, 2, 3) == 1);
    }

    #[test]
    fn any_of_with_three_ints_matches_second_int() {
        assert!(any_of!(1, 2, 3) == 2);
    }

    #[test]
    fn any_of_with_three_ints_matches_third_int() {
        assert!(any_of!(1, 2, 3) == 3);
    }

    #[test]
    fn any_of_with_three_ints_doesnt_match_other_int() {
        assert!(!(any_of!(1, 2, 3) == 7));
    }

}
