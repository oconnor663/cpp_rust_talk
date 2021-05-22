#include <cassert>
#include <fstream>
#include <iostream>
#include <mutex>
#include <string>
#include <string_view>
#include <vector>

using namespace std;

void f(string &s1) {
  string s2 = move(s1);
}

void g() {
  string s1 = "abcdefghijklmnopqrstuvwxyz";
  f(s1);
  cout << s1;
}

int main() {
  {
    string s1 = "abcdefghijklmnopqrstuvwxyz";
    string s2 = move(s1);
    vector<string> v = {move(s2)};
  }

  cout << endl << "---------------------------" << endl;

  {
    string s1 = "abcdefghijklmnopqrstuvwxyz";
    string s2 = s1;
    vector<string> v = {s2};
  }

  cout << endl << "---------------------------" << endl;

  {
    string s1 = "abcdefghijklmnopqrstuvwxyz";
    string s2 = move(s1);
    cout << s1;
  }

  cout << endl << "----- moved from -----" << endl;

  {
    string s1 = "abcde";
    string_view my_view = s1;
    string s2 = move(s1);
    cout << my_view;
  }

  cout << endl << "----- g() -----" << endl;

  g();

  cout << endl << "---------------------------" << endl;

  {
    string s1 = "abcdefghijklmnopqrstuvwxyz";
    string s2 = move(s1);
    vector<string> v = {move(s2)};
    string s3 = move(v[0]);
    cout << s3;
  }

  cout << endl << "---------------------------" << endl;

  {
    string s1("abc");
    string_view view(s1);
    string s2 = move(s1);
    cout << view;
    s1 = "abcdefghijklmnopqrstuvwxyz";
    cout << view;
  }

  cout << endl << "---------------------------" << endl;

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
    mutex mutex1;
    lock_guard<mutex> guard1(mutex1);
    // lock_guard<mutex> guard2 = move(guard1);
  }
}
