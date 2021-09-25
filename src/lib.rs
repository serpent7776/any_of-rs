struct AnyOfPack<Tuple> {
    tuple: Tuple,
}

// HACK https://www.reddit.com/r/rust/comments/339yj3/tuple_indexing_in_a_macro/cqixd5h/
// modified to return a reference, due to compilation error
macro_rules! tuple_index {
    ($tuple:expr, $idx:tt) => {{ &$tuple.$idx }}
}

macro_rules! make_partialeq {
    ($pack: ident, $fn: ident, $map: ident, $reduce: tt, $(($t: ident, $n: tt)),+) => {
        impl<X, $($t, )+> PartialEq<X> for $pack<($($t, )+)>
        where
            $($t: std::cmp::PartialEq<X>, )+
        {
            fn $fn(&self, value: &X) -> bool {
                $reduce!($($map(&self.tuple.$n, value)),+)
            }
        }
    }
}

fn equals<T, X>(lhs: &T, rhs: &X) -> bool
where
    T: std::cmp::PartialEq<X>,
{
    lhs == rhs
}

macro_rules! or {
    ($($v: expr),+) => {
        $($v ||)+ false
    }
}

make_partialeq!(AnyOfPack, eq, equals, or, (T0, 0));
make_partialeq!(AnyOfPack, eq, equals, or, (T0, 0), (T1, 1));
make_partialeq!(AnyOfPack, eq, equals, or, (T0, 0), (T1, 1), (T2, 2));

macro_rules! any_of {
    ($($value: literal),+) => {
        AnyOfPack {
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
