mod utils;


wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod seconds {
    use super::*;

    test! {
        name: letter_s,
        now: time! {},
        total: duration!{::99},
        variants: {
            long => "99 s",
            short => "99s",
        }
    }

    test! {
        name: word_sec,
        now: time! {},
        total: duration!{::190},
        variants: {
            long => "190sec",
            short => "190sec",
        }
    }

    test! {
        name: word_secs,
        now: time! {},
        total: duration!{::11},
        variants: {
            long => "11 secs",
            short => "11secs",
        }
    }

    test! {
        name: word_second,
        now: time! {},
        total: duration!{::1},
        variants: {
            long => "1 second",
            short => "1second",
        }
    }

    test! {
        name: word_seconds,
        now: time! {},
        total: duration!{::60},
        variants: {
            long => "60 seconds",
            short => "60seconds",
        }
    }

    test! {
        name: with_text,
        now: time! {},
        total: duration!{::60},
        variants: {
            before => "one minute is 60 seconds",
            after => "60 seconds equals one minute",
            both_sides => "there it is 60 seconds equals one minute",
        }
    }

    test! {
        name: with_spaces,
        now: time! {},
        total: duration!{::60},
        variants: {
            before => "   60 seconds",
            after => "60 seconds     ",
            both_sides => "    60 seconds    ",
            between => "60     seconds",
        }
    }

    test! {
        name: with_new_lines,
        now: time! {},
        total: duration!{::60},
        variants: {
            before => " \n\n   60 seconds",
            after => "60 seconds  \n\n   ",
            both_sides => "  \n\n  60 seconds  \n\n  ",
            between => "60  \n\n   seconds",
        }
    }

    test! {
        name: floating_point_should_not_work,
        now: time! {},
        total: duration!{::1},
        variants: {
            one_n_half => "1.5s",
            one_n_nine => "1.9s",
            one_n_one => "1.1s",
        }
    }

    test! {
        name: add,
        now: time! {},
        variants: {
            two => "1s 2s" match duration!(::3),
            three => "1s 2s 3s" match duration!(::6),
        }
    }

    test! {
        name: sub,
        now: time! {},
        variants: {
            to_zero => "1s -1s" match duration!(::0),
            reverse => "-1s 1s" match duration!(::0),
            two => "5s -3s" match duration!(::2),
            many => "10 seconds -1s -1s -1s -2 secs 5 seconds" match duration!(::10),
            reverse_many => "-10s 2s 2s 2s 2s 2s 3 seconds at the end" match duration!(::3),
            to_negative => "-1s -9s" match duration!{::0},
        }
    }
}
