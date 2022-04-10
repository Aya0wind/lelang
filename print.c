#include <stdio.h>


struct A{
  int a;
  int b;
};


int print_int(int64_t a) { return printf("%lld\n", a); }
int print_float(double a) {
  struct A b = {10,20};
  print_int(b.a);
  return printf("%lf\n", a);
}
