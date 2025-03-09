# geekCaps

A tool for keyboard customization and key remapping, inspired by [cloudle/geekCaps](https://github.com/cloudle/geekCaps).

## Features (Planned)

- [ ] Remap keys on your keyboard
- [ ] Create custom key combinations
- [ ] Terminal UI for configuration
- [ ] Multi-platform support

## Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/geekCaps.git
cd geekCaps

# Build the project
cargo build --release

# Run the binary
./target/release/geekCaps
```

### Usage

List available keyboard devices:

```bash
./target/release/geekCaps list-devices
```

Create a sample configuration:

```bash
./target/release/geekCaps init -o config.toml
```

Start key remapping with a configuration:

```bash
./target/release/geekCaps start -c config.toml
```

## Configuration

Configuration is stored in TOML format. Here's an example:

```toml
name = "Developer Keyboard Layout"

# Device to apply mappings to (optional)
# device = "Device Name"

# List of key mappings
[[mappings]]
original_key = "CapsLock"
target_key = "Escape"

[[mappings]]
original_key = "Escape"
target_key = "CapsLock"
```

### Requirements

- Rust 1.70 or higher

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

## License

MIT
