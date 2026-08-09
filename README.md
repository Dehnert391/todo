To-Do List

# How to Run This Rust Program

This guide explains how to load the source code, build the Rust project, and run the produced executable. After building, copy the executable into a target folder that contains only the executable (plus `config_files`), matching the structure below.

## 1 Load the source code
1. Copy/clone the repository into a local folder.
2. Open a terminal in the project root (the folder that contains `Cargo.toml`).

## 2 Build the program
Build in release mode:

```bash
cargo build --release
```

## 3 Copy the executable
1. The executable is created at:

```text
target/release/<bin-name>
```

2. Create your run folder and copy the executable:

```bash
mkdir -p <some-folder>
cp target/release/<bin-name> <some-folder>/
```

If your `config_files/` folder is in the project root and you want it next to the executable, copy it too:

```bash
cp -r config_files <some-folder>/
```

## 4 Run
From inside `<some-folder>`:

```bash
cd <some-folder>
./<bin-name>
```

## Expected folder structure (for your screenshot)

```text
<some-folder>/
├─ config_files/
│  └─ ... (configuration files)
└─ <bin-name>
```