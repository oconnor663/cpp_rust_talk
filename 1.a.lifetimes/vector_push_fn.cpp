#include <iostream>
#include <vector>

void my_push_back(std::vector<std::string_view> &v, std::string_view s) {
  v.push_back(s);
}

int main() {
  std::vector<std::string_view> my_vector;
  {
    std::string my_string = "hello world";
    my_push_back(my_vector, my_string);
  }
  std::cout << my_vector[0];
}
