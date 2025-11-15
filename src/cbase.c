#include <stdio.h>

#define STACK_SIZE 1000

int stack [STACK_SIZE];
int* sptr = stack;

void psh(int x) {
  *sptr = x;
  sptr += 1;
  if (sptr >= stack + STACK_SIZE) {
    // PANIC handle
  }
}

int pop() {
  if (sptr <= 0) {
    // PANIC
  }
  sptr -= 1;
  return *sptr;
}

int top() {
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
  int b = pop();
  int a = pop();
  psh(a - b);
}

void div() {
  int b = pop();
  int a = pop();
  psh(a / b);
}

void mod() {
  int b = pop();
  int a = pop();
  psh(a % b);
}

void puti() {
  printf("%d", pop());
}
