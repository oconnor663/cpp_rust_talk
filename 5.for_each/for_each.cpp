#include <algorithm>
#include <atomic>
#include <cassert>
#include <execution>
#include <iostream>
#include <mutex>
#include <numeric>
#include <vector>

using namespace std;

vector<int> vector_of_random_numbers() {
  size_t len = 100'000;
  vector<int> v;
  for (size_t i = 0; i < len; i++) {
    v.push_back(std::rand() % 100);
  }
  return v;
}

void assert_sum(const char *name, const vector<int> &v, int sum) {
  int actual_sum = accumulate(v.begin(), v.end(), 0);
  cout << name << " expected: " << actual_sum << " reported: " << sum << endl;
  assert(actual_sum == sum);
}

void serial() {
  vector<int> v = vector_of_random_numbers();
  for_each(v.begin(), v.end(), [&](int &x) { x += 1; });
}

void parallel() {
  vector<int> v = vector_of_random_numbers();
  for_each(execution::par, v.begin(), v.end(), [&](int &x) { x += 1; });
}

void with_counter() {
  vector<int> v = vector_of_random_numbers();
  int sum = 0;
  for_each(execution::par, v.begin(), v.end(), [&](int &x) {
    x += 1;
    sum += x;
  });
  assert_sum("counter", v, sum);
}

void with_mutex() {
  vector<int> v = vector_of_random_numbers();
  int sum = 0;
  mutex sum_lock;
  for_each(execution::par, v.begin(), v.end(), [&](int &x) {
    lock_guard<mutex> guard(sum_lock);
    x += 1;
    sum += x;
  });
  assert_sum("mutex", v, sum);
}

void with_atomic() {
  vector<int> v = vector_of_random_numbers();
  atomic<int> sum = 0;
  for_each(execution::par, v.begin(), v.end(), [&](int &x) {
    x += 1;
    sum.fetch_add(x, std::memory_order_relaxed);
  });
  assert_sum("atomic", v, sum);
}

int main() {
  serial();
  parallel();
  with_counter();
  with_mutex();
  with_atomic();
}
