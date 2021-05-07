all: simple_vector push_string

simple_vector: simple_vector.cpp
	g++ $^ -o $@ -fsanitize=address

push_string: push_string.cpp
	g++ $^ -o $@ -fsanitize=address

clean:
	rm -f simple_vector push_string
