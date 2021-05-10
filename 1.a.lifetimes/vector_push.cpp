#include <iostream>
#include <vector>

int main() {
  std::vector<std::string_view> my_vector;
  {
    std::string my_string = "hello world";
    my_vector.push_back(my_string);
  }
  std::cout << my_vector[0];
}
