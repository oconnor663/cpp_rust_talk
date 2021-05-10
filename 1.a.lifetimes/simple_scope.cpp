#include <iostream>

int main() {
  int *my_int_ptr;
  {
    int my_int = 5;
    my_int_ptr = &my_int;
  }
  std::cout << *my_int_ptr;
}
