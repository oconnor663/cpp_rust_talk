all: for_each

for_each: for_each.cpp
	g++ $^ -o $@ -g -ltbb -std=c++20

clean:
	rm -f for_each
