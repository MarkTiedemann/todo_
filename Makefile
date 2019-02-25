.PHONY: all clean

src := $(wildcard *.rs)
bin := $(patsubst %.rs,%,$(src))
env := PATH=$(CURDIR) TODO_LIST=.todo_list

all: .todo_list

.todo_list: $(bin) test.sh
	sh test.sh

%: %.rs
	rustfmt $^
	rustc $^
	rm -f .todo_list

clean:
	rm -rf $(bin) .todo_list
