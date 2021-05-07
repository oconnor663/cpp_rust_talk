#include <cassert>
#include <vector>

int main() {
  std::vector<int> a = {1, 2, 3};
  std::vector<int> b = a;
  std::vector<int> c = std::move(b);
}
