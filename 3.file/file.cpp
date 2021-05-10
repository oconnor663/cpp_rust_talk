#include <fstream>
#include <iostream>
#include <optional>
#include <vector>

using namespace std;

void file_close() {
  fstream my_file("/dev/null");
  // sizeof(fstream) == 528
  // dtor closes my_file
}

void file_close_scope() {
  {
    fstream my_file("/dev/null");
  }
}

void file_close_method() {
  fstream my_file("/dev/null");
  my_file.close();
}

void file_close_drop() {
  fstream my_file("/dev/null");
  my_file.~fstream(); // BAD!!!
}

void file_close_vec() {
  vector<fstream> my_files;
  my_files.push_back(fstream("/dev/null"));
  my_files.push_back(fstream("/dev/null"));
  my_files.push_back(fstream("/dev/null"));
  my_files[1].close();
}

void file_close_vec_erase() {
  vector<fstream> my_files;
  my_files.push_back(fstream("/dev/null"));
  my_files.push_back(fstream("/dev/null"));
  my_files.push_back(fstream("/dev/null"));
  my_files.erase(my_files.begin() + 1);
}

void file_close_field() {
  class foo {
  public:
    fstream file;
  };
  foo my_foo = {fstream("/dev/null")};
  my_foo.file.close();
}

void file_close_optional() {
  class foo {
  public:
    optional<fstream> file;
  };
  foo my_foo = {fstream("/dev/null")};
  my_foo.file.reset();
}

int main() {
  cout << sizeof(fstream) << "\n";
}
