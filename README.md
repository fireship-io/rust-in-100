## Rust in 100 seconds

Article:
[What is Rust, and why is it so popular?](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)

### What is Rust?

Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

> "My biggest compliment to Rust is that it's boring, and this is an amazing compliment."
>
> – Chris Dickinson, Engineer at npm, Inc

## Did you know?

- Rust developers are known as "Rustaceans"
- Firefox, Dropbox, and Cloudflare use Rust!
- Rust is installed and managed by a tool called rustup
- Rust has a 6-week rapid release process and supports a great number of platforms, so there are many builds of Rust available at any time

### Documentation

Official site: [rust-lang.org](https://www.rust-lang.org/)

Github: [github.com/rust-lang](https://github.com/rust-lang)

Code formatting: [github.com/rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)

Crates: [crates.io](https://crates.io/)

### VS Code extensions:

- [Rust Lang](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax)
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- [Prettier TOML](https://marketplace.visualstudio.com/items?itemName=bodil.prettier-toml)

### Installation

#### Linux and macOS

```sh
curl https://sh.rustup.rs -sSf | sh
```

#### Windows

_Follow the installation steps: [forge.rust-lang.org/infra/other-installation-methods](https://forge.rust-lang.org/infra/other-installation-methods.html)_

#### Add Rust to your system PATH

In the Rust development environment, all tools are installed to the `~/.cargo/bin` directory, and this is where you will find the Rust toolchain, including rustc, cargo, and rustup.

Note:

> Is customary for Rust developers to include this directory in their PATH environment variable. During installation rustup will attempt to configure the PATH. Because of differences between platforms, command shells, and bugs in rustup, the modifications to PATH may not take effect until the terminal is restarted, the user is logged out, or it may not succeed at all

#### Options

1. Restart your computer

2. Add Rust to PATH manually:

```sh
source $HOME/.cargo/env
```

3. Add to your bash/zsh profile:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

### Toolchain management with rustup

#### Update

```sh
rustup update
```

---

#### Uninstall

```sh
rustup self uninstall
```

---

#### Troubleshooting

```sh
rustc --version
# rustc x.y.z (abcabcabc yyyy-mm-dd)
# If this fails, check your $PATH configuration
```

### Create a new project with Cargo

#### What is Cargo?

Cargo is a package manager that is equivalent to npm or pub, and comes built in with Rust.

#### Get started

1. Create a new project:

```sh
cargo new project_name # Scaffold a new project
cd project_name # Open the project folder
code . # Open VS Code (or text editor of choice)
```

### Project anatomy

#### File Structure

```
project_name
├── Cargo.toml
├── .gitignore
└── src
    └── main.rs
```

#### Cargo.toml

> Equivalent to pubspec.yaml in Dart/Flutter or package.json in JS/TS

```toml
[package]
name = "project_name"
version = "0.1.0"
edition = "2018"

[dependencies]
# Dependencies (crates) go here
```

#### main.rs

> Equivalent to index.js, main.dart, etc.

```rust
// Main application
fn main() {
    // Print to console
    println!("Hello, world!");
}
```

### Build and run a Cargo project

Builds update the file structure accordingly:

```
project_name
...
├── Cargo.lock
└── target
    ├── debug
    └── release
```

#### Debug build

```sh
cargo build
```

#### Release build

```sh
cargo build --release
```

#### Run it

> Builds with optimizations for client delivery

```sh
cargo run

  Compiling rust_in_100 v0.1.0 (/Users/klutch/Desktop/FireshipRust/rust_in_100)
      Finished dev [unoptimized + debuginfo] target(s) in 4.52s
       Running `target/debug/rust_in_100`
Hello, world!
```

#### Check

```sh
cargo check # Verify that it compiles without building
```
