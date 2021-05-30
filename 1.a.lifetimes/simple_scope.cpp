#include <iostream>

using namespace std;

int main() {
  const int *my_int_ptr;
  {
    int my_int = 5;
    my_int_ptr = &my_int;
  }
  cout << *my_int_ptr;
}
