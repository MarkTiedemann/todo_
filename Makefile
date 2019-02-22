src := $(wildcard *.rs)
bin := $(patsubst %.rs,%,$(src))

.PHONY: test
test: check-format build
	@sh test.sh

.PHONY: build
build: $(bin)

%: %.rs
	rustc $^

.PHONY: format
format:
	rustfmt $(src)

.PHONY: check-format
check-format:
	rustfmt --check $(src)

.PHONY: clean
clean:
	rm -rf $(bin)
