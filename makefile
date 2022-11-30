debug ?=
$(info debug is $(debug))

ifdef debug
  release :=
else
  release :=--release
endif

build:
	cargo build $(release)

install:
	cargo build $(release) -Z unstable-options --out-dir ./bin 

test:
	cargo test -p numbers_to_words

bench:
	cargo bench

all: test install