#include <iostream>
#include <vector>

using namespace std;

int main() {
  vector<string_view> my_vector;
  {
    string my_string = "hello world";
    my_vector.push_back(my_string);
  }
  cout << my_vector[0];
}
