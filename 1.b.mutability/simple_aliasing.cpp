#include <iostream>

int main() {
  int my_int = 5;
  int &reference1 = my_int;
  int &reference2 = my_int;
  reference1++;
  reference2++;
}
