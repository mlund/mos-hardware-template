# MOS Hardware Template

Template for projects using the mos-hardware crate.

## Getting started

The project requires [rust-mos](https://github.com/mrk-its/rust-mos) and
is setup to build for C64 by default.
A docker image of rust-mos is available for x86
([mrkits/rust-mos](https://hub.docker.com/r/mrkits/rust-mos)) or for arm64
([mikaellund/rust-mos](https://hub.docker.com/r/mrkits/rust-mos)) if you're not up for the painful compilation of LLVM and Rust.
See also [llvm-mos wiki](https://llvm-mos.org/wiki/Rust).

### Docker and Visual Studio Code

The easiest way is to use the provided `.devcontainer.json` configuration for vscode.
Please edit it to match your architecture, e.g. x86 or arm64, see above.

1. Configure Visual Studio Code with the _Remote - Containers_ extension. From the command-line:
   ~~~ bash
   code --install-extension ms-vscode-remote.remote-containers
   ~~~
2. Install and start [Docker](https://www.docker.com/products/docker-desktop/)
3. Open the project (e.g. with `code .`) and when asked, re-open in _Dev container_
4. In the vscode terminal type:
   ~~~ bash
   export CARGO_NET_GIT_FETCH_WITH_CLI=true # only if Docker uses qemu
   cargo build --release --target mos-c64-none
   ~~~