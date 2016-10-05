execs = cp wc

all: $(execs)

$(execs): %: %.rs
	rustc $@.rs
