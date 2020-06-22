# Rust example for LPC845-BRK

This is a small example program, written in the Rust programming language, for the LCP845-BRK. It lights the blue LED, if the user button is pressed. It is based on [LPC8xx HAL](https://github.com/lpc-rs/lpc8xx-hal).

You can download and run it using [*cargo-flash*](https://crates.io/crates/cargo-flash):
```
cargo flash --chip=LPC845M301JBD48
```
