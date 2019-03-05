.PHONY: all test fmt clean

src := $(wildcard *_.rs)
bin := $(patsubst %_.rs,%_,$(src))

all: $(bin)

%_: %_.rs
	rustc -O -C lto $^

test:
	sh test.sh

fmt:
	rustfmt $(wildcard *.rs)

clean:
	rm -f $(bin) .todo_list
