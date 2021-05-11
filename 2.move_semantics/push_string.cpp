#include <string>
#include <vector>

int main() {
  std::string my_string = "abcdefghijklmnopqrstuvwxyz";
  std::vector<std::string> my_vector;
  my_vector.push_back(my_string);
  my_vector.push_back(std::move(my_string));
}
