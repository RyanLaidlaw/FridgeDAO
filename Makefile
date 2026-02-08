
.PHONY: all clean test

all :; remove build

remove :; rm -rf ~/.cargo/registry && rm Cargo.lock

clean :; cargo clean

build :; anchor build

sync :; anchor keys sync

test :; anchor test
