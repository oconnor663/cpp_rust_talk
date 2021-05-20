#include <iostream>

using namespace std;

int main() {
  string my_string =
      "abcdefghijklmnopqrstuvwxy";
  string_view my_string_view =
      my_string + "z";
  cout << my_string_view;
}
