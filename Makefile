execs = cp wc detab entab compress decompress

all: $(execs)

$(execs): %: src/%.rs
	rustc src/$@.rs
