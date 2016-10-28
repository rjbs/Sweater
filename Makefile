execs = cp wc detab entab compress decompress crypt

all: $(execs)

$(execs): %: src/%.rs
	rustc src/$@.rs
