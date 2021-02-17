mod utils;


wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod add {
    use super::*;

    test! {
        name: two,
        now: time! {},
        variants: {
            s_m => "1s 1m" match duration!{::61},
            m_h => "1m 1h" match duration!{01:01},
            s_h => "1s 1h" match duration!{01:00-01},
        }
    }

    test! {
        name: overflow,
        now: time! {},
        variants: {
            s_m => "180s 3m" match duration!{00:06},
            m_h => "300m 5h" match duration!{10:00},
            s_h => "3600 seconds 1 hour" match duration!{02:00},
        }
    }

    test! {
        name: mixed,
        now: time! {},
        variants: {
            t_1h_1m_1s => "1s 1m 1h" match duration!{01:01-01},
            t_13h_30m_15s => "3600 seconds 90 minutes 11 hours 15 sec" match duration!{13:30-15},
            t_11h_11m_47s => "10.5h 1800 secs 77.7s 10.5 mins" match duration!{11:11-47},
        }
    }
}
