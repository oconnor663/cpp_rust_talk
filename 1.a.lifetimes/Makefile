all: simple_scope string_view vector_push vector_push_fn

simple_scope: simple_scope.cpp
	g++ $^ -o $@ -fsanitize=address

string_view: string_view.cpp
	g++ $^ -o $@ -std=c++17 -fsanitize=address

vector_push: vector_push.cpp
	g++ $^ -o $@ -std=c++17 -fsanitize=address

vector_push_fn: vector_push_fn.cpp
	g++ $^ -o $@ -std=c++17 -fsanitize=address

clean:
	rm -f simple_scope string_view vector_push vector_push_fn
