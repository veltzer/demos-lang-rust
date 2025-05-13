# Iterator that works over two iterators

Create an iterator that runs over two other iterators (say over arrays)
and demonstrate it's use.

It's constructor receives two iterators.

Hints:
* Call the constructor trait `new`.
* To implement an iterator you need a `fn next(&mut self) -> Option<Self::Item>` method.
    where `Item` is the type that the iterator returns.
* For the entire exercise the type of the iterator can be hardcoded and be i32.

Simplification:
* Make the constructor of your ChainedIterator receive two &Vec<i32>.
