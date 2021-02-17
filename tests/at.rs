mod utils;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod utc_0 {
    use super::*;

    mod at {
        use super::*;

        test! {
            name: optional_minutes,
            now: time! {10:00},
            total: duration! {01:00},
            variants: {
                without_mins => "at 11",
                with_mins => "at 11:00",
                am_without_mins => "at 11am",
                am_with_mins => "at 11:00am",
            }
        }

        test! {
            name: at_11_30,
            now: time! {10:00},
            total: duration! {01:30},
            variants: {
                h24 => "at 11:30",
                h12_short => "at 11:30am",
                h12_long => "at 11:30 am",
            }
        }

        test! {
            name: at_11_30_now_10_30,
            now: time! {10:30},
            total: duration! {01:00},
            variants: {
                h24 => "at 11:30",
                h12_short => "at 11:30am",
                h12_long => "at 11:30 am",
            }
        }

        test! {
            name: with_spaces,
            now: time! {10:30},
            total: duration! {01:00},
            variants: {
                before => "      at 11:30",
                after => "at 11:30      ",
                after_at => "at      11:30",
                after_am => "at 11:30       am",
            }
        }

        test! {
            name: with_new_lines,
            now: time! {10:30},
            total: duration! {01:00},
            variants: {
                before => "    \n\n  at 11:30",
                after => "at 11:30   \n\n   ",
                after_at => "at   \n\n   11:30",
                after_am => "at 11:30   \n\n    am",
            }
        }

        test! {
            name: with_seconds,
            now: time! {11:59-52},
            total: duration! {::8},
            variants: {
                h12 => "at 12",
                pm_12 => "at 12pm",
                pm_0 => "at 0pm",
            }
        }

        test! {
            name: tomorrow,
            now: time! {22:45-30},
            variants: {
                at_0 => "at 0" match duration!{01:14-30},
                at_2am => "at 2am" match duration!{03:14-30},
                at_4_30 => "at 4:30" match duration!{05:44-30},
                at_12_pm => "at 12pm" match duration!{13:14-30},
                at_10pm => "at 10 pm" match duration!{23:14-30},
                at_22_45 => "at 22:45" match duration!{23:59-30},
            }
        }
    }
}

const UTC_PLUS_2: i32 = 2 * 3600;
const UTC_MINUS_2: i32 = -UTC_PLUS_2;

test! {
    name: utc_plus_2,
    now: time! {22:45-30 UTC_PLUS_2},
    offset: UTC_PLUS_2,
    variants: {
        at_0 => "at 0" match duration!{01:14-30},
        at_2am => "at 2am" match duration!{03:14-30},
        at_4_30 => "at 4:30" match duration!{05:44-30},
        at_12_pm => "at 12pm" match duration!{13:14-30},
        at_10pm => "at 10 pm" match duration!{23:14-30},
        at_22_45 => "at 22:45" match duration!{23:59-30},
    }
}

test! {
    name: utc_minus_2,
    now: time! {22:45-30 UTC_MINUS_2},
    offset: UTC_MINUS_2,
    variants: {
        at_0 => "at 0" match duration!{01:14-30},
        at_2am => "at 2am" match duration!{03:14-30},
        at_4_30 => "at 4:30" match duration!{05:44-30},
        at_12_pm => "at 12pm" match duration!{13:14-30},
        at_10pm => "at 10 pm" match duration!{23:14-30},
        at_22_45 => "at 22:45" match duration!{23:59-30},
    }
}
