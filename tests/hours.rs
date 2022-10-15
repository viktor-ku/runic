mod utils;

mod hours {
    use super::*;

    test! {
        name: letter_h,
        now: time! {},
        total: duration!{99:00-00},
        variants: {
            long => "99 h",
            short => "99h",
        }
    }

    test! {
        name: word_hr,
        now: time! {},
        total: duration!{12:00-00},
        variants: {
            long => "12 hr",
            short => "12hr",
        }
    }

    test! {
        name: word_hrs,
        now: time! {},
        total: duration!{13:00-00},
        variants: {
            long => "13 hrs",
            short => "13hrs",
        }
    }

    test! {
        name: word_hour,
        now: time! {},
        total: duration!{1:00-00},
        variants: {
            long => "1 hour",
            short => "1hour",
        }
    }

    test! {
        name: word_hours,
        now: time! {},
        total: duration!{69:00-00},
        variants: {
            long => "69 hours",
            short => "69hours",
        }
    }

    test! {
        name: with_text,
        now: time! {},
        total: duration!{02:00-00},
        variants: {
            before => "session is 2 hours",
            after => "2 hours is how long the session",
            both_sides => "session is 2 hours long",
        }
    }

    test! {
        name: with_spaces,
        now: time! {},
        total: duration!{02:00-00},
        variants: {
            before => "   2 hours",
            after => "2 hours   ",
            both_sides => "    2 hours    ",
            between => "2     hours",
        }
    }

    test! {
        name: with_new_lines,
        now: time! {},
        total: duration!{02:00},
        variants: {
            before => " \n\n  2 hours",
            after => "2 hours  \n\n ",
            both_sides => "  \n\n  2 hours  \n\n  ",
            between => "2  \n\n   hours",
        }
    }

    test! {
        name: floating_point,
        now: time! {},
        variants: {
            one_n_half => "1.5h" match duration!{01:30},
            two_n_quarter => "2.25h" match duration!{02:15},
        }
    }

    test! {
        name: add,
        now: time! {},
        variants: {
            two => "1h 2h" match duration!{03:00},
            three => "1h 2h 3h" match duration!{06:00},
        }
    }

    test! {
        name: sub,
        now: time! {},
        variants: {
            to_zero => "1h -1h" match duration!{::0},
            reverse => "-1h 1h" match duration!{::0},
            two => "5h -3h" match duration!{02:00},
            many => "10 hours -1h -1h -1h -2 hrs 5 hrs" match duration!{10:00},
            reverse_many => "-10h 2h 2h 2h 2h 2h 3 hrs at the end" match duration!{03:00},
            to_negative => "-1h -9h" match duration!{::0},
        }
    }
}
