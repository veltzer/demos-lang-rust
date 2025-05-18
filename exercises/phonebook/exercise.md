# Phone book

Write a phone book in rust.

The phone book should store it's data in a file called "phonebook.txt"
with the following format:

```txt
    <name1>:<phone1>
    <name2>:<phone2>
    <name3>:<phone3>
    ...
```

Read the book at the beginning

Show to following menu:
    1 print the phonebook
    2 search for a name
    3 remove a name
    4 add name and phone (only in RAM)
    5 write phonebook to disk.
    6 exit

Implement every option in its own function.

Hint:
* [using a hashmap in rust](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
