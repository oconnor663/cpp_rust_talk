#include <iostream>
#include <memory>

using namespace std;

struct widget {
public:
  int x;
  widget() : x(0) {}
};

void use(const widget &w) {
  std::cout << w.x;
}

void g();

shared_ptr<widget> g_p = make_shared<widget>();

void f(widget &w) {
  g();
  use(w);
}

void g() {
  g_p = make_shared<widget>();
}

int main() {
  f(*g_p);
}
