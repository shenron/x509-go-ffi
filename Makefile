# Detect operating system in Makefile.
# Author: He Tao
# Date: 2015-05-30

OSFLAG 				:=
RUSTFLAGS     :=
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	RUSTFLAGS += -Clink-arg=-framework -Clink-args=CoreFoundation -Clink-args=-framework -Clink-args=Security
endif

buildgo:
	cd golib && go build -buildmode=c-archive -o libgophernize.a main.go
build:
	cd golib && go build -buildmode=c-archive -o libgophernize.a main.go
	RUSTFLAGS="${RUSTFLAGS}" cargo build --release --verbose
