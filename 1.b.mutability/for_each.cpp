#include <algorithm>
#include <atomic>
#include <cassert>
#include <execution>
#include <iostream>
#include <mutex>
#include <numeric>
#include <vector>

using namespace std;

vector<int> vector_of_ints() {
  size_t len = 100'000;
  vector<int> v;
  for (size_t i = 0; i < len; i++) {
    v.push_back(rand() % 100);
  }
  return v;
}

void assert_sum(const char *name, const vector<int> &v,
                int sum) {
  int actual_sum = accumulate(v.begin(), v.end(), 0);
  cout << name << " expected: " << actual_sum
       << " reported: " << sum << endl;
  assert(actual_sum == sum);
}

void loop() {
  vector<int> v = vector_of_ints();
  for (auto &x : v) {
    x++;
  }
}

void serial() {
  vector<int> v = vector_of_ints();
  for_each(v.begin(), v.end(), [&](int &x) {
    x++;
  });
}

void parallel() {
  vector<int> v = vector_of_ints();
  for_each(execution::par, v.begin(), v.end(), [&](int &x) {
    x++;
  });
}

void loop_counter() {
  vector<int> v = vector_of_ints();
  int sum = 0;
  for (auto &x : v) {
    x++;
    sum += x;
  }
  assert_sum("loop", v, sum);
}

void serial_counter() {
  vector<int> v = vector_of_ints();
  int sum = 0;
  for_each(v.begin(), v.end(), [&](int &x) {
    x++;
    sum += x;
  });
  assert_sum("serial", v, sum);
}

void parallel_counter() {
  vector<int> v = vector_of_ints();
  int sum = 0;
  for_each(execution::par, v.begin(), v.end(), [&](int &x) {
    x++;
    sum += x;
  });
  assert_sum("parallel", v, sum);
}

void parallel_atomic() {
  vector<int> v = vector_of_ints();
  atomic<int> sum = 0;
  for_each(execution::par, v.begin(), v.end(), [&](int &x) {
    x += 1;
    sum.fetch_add(x, memory_order_relaxed);
  });
  assert_sum("atomic", v, sum);
}

void parallel_mutex() {
  vector<int> v = vector_of_ints();
  int sum = 0;
  mutex sum_lock;
  for_each(execution::par, v.begin(), v.end(), [&](int &x) {
    x += 1;
    lock_guard<mutex> guard(sum_lock);
    sum += x;
  });
  assert_sum("mutex", v, sum);
}

int main() {
  loop();
  serial();
  parallel();
  loop_counter();
  serial_counter();
  parallel_counter();
  parallel_atomic();
  parallel_mutex();
}
