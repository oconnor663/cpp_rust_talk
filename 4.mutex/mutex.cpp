#include <memory>
#include <mutex>
#include <shared_mutex>
#include <string>
#include <thread>
#include <utility>
#include <vector>

using namespace std;

void good() {
  auto my_string = make_shared<pair<mutex, string>>();
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([=] {
      lock_guard<mutex> guard(my_string->first);
      string &my_string_reference = my_string->second;
      my_string_reference += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void bad1() {
  auto my_string = make_shared<pair<mutex, string>>();
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([=] {
      // lock_guard<mutex> guard(my_string->first);
      string &my_string_reference = my_string->second;
      my_string_reference += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void bad2() {
  auto my_string = make_shared<pair<shared_mutex, string>>();
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([=] {
      shared_lock<shared_mutex> guard;
      string &my_string_reference = my_string->second;
      my_string_reference += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void bad3() {
  string my_string;
  mutex my_mutex;
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([&] {
      unique_lock<mutex>(my_mutex);
      string &my_string_reference = my_string;
      my_string_reference += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void good3() {
  string my_string;
  mutex my_mutex;
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([&] {
      unique_lock<mutex> guard(my_mutex);
      string &my_string_reference = my_string;
      my_string_reference += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

int main() {
  // good();
  // bad1();
  // bad2();
  // bad3();
  // good3();
}
