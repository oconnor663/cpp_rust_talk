#include <algorithm>
#include <execution>
#include <iostream>
#include <vector>

int main() {
  int x = 0;
  std::vector<char> v(1'000'000, 1);
  std::for_each(std::execution::par, v.begin(), v.end(),
                [&x](auto &i) { x += i; });
  std::cout << x << std::endl;
}
