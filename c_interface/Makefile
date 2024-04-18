
main.elf: main.rs libclib.a
	rustc -L. -lclib -o $@ $<
libclib.a: clib.c
	gcc -c -o clib.o clib.c
	ar rcs libclib.a clib.o

.PHONY: clean
clean:
	rm -f clib.o libclib.a main.elf
