#include <algorithm>
#include <execution>
#include <iostream>
#include <vector>

int main() {
  int x = 0;
  std::vector<int> v{0, 1, 2, 3, 4, 5};
  std::for_each(std::execution::par, v.begin(), v.end(),
                [&x](int i) { x += i; });
  std::cout << x << std::endl;
}
