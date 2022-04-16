#include <stdio.h>


struct A{
  int a;
  int b;
};

void f();

int print_int(int64_t a) { return printf("%lld\n", a); }
int print_float(double a) {
  struct A b = {10, 20};
  b.b = 10 + -30;
  b.b = f();
  print_int(b.a);
  return printf("%lf\n", a);
}
