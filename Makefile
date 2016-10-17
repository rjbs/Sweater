execs = cp wc detab entab compress

all: $(execs)

$(execs): %: src/%.rs
	rustc src/$@.rs
