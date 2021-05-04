all: simple_aliasing

simple_aliasing: simple_aliasing.cpp
	g++ $^ -o $@ -fsanitize=address

clean:
	rm -f simple_aliasing
