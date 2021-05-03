#include <iostream>

int main() {
  std::string my_string = "abcdefghijklmnopqrstuvwxy";
  std::string_view my_string_view = my_string + "z";
  std::cout << my_string_view << std::endl;
}
