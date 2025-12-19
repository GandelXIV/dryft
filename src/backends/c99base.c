/*
* Copyright (C) 2025 Filip Chovanec
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
  printf("%zu", pop());
}

inline static void putstr() {
  printf("%s", (char*) pop());
}

inline static void copy() {
  size_t a = pop();
  psh(a);
  psh(a);
}

inline static void drop() {
  pop();
}

inline static void swap() {
  size_t b = pop();
  size_t a = pop();
  psh(b);
  psh(a);
}

inline static void simple_equality() {
  size_t b = pop();
  size_t a = pop();
  psh(a == b);
}

inline static void simple_non_equality() {
  size_t b = pop();
  size_t a = pop();
  psh(a != b);
}

int main() {
  fun_main();
  return 0;
}
