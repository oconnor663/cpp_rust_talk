all: simple_vector

simple_vector: simple_vector.cpp
	g++ $^ -o $@ -fsanitize=address

clean:
	rm -f simple_vector
