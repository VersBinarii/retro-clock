file target/thumbv7m-none-eabi/release/retro_clock
target remote :3333

set print asm-demangle on
set print pretty on
set confirm off

load

break init

continue
