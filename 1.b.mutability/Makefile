all: simple_aliasing two_indexes vector_invalidate

simple_aliasing: simple_aliasing.cpp
	g++ $^ -o $@ -fsanitize=address

two_indexes: two_indexes.cpp
	g++ $^ -o $@ -fsanitize=address

vector_invalidate: vector_invalidate.cpp
	g++ $^ -o $@ -fsanitize=address

clean:
	rm -f simple_aliasing two_indexes vector_invalidate
