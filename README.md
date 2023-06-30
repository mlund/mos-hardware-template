# MOS Hardware Template

Template for projects using the mos-hardware crate.

## Getting started

The project requires [rust-mos](https://github.com/mrk-its/rust-mos) and
is setup to build for C64 by default.
A docker image of rust-mos is available for x86 and arm:
([mrkits/rust-mos](https://hub.docker.com/r/mrkits/rust-mos)) or for arm64.
See also [llvm-mos wiki](https://llvm-mos.org/wiki/Rust).

### Docker and Visual Studio Code

The easiest way is to use the provided `.devcontainer.json` configuration for vscode.

1. Configure Visual Studio Code with the _Remote - Containers_ extension. From the command-line:
   ~~~ bash
   code --install-extension ms-vscode-remote.remote-containers
   ~~~
2. Install and start [Docker](https://www.docker.com/products/docker-desktop/)
3. Open the project (e.g. with `code .`) and when asked, re-open in _Dev container_
4. In the vscode terminal type:
   ~~~ bash
   cargo build --release --target mos-c64-none
   ~~~
   resulting in the binary `target/mos-c64-none/release/mos-example` (not missing `.prg` extension).
   