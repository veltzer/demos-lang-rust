
main.elf: main.rs libadd.a
	rustc -L. -ladd -o $@ $<
libadd.a: add.c
	gcc -c -o add.o add.c
	ar rcs libadd.a add.o

.PHONY: clean
clean:
	rm -f add.o libadd.a main.elf
