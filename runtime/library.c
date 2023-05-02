#include <stdio.h>

int print_int32(int32_t a) { return printf("%d\n", a); }

int print_int64(int64_t a) { return printf("%lld\n", a); }

int print_bool(uint8_t a) { return printf("%d\n", a); }

int print_float32(float a) { return printf("%f\n", a); }

int print_float64(double a) { return printf("%lf\n", a); }
