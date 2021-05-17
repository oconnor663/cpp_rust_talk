all: simple_vector push_string move_examples

simple_vector: simple_vector.cpp
	g++ $^ -o $@ -fsanitize=address

push_string: push_string.cpp
	g++ $^ -o $@ -fsanitize=address

move_examples: move_examples.cpp
	g++ $^ -o $@ -fsanitize=address,undefined -std=c++17

clean:
	rm -f simple_vector push_string move_examples
