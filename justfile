install:
    cargo build --release
    cp ./target/release/libqalculate.so ~/.config/anyrun/plugins/
