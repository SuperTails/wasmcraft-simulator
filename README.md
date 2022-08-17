
# Wasmcraft Preview Simulator

*This tool is part of the [Wasmcraft Suite](https://github.com/SuperTails/wasmcraft2).
See the link for more details on the project.*

The Wasmcraft Preview Simulator is a debugging tool intended
to make editing programs for Wasmcraft faster and easier.

Simulating an actual datapack (or running it in a real Minecraft world) can be slow and difficult to debug,
especially in the presence of miscompilations.
This simulator runs the WebAssembly directly and presents a simplified version of the output from the program,
which makes quickly verifying changes much easier.

## Output

The simulator will print chat messages to stdout (e.g. from `print` or `mc_putc`).
It can also display placed blocks on an X-Y plane, making it useful for previewing games.

By default, blocks placed at Z=-60 will appear on-screen with one block representing one pixel.
The Z coordinate used for the plane can be configured using a command-line argument.

As an additional feature for simulating games, `print`ing a magic value (specifiable on the command line)
will signal to the simulator that a "frame" has been completed, which will immediately update the block display
(which can sometimes lag behind) and sleep for a short duration to show the completed frame.

## Usage

Use `cargo run` to run the simulator on a file, as shown below:

```bash
cargo run -- /my/compiled/program.wasm
```

Running the program without any arguments will print usage information
that details the various arguments that are available.

```bash
cargo run
```

## Limitations

This simulator...
- ... cannot check that `mc_sleep()` calls are inserted at the proper intervals.
- ... cannot guarantee that a webassembly file will compile correctly using Wasmcraft
- ... currently has no way of taking user input
- ... only supports a single X-Y display plane