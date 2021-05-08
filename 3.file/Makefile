all: file

file: file.cpp
	g++ $^ -o $@ -fsanitize=address -std=c++17

clean:
	rm -f file
