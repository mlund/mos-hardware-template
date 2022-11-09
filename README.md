# IMPORTANT!

**Using `mos-hardware` as a dependency (as in this template) can cause
mis-compilation as detailed in this
[rust-mos issue](https://github.com/mrk-its/rust-mos/issues/16).
Until fixed, simply work in the `examples/` directory of [`mos-hardware`](https://github.com/mlund/mos-hardware).**


# mos-hardware-template

Template for projects using the mos-hardware crate.

## Getting started

The project requires [rust-mos](https://github.com/mrk-its/rust-mos) and
is setup to build for C64 by default.
A docker image of rust-mos is [available](https://hub.docker.com/r/mrkits/rust-mos) if you
do not fancy compiling LLVM.
See also [llvm-mos wiki](https://llvm-mos.org/wiki/Rust).

### Docker and Visual Studio Code

The easiest way is to use the provided `.devcontainer.json` configuration for vscode:

1. Configure Visual Studio Code with the _Remote - Containers_ extension
2. Start Docker
3. Open the project (e.g. with `code .`) and when asked, re-open in Docker container
4. In the vscode terminal do something like:
   ~~~ bash
   cargo build --release # default c64 target
   cargo build --target mos-mega65-none # build for MEGA65
   ~~~

