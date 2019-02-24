.PHONY: all clean

src := $(wildcard *.rs)
bin := $(patsubst %.rs,%,$(src))
env := PATH=$(shell pwd) TODO_LIST=.todo_list

all: .todo_list

.todo_list: $(bin)
	@echo '\n$$ todo_'; $(env) todo_
	@echo '\n$$ do_ Write tests'; $(env) do_ Write tests
	@echo '\n$$ do_ Setup CI'; $(env) do_ Setup CI
	@echo '\n$$ todo_'; $(env) todo_
	@echo '\n$$ did_ Write tests'; $(env) did_ Write tests
	@echo '\n$$ did_ Setup CI'; $(env) did_ Setup CI
	@echo '\n$$ todo_'; $(env) todo_

%: %.rs
	rustfmt $^
	rustc $^
	rm -f .todo_list

clean:
	rm -rf $(bin) .todo_list
