#include <stdio.h>
#include <stddef.h>

#define STACK_SIZE 1000

size_t stack [STACK_SIZE];
size_t* sptr = stack;

void psh(size_t x) {
  *sptr = x;
  sptr += 1;
  if (sptr >= stack + STACK_SIZE) {
    // PANIC handle
  }
}


size_t pop() {
  if (sptr <= 0) {
    // PANIC
  }
  sptr -= 1;
  return *sptr;
}

size_t top() {
  return *sptr;
}

/* define methods */

void add() {
  psh(pop() + pop());
}

void mul() {
  psh(pop() * pop());
}

void sub() {
  size_t b = pop();
  size_t a = pop();
  psh(a - b);
}

void div() {
  size_t b = pop();
  size_t a = pop();
  psh(a / b);
}

void mod() {
  size_t b = pop();
  size_t a = pop();
  psh(a % b);
}

void puti() {
  printf("%zu ", pop());
}

void putstr() {
  printf("%s ", (char*) pop());
}

void copy() {
  size_t a = pop();
  psh(a);
  psh(a);
}

void drop() {
  pop();
}

int main() {
  fun_main();
  return 0;
}

