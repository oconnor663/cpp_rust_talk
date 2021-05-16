#include <cassert>
#include <vector>

using namespace std;

int main() {
  vector<int> a = {1, 2, 3, 4, 5, 6};
  vector<int> b = a;
  vector<int> c = move(a);
}
