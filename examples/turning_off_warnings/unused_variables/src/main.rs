#[expect(unused_variables, reason = "teaching: the attribute silences the unused-variable warning on this function")]
#[cfg_attr(test, expect(clippy::missing_const_for_fn, reason = "built as a test, main is an ordinary function that could be const"))]
fn main() {
    let x=42;
}
