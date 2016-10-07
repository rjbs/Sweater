execs = cp wc detab

all: $(execs)

$(execs): %: src/%.rs
	rustc src/$@.rs
