default: run

run: compile_module
    cargo run

compile_module:
    cd string_utils && cargo build --release
    cp string_utils/target/release/libstring_utils.dylib lua/string_utils.so
