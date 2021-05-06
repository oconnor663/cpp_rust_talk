all: simple_aliasing two_indexes

simple_aliasing: simple_aliasing.cpp
	g++ $^ -o $@ -fsanitize=address

two_indexes: two_indexes.cpp
	g++ $^ -o $@ -fsanitize=address

clean:
	rm -f simple_aliasing two_indexes
