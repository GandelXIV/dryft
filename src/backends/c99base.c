#include <stdio.h>
#include <stddef.h>

#define STACK_SIZE 1000

size_t stack [STACK_SIZE];
size_t* sptr = stack;

inline static void psh(size_t x) {
  *sptr = x;
  sptr += 1;
  if (sptr >= stack + STACK_SIZE) {
    // PANIC handle
  }
}

inline static size_t pop() {
  if (sptr <= 0) {
    // PANIC
  }
  sptr -= 1;
  return *sptr;
}

inline static size_t top() {
  return *sptr;
}

/* define methods */

inline static void add() {
  psh(pop() + pop());
}

inline static void mul() {
  psh(pop() * pop());
}

inline static void sub() {
  size_t b = pop();
  size_t a = pop();
  psh(a - b);
}

inline static void div() {
  size_t b = pop();
  size_t a = pop();
  psh(a / b);
}

inline static void mod() {
  size_t b = pop();
  size_t a = pop();
  psh(a % b);
}

inline static void puti() {
  printf("%zu ", pop());
}

inline static void putstr() {
  printf("%s ", (char*) pop());
}

inline static void copy() {
  size_t a = pop();
  psh(a);
  psh(a);
}

inline static void drop() {
  pop();
}

int main() {
  fun_main();
  return 0;
}
