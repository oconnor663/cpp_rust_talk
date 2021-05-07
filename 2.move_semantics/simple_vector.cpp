#include <cassert>
#include <vector>

int main() {
  std::vector<int> a = {1, 2, 3, 4, 5, 6};
  std::vector<int> b = a;
  std::vector<int> c = std::move(a);
}
