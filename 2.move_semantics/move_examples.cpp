#include <cassert>
#include <fstream>
#include <iostream>
#include <mutex>
#include <string>
#include <vector>

int main() {
  {
    std::string s1 = "abcdefghijklmnopqrstuvwxyz";
    std::string s2 = s1;
    std::vector<std::string> v = {s2};
    std::cout << s1;
    std::cout << s2;
    std::cout << v[0];
  }

  std::cout << std::endl;

  {
    std::string s1 = "abcdefghijklmnopqrstuvwxyz";
    std::string s2 = std::move(s1);
    std::vector<std::string> v = {std::move(s2)};
    std::cout << s1;
    std::cout << s2;
    std::cout << v[0];
  }

  {
    std::fstream file1("/dev/null");
    // std::fstream file2 = file1;
  }

  {
    std::fstream file1("/dev/null");
    std::fstream file2 = std::move(file1);
  }

  {
    std::mutex mutex1;
    // std::mutex mutex2 = std::move(mutex1);
  }

  {
    std::mutex mutex;
    std::lock_guard<std::mutex> guard1(mutex);
    // std::lock_guard<std::mutex> guard2 = std::move(guard1);
  }
}
