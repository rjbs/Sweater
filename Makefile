execs = cp wc

all: $(execs)

$(execs): %: src/%.rs
	rustc src/$@.rs
