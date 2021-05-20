all: simple_aliasing two_indexes vector_invalidate for_each

simple_aliasing: simple_aliasing.cpp
	g++ $^ -o $@ -fsanitize=address

two_indexes: two_indexes.cpp
	g++ $^ -o $@ -fsanitize=address

vector_invalidate: vector_invalidate.cpp
	g++ $^ -o $@ -fsanitize=address

for_each: for_each.cpp
	g++ $^ -o $@ -g -ltbb -std=c++17

clean:
	rm -f simple_aliasing two_indexes vector_invalidate for_each
