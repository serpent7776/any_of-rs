struct AnyOfPack<Tuple> {
    tuple: Tuple,
}

struct NoneOfPack<Tuple> {
    tuple: Tuple,
}

struct AllOfPack<Tuple> {
    tuple: Tuple,
}

macro_rules! make_partialeq {
    ($pack: ident, $fn: ident, $map: ident, $reduce: tt, $(($t: ident, $n: tt)),+) => {
        impl<Value, $($t, )+> PartialEq<Value> for $pack<($($t, )+)>
        where
            $($t: std::cmp::PartialEq<Value>, )+
        {
            fn $fn(&self, value: &Value) -> bool {
                $reduce!($($map(&self.tuple.$n, value)),+)
            }
        }
    }
}

fn equals<Tuple, Value>(lhs: &Tuple, rhs: &Value) -> bool
where
    Tuple: std::cmp::PartialEq<Value>,
{
    lhs == rhs
}

fn not_equals<Tuple, Value>(lhs: &Tuple, rhs: &Value) -> bool
where
    Tuple: std::cmp::PartialEq<Value>,
{
    lhs != rhs
}

macro_rules! or {
    ($($v: expr),+) => {
        $($v ||)+ false
    }
}

macro_rules! and {
    ($($v: expr),+) => {
        $($v &&)+ true
    }
}

make_partialeq!(AnyOfPack, eq, equals, or, (T0, 0));
make_partialeq!(AnyOfPack, eq, equals, or, (T0, 0), (T1, 1));
make_partialeq!(AnyOfPack, eq, equals, or, (T0, 0), (T1, 1), (T2, 2));

make_partialeq!(NoneOfPack, eq, not_equals, and, (T0, 0));
make_partialeq!(NoneOfPack, eq, not_equals, and, (T0, 0), (T1, 1));
make_partialeq!(NoneOfPack, eq, not_equals, and, (T0, 0), (T1, 1), (T2, 2));

make_partialeq!(AllOfPack, eq, equals, and, (T0, 0));
make_partialeq!(AllOfPack, eq, equals, and, (T0, 0), (T1, 1));
make_partialeq!(AllOfPack, eq, equals, and, (T0, 0), (T1, 1), (T2, 2));

macro_rules! any_of {
    ($($value: literal),+) => {
        AnyOfPack {
            tuple : ($($value, )+)
        }
    };
}

macro_rules! none_of {
    ($($value: literal),+) => {
        NoneOfPack {
            tuple : ($($value, )+)
        }
    };
}

macro_rules! all_of {
    ($($value: literal),+) => {
        AllOfPack {
            tuple : ($($value, )+)
        }
    };
}

#[cfg(test)]
mod any_of_tests {
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

    #[test]
    fn any_of_with_two_strings_matches_first_string() {
        assert!(any_of!("a", "b") == "a");
    }

    #[test]
    fn any_of_with_two_strings_matches_second_string() {
        assert!(any_of!("a", "b") == "b");
    }

    #[test]
    fn any_of_with_two_strings_doesnt_match_other_string() {
        assert!(!(any_of!("a", "b") == "x"));
    }

}

#[cfg(test)]
mod none_of_tests {
    use super::*;

    #[test]
    fn none_of_with_single_int_doesnt_match_its_int() {
        assert!(!(none_of!(1) == 1));
    }

    #[test]
    fn none_of_with_single_int_matches_other_int() {
        assert!(none_of!(1) == 7);
    }

    #[test]
    fn none_of_with_two_ints_doesnt_match_first_int() {
        assert!(!(none_of!(1, 2) == 1));
    }

    #[test]
    fn none_of_with_two_ints_doesnt_match_second_int() {
        assert!(!(none_of!(1, 2) == 2));
    }

    #[test]
    fn none_of_with_two_ints_matches_other_int() {
        assert!(none_of!(1, 2) == 7);
    }

    #[test]
    fn none_of_with_three_ints_doesnt_match_first_int() {
        assert!(!(none_of!(1, 2, 3) == 1));
    }

    #[test]
    fn none_of_with_three_ints_doesnt_match_second_int() {
        assert!(!(none_of!(1, 2, 3) == 2));
    }

    #[test]
    fn none_of_with_three_ints_doesnt_match_third_int() {
        assert!(!(none_of!(1, 2, 3) == 3));
    }

    #[test]
    fn none_of_with_three_ints_matches_other_int() {
        assert!(none_of!(1, 2, 3) == 7);
    }

    #[test]
    fn none_of_with_two_strings_doesnt_match_first_string() {
        assert!(!(none_of!("a", "b") == "a"));
    }

    #[test]
    fn none_of_with_two_strings_doesnt_match_second_string() {
        assert!(!(none_of!("a", "b") == "b"));
    }

    #[test]
    fn none_of_with_two_strings_matches_other_string() {
        assert!(none_of!("a", "b") == "x");
    }

}

#[cfg(test)]
mod all_of_tests {
    use super::*;

    #[test]
    fn all_of_with_single_int_matches_its_int() {
        assert!(all_of!(1) == 1);
    }

    #[test]
    fn all_of_with_single_int_doesnt_match_other_int() {
        assert!(!(all_of!(1) == 7));
    }

    #[test]
    fn all_of_with_two_distinct_ints_doesnt_match_first_int() {
        assert!(!(all_of!(1, 2) == 1));
    }

    #[test]
    fn all_of_with_two_distinct_ints_doesnt_match_second_int() {
        assert!(!(all_of!(1, 2) == 2));
    }

    #[test]
    fn all_of_with_two_same_ints_matches_that_int() {
        assert!(all_of!(2, 2) == 2);
    }

    #[test]
    fn all_of_with_two_same_ints_doesnt_match_other_int() {
        assert!(!(all_of!(2, 2) == 7));
    }

    #[test]
    fn all_of_with_three_distinct_ints_doesnt_match_first_int() {
        assert!(!(all_of!(1, 2, 3) == 1));
    }

    #[test]
    fn all_of_with_three_distinct_ints_doesnt_match_second_int() {
        assert!(!(all_of!(1, 2, 3) == 2));
    }

    #[test]
    fn all_of_with_three_distinct_ints_doesnt_match_third_int() {
        assert!(!(all_of!(1, 2, 3) == 3));
    }

    #[test]
    fn all_of_with_three_same_ints_matches_that_int() {
        assert!(all_of!(3, 3, 3) == 3);
    }

    #[test]
    fn all_of_with_three_same_ints_doesnt_match_other_int() {
        assert!(!(all_of!(3, 3, 3) == 7));
    }

    #[test]
    fn all_of_with_two_distinct_strings_doesnt_match_first_string() {
        assert!(!(all_of!("a", "b") == "a"));
    }

    #[test]
    fn all_of_with_two_distinct_strings_doesnt_match_second_string() {
        assert!(!(all_of!("a", "b") == "b"));
    }

    #[test]
    fn all_of_with_two_same_strings_matches_that_string() {
        assert!(all_of!("r", "r") == "r");
    }

    #[test]
    fn all_of_with_two_same_strings_doesnt_match_other_string() {
        assert!(!(all_of!("r", "r") == "a"));
    }

}

