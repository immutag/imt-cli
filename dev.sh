#!/bin/bash

start-docker() {
    docker-compose up
}

cargo-build() {
    docker exec \
    -it \
    cli-starter_coding_1 \
    cargo build
}

cargo-bin-build() {
    docker exec \
    -it \
    cli-starter_coding_1 \
    cargo build --bin cli-starter
}

build() {
    cargo-build
    cargo-bin-build
}

rust-lib-test() {
    docker exec \
    -it \
    cli-starter_coding_1 \
    cargo test $1 -- --test-threads=1 --nocapture
}

rust-bin-test() {
    docker exec \
    -it \
    cli-starter_coding_1 \
    cargo test --bin $1
}

run-test() {
    if [ "$1" = "rust-lib" ]; then
        cargo-bin-build
        rust-lib-test $2
    fi

    if [ "$1" = "rust-bin" ]; then
        cargo-bin-build
        rust-bin-test $2
    fi

    if [ "$1" = "" ]; then
        cargo-bin-build
        rust-lib-test
        rust-bin-test
    fi
}

if [ "$1" == "rust-build" ]; then
    echo "rust build"
    cargo-build
fi

if [ "$1" == "test" ]; then
    echo "test"
    run-test $2 $3
fi
