# show this help
help:
    just --list

# lint code
lint:
    cargo clippy --all-targets --all-features --workspace -- -D warnings

# check formatting
fmt-check:
    cargo fmt --all --check

# format code
fmt:
    cargo fmt --all

# run all tests
test:
    cargo test --all-features --workspace

RUSTDOCFLAGS:="-D warnings"
# check docs
doc-check:
     cargo doc --no-deps --document-private-items --all-features --workspace --examples

profile:="release"
# build program
build:
    cargo build --profile {{profile}}