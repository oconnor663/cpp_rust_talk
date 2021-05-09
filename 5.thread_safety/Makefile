all: for_each

for_each: for_each.cpp
	g++ $^ -o $@ -ltbb -fsanitize=thread -std=c++20

clean:
	rm -f for_each
