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

16 tools parsed

No.	Dia.	Description
#11	1mm	2 flute shank cutter (flat end mill)
#3	4mm	3 flute 4mm polished carbide for aluminium (flat end mill)
#10	4.2mm	TiN jobber drill (drill)
#1	2mm	2mm 2 flute carbide (flat end mill)
#12	3mm	3 flute corner radius (bull nose end mill)
#7	6mm	2 flute center cutting (flat end mill)
#14	5.4mm	TiN coated jobber drill (drill)
#13	3.4mm	TiN coated jobber drill (drill)
#9	4mm	Spot drill 2 flute 4mm (spot drill)
#2	5mm	3 flute 5mm polished carbide for aluminium (flat end mill)
#16	3mm	Polished carbide (flat end mill)
#6	6mm	Ball endmill 4 flute (ball end mill)
#15	6mm	Spot drill (spot drill)
#5	8.3mm	3 flute 8.3mm chamfer mill (chamfer mill)
#4	4mm	Ball endmill 4 flute (ball end mill)
#8	3mm	3mm HSS twist drill (drill)

Tool table saved to ./tool.tbl successfully

$ ls
src/
export.tools
README.md
Cargo.toml
tool.tbl