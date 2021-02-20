mod utils;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod minutes {
    use super::*;

    test! {
        name: letter_m,
        now: time! {},
        total: duration!{00:99-00},
        variants: {
            long => "99 m",
            short => "99m",
        }
    }

    test! {
        name: word_min,
        now: time! {},
        total: duration!{00:50-00},
        variants: {
            long => "50 min",
            short => "50min",
        }
    }

    test! {
        name: word_mins,
        now: time! {},
        total: duration!{00:11-00},
        variants: {
            long => "11 mins",
            short => "11mins",
        }
    }

    test! {
        name: word_minute,
        now: time! {},
        total: duration!{00:01-00},
        variants: {
            long => "1 minute",
            short => "1minute",
        }
    }

    test! {
        name: word_minutes,
        now: time! {},
        total: duration!{00:69-00},
        variants: {
            long => "69 minutes",
            short => "69minutes",
        }
    }

    test! {
        name: with_text,
        now: time! {},
        total: duration!{00:40-00},
        variants: {
            before => "session is 40 minutes",
            after => "40 minutes is how long the session",
            both_sides => "session is 40 minutes long",
        }
    }

    test! {
        name: with_spaces,
        now: time! {},
        total: duration!{00:40-00},
        variants: {
            before => "   40 m",
            after => "40 m     ",
            both_sides => "    40 m    ",
            between => "40     m",
        }
    }

    test! {
        name: new_lines,
        now: time! {},
        total: duration!{00:40-00},
        variants: {
            before => " \n\n  40 m",
            after => "40 m   \n\n  ",
            both_sides => "  \n\n  40 m    ",
            between => "40  \n\n   m",
        }
    }

    test! {
        name: floating_point,
        now: time! {},
        variants: {
            one_n_half => "1.5m" match duration!{::90},
            two_n_quarter => "2.25m" match duration!{::135},
        }
    }

    test! {
        name: add,
        now: time! {},
        variants: {
            two => "1m 2m" match duration!{00:03-00},
            three => "1m 2m 3m" match duration!{00:06-00},
        }
    }

    test! {
        name: sub,
        now: time! {},
        variants: {
            to_zero => "1m -1m" match duration!{::0},
            reverse => "-1m 1m" match duration!{::0},
            two => "5m -3m" match duration!{00:02-00},
            many => "10 minutes -1m -1m -1m -2 mins 5 min" match duration!{00:10-00},
            reverse_many => "-10m 2m 2m 2m 2m 2m 3 mins at the end" match duration!{00:03-00},
            to_negative => "-1m -9m" match duration!{::0},
        }
    }
}
