all: mutex

mutex: mutex.cpp
	g++ $^ -o $@ -fsanitize=thread -std=c++17 -Wall

clean:
	rm -f mutex
