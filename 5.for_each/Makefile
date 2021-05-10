all: for_each

for_each: for_each.cpp
	g++ $^ -o $@ -g -ltbb -std=c++17

clean:
	rm -f for_each
