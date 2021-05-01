all: simple_scope string_view

simple_scope: simple_scope.cpp
	g++ $^ -o $@ -fsanitize=address

string_view: string_view.cpp
	g++ $^ -o $@ -std=c++17 -fsanitize=address

clean:
	rm -f simple_scope string_view
