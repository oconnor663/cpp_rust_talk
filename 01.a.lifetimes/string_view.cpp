#include <iostream>

int main() {
  std::string my_string = "hello";
  std::string_view my_string_view = my_string + " world";
  std::cout << my_string_view << std::endl;
}
