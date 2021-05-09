#include <algorithm>
#include <execution>
#include <iostream>
#include <mutex>
#include <vector>

using namespace std;

void serial() {
  string my_string;
  vector<const char *> my_vector{"some", "words", "here"};
  for_each(my_vector.begin(), my_vector.end(),
           [&](const char *s) { my_string += s; });
  cout << my_string << endl;
}

void parallel() {
  string my_string;
  vector<const char *> my_vector{"some", "words", "here"};
  for_each(execution::par, my_vector.begin(), my_vector.end(),
           [&](const char *s) { my_string += s; });
  cout << my_string << endl;
}

void with_mutex() {
  string my_string;
  mutex my_mutex;
  vector<const char *> my_vector{"some", "words", "here"};
  for_each(execution::par, my_vector.begin(), my_vector.end(),
           [&](const char *s) {
             lock_guard<mutex> guard(my_mutex);
             my_string += s;
           });
  cout << my_string << endl;
}

int main() {
  serial();
  parallel();
  with_mutex();
}
