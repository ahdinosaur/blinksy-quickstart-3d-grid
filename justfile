mcu:
  cargo +esp run --release --target="xtensa-esp32-none-elf" --no-default-features --features="mcu"

desktop:
  cargo +stable run --release
