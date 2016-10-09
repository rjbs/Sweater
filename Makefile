execs = cp wc detab entab

all: $(execs)

$(execs): %: src/%.rs
	rustc src/$@.rs
