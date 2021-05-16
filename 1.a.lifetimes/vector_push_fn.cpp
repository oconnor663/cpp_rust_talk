#include <iostream>
#include <vector>

using namespace std;

void my_push_back(vector<string_view> &v, string_view s) { v.push_back(s); }

int main() {
  vector<string_view> my_vector;
  {
    string my_string = "hello world";
    my_push_back(my_vector, my_string);
  }
  cout << my_vector[0];
}
