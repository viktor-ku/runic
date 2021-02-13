mod utils;

use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod mixed {
    use super::*;

    test! {
        name: with_text,
        now: time! {16:11},
        total: duration! {00:19},
        describe: r###"
            Originally event should have started at 6pm
            but they moved it 1 hour further
            I need -2hours to get there and around -.5h just in case
        "###
    }
}
