#include <array>
#include <iostream>

using namespace std;

int main() {
  array<char, 2> char_array = {'a', 'b'};
  char &first_element = char_array[0];
  const char &second_element = char_array[1];
  first_element = second_element;
}
