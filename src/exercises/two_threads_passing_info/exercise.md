# Two threads

thread one: reads integers from standard input.
Whenever it gets a int it sends it to thread two.
thread two: prints the integer out.

use std::sync::mpsc as a message passing channel between them.
