# RUST

<a href="https://doc.rust-lang.org/book/">The Book </a>

### Installation

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

```

### Check Rust

```
rustc --version
```

### Update & Uninstall Rust

```
rustup update
```

```
rustup self uninstall
```

# Creating a RUST program with Cargo

### check for cargo

```
cargo --version
```

### creating a project with Cargo

```
cargo new hello_world
```

### File Structure

![File System](image.png)

### Cargo.toml

![Cargo.toml](image-1.png)

### Hello World using RUST

```
cd hello_world
```

```
src/main.src
```

![alt text](image-2.png)

# To execute RUST via Terminal

### via rust compiler

```
rustc main.rs
```

### via Cargo

```
cargo run
```

### To check error with Cargo without execution

```
cargo check
```
