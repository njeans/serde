#[cfg_attr(target_os = "emscripten", ignore)]
#[cfg_attr(not(expandtest), ignore)]
#[rustversion::attr(not(nightly), ignore)]
#[cfg_attr(miri, ignore)]
#[allow(unused_attributes)]
#[test]
fn expandtest() {
    macrotest::expand("tests/expand/*.rs");
}
