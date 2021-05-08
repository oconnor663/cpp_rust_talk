#include <fstream>
#include <optional>
#include <vector>

void file_close_at_end_of_scope() {
  std::fstream my_file("/dev/null");
}

void file_close_method() {
  std::fstream my_file("/dev/null");
  my_file.close();
}

void file_close_scope() {
  {
    std::fstream my_file("/dev/null");
  }
}

void file_close_drop() {
  using std::fstream;
  fstream my_file("/dev/null");
  my_file.~fstream(); // BAD!!!
}

void file_close_vec() {
  std::vector<std::fstream> my_files;
  my_files.push_back(std::fstream("/dev/null"));
  my_files.push_back(std::fstream("/dev/null"));
  my_files.push_back(std::fstream("/dev/null"));
  my_files[1].close();
}

void file_close_vec_erase() {
  std::vector<std::fstream> my_files;
  my_files.push_back(std::fstream("/dev/null"));
  my_files.push_back(std::fstream("/dev/null"));
  my_files.push_back(std::fstream("/dev/null"));
  my_files.erase(my_files.begin() + 1);
}

void file_close_field() {
  class foo {
  public:
    std::fstream file;
  };
  foo my_foo = {std::fstream("/dev/null")};
  my_foo.file.close();
}

void file_close_optional() {
  class foo {
  public:
    std::optional<std::fstream> file;
  };
  foo my_foo = {std::fstream("/dev/null")};
  my_foo.file.reset();
}

int main() {}
