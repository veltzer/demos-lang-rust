/*
 * This is how to throw an error in rust
 * Notes:
 * - the output goes to stderr
 * - you can see a backtrace if you set the right environment variable:
 *      RUST_BACKTRACE=1
 *      or
 *      RUST_BACKTRACE=full
 */

fn main() {
    panic!("crash and burn");
}
