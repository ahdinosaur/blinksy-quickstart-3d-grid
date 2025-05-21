mcu:
  cargo +esp run --release --target="xtensa-esp32-none-elf"

desktop:
  cargo +stable run --release
