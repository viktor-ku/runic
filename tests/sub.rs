mod utils;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod sub {
    use super::*;

    test! {
        name: two,
        now: time! {},
        variants: {
            s_m => "-1s 1m" match duration!{::59},
            m_h => "-1m 1h" match duration!{00:59},
            s_h => "-1s 1h" match duration!{00:59-59},
        }
    }

    test! {
        name: mixed,
        now: time! {},
        variants: {
            t_29m_30s => "-3600s 3900s -5.5m .5h" match duration!{00:29-30},
        }
    }
}
