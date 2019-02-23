.PHONY: all clean

src := $(wildcard *.rs)
bin := $(patsubst %.rs,%,$(src))
env := PATH=$(shell pwd) TODO_DB=.todo_db

all: .todo_db

.todo_db: $(bin)
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
	rm -f .todo_db

clean:
	rm -rf $(bin) .todo_db
