#include <iostream>
int main() {
  int a = 10;
  auto b = [=](int b) { std::cout << a << "\n"; };
  b(10);
}