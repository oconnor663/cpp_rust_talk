#include <cassert>
#include <fstream>
#include <iostream>
#include <mutex>
#include <string>
#include <vector>

using namespace std;

int main() {
  {
    string s1 = "abcdefghijklmnopqrstuvwxyz";
    string s2 = s1;
    vector<string> v = {s2};
    cout << s1;
    cout << s2;
    cout << v[0];
  }

  cout << endl;

  {
    string s1 = "abcdefghijklmnopqrstuvwxyz";
    string s2 = move(s1);
    vector<string> v = {move(s2)};
    cout << s1;
    cout << s2;
    cout << v[0];
  }

  {
    fstream file1("/dev/null");
    // fstream file2 = file1;
  }

  {
    fstream file1("/dev/null");
    fstream file2 = move(file1);
  }

  {
    mutex mutex1;
    // mutex mutex2 = move(mutex1);
  }

  {
    mutex mutex;
    lock_guard<mutex> guard1(mutex);
    // lock_guard<mutex> guard2 = move(guard1);
  }
}
