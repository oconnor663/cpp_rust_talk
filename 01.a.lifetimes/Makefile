all: simple_scope

simple_scope: simple_scope.cpp
	g++ $^ -o $@ -fsanitize=address

clean:
	rm -f simple_scope
