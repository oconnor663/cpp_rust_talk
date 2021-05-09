#include <memory>
#include <mutex>
#include <shared_mutex>
#include <string>
#include <thread>
#include <utility>
#include <vector>

using namespace std;

void stack_local() {
  string my_string;
  mutex my_mutex;
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([&] {
      unique_lock<mutex> guard(my_mutex);
      my_string += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void with_shared_ptr() {
  auto my_pair = make_shared<pair<mutex, string>>();
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([=] {
      lock_guard<mutex> guard(my_pair->first);
      my_pair->second += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void oops() {
  auto my_pair = make_shared<pair<mutex, string>>();
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([=] {
      // lock_guard<mutex> guard(my_pair->first);
      my_pair->second += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void with_shared_mutex() {
  auto my_pair = make_shared<pair<shared_mutex, string>>();
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([=] {
      shared_lock<shared_mutex> guard(my_pair->first);
      my_pair->second += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

void vexing_parse() {
  string my_string;
  mutex my_mutex;
  vector<thread> thread_handles;
  for (int i = 0; i < 10; i++) {
    thread thread_handle([&] {
      unique_lock<mutex>(my_mutex);
      my_string += "some characters";
    });
    thread_handles.push_back(std::move(thread_handle));
  }
  for (auto &thread_handle : thread_handles) {
    thread_handle.join();
  }
}

int main() {
  stack_local();
  with_shared_ptr();
  oops();
  with_shared_mutex();
  vexing_parse();
}
