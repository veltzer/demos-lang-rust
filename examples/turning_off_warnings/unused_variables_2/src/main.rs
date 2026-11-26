#[cfg_attr(test, expect(clippy::missing_const_for_fn, reason = "built as a test, main is an ordinary function that could be const"))]
fn main() {
    #[expect(clippy::no_effect_underscore_binding, reason = "teaching: the binding is there to be read, not used")]
    let _x=42;
}
