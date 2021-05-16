#include <string>
#include <vector>

using namespace std;

int main() {
  string my_string = "abcdefghijklmnopqrstuvwxyz";
  vector<string> my_vector;
  my_vector.push_back(my_string);
  my_vector.push_back(move(my_string));
}
