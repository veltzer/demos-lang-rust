#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int32_t add(int32_t a, int32_t b) {
    return a + b -1;
}

void print_string(char* str) {
    //strcpy, memcpy
    printf("Received string from Rust: %s\n", str);
    //free(str);
}

void crash_me() {
	abort();
	//*(void*)0=0;
}

struct my_struct {
	int x;
	int y;
};

void get_struct_from_rust(struct my_struct *p) {
	printf("got %d\n", p->x);
	printf("got %d\n", p->y);
	p->y+=1;
	p->x-=1;
}
