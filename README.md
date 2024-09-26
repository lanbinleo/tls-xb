# tls-xb

tls-xb is a cli tool that fetches scores and GPA from [Tsinglan Xiaobao](https://tsinglanstudent.schoolis.cn).

## Features

- Access weighted and unweighted GPA, even when hidden.
- View detailed subject scores for all subjects.
- View scores from previous semesters.
- Color coded scores depending on performance.
- Tabled output.

## Prerequisites

- tls-xb uses [viuer](https://github.com/atanunq/viuer) to display the captcha,
  so a terminal supporting one of the [supported graphics protocols](https://docs.rs/crate/viuer/latest)
  is needed, terminals supporting sixel be can found [here](https://www.arewesixelyet.com/).

  Recommended terminals:
  - Windows: [Windows Terminal](https://github.com/microsoft/terminal)
  - macOS: [iTerm 2](https://iterm2.com/)
  - Linux: [Kitty](https://sw.kovidgoyal.net/kitty/)

## Install

### From binaries

The [release page](https://github.com/hey2022/tls-xb/releases) contains
precompiled binaries for:

- Windows
   - [Installer](https://github.com/hey2022/tls-xb/releases/latest/download/tls-xb-x86_64-pc-windows-msvc.msi)
   - [Executable](https://github.com/hey2022/tls-xb/releases/download/v0.3.5/tls-xb-x86_64-pc-windows-msvc.zip)
- macOS
   - [Apple Silicon](https://github.com/hey2022/tls-xb/releases/latest/download/tls-xb-aarch64-apple-darwin.tar.xz)
   - [Intel](https://github.com/hey2022/tls-xb/releases/latest/download/tls-xb-x86_64-apple-darwin.tar.xz)
- [Linux](https://github.com/hey2022/tls-xb/releases/latest/download/tls-xb-x86_64-unknown-linux-gnu.tar.xz)

### From source

tls-xb is written in [Rust](https://www.rust-lang.org),
so the Rust [toolchain](https://rustup.rs) will be needed to compile it.

``` sh
cargo install tls-xb

# git version
cargo install --git https://github.com/hey2022/tls-xb.git
```

## Development

``` sh
git clone https://github.com/hey2022/tls-xb.git

# Build
cd tls-xb
cargo build

# Install
cargo install --path .
```

## Usage

In your terminal, run `tls-xb login` to login, then run `tls-xb` to run the program.
