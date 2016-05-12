# Fusion 360 to LinuxCNC tool table converter

This is primarily a project for learning Rust, but it should also end up as a reasonably useful utility. Its purpose is to take an exported tool library from Fusion 360's tool library (which export JSON) and convert into a `.tbl` file ready to be read by LinuxCNC.

## Usage

```bash
cargo run /path/to/export.tools /path/to/linuxcnc_tools.tbl
```

Example:

```bash
$ ls
src/
export.tools
README.md
Cargo.toml

$ cargo run ./export.tools ./tool.tbl

$ ls
src/
export.tools
README.md
Cargo.toml
tool.tbl