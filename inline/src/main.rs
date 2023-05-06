use inline_c::assert_c;
extern crate inline_c;

fn main() {
    (assert_c! {
        #include <stdio.h>

        int main() {
            printf("Hello, World from C!\n");

            return 0;
        }
    }).success();
    // .stdout("Hello, World!");
}
