#include <vector>

using namespace std;

void push_int_twice(vector<int> &v, const int &n) {
  v.push_back(n);
  v.push_back(n);
}

int main() {
  vector<int> my_vector = {0};
  const int &my_int_reference = my_vector[0];
  push_int_twice(my_vector, my_int_reference);
}
