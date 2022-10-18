# mos-hardware-template

Template for projects using the mos-hardware crate.

## Getting started

The project requires [rust-mos](https://github.com/mrk-its/rust-mos) and
is setup to build for C64 by default.
A docker image of rust-mos is [available](https://hub.docker.com/r/mrkits/rust-mos) if you
do not fancy compiling LLVM.

### Docker and Visual Studio Code

The easiest way is to use the provided `.devcontainer.json` configuration for vscode:

1. Configure Visual Studio Code with the _Remote - Containers_ extension
2. Start Docker
3. Open the project (e.g. with `code .`) and when asked, re-open with Docker
4. In the vscode terminal do:
   ~~~ bash
   cargo build --target mos-mega65-none # build for MEGA65
   ~~~

