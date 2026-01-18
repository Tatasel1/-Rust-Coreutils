# -Rust-Coreutils

# Rusty Coreutils

A collection of basic Unix-like command-line utilities implemented in **Rust**. This project was developed as part of the **Rust Workshop at Politehnica Bucharest**. It serves as a functional demonstration of file I/O, argument parsing, and system interaction using the Rust standard library and external crates.

## 🚀 Features

The application uses a modular architecture where each command is handled by its own dedicated module.

| Command | Description | Supported Flags | Source |
| :--- | :--- | :--- | :--- |
| **`cat`** | Concatenates and displays file content. | N/A | `cat.rs` |
| **`date`** | Displays the current date and time (Local or UTC). | `-u`, `--utc` | `date.rs` |
| **`echo`** | Prints text to the standard output. | `-n`, `--no-newline` | `echo.rs` |
| **`env`** | Lists all current environment variables. | N/A | `env.rs` |
| **`head`** | Outputs the first part of files (default 10 lines). | `-n`, `--number-of-lines` | `head.rs` |
| **`rm`** | Removes files or directories. | `-d` (dir), `-r` (recursive) | `rm.rs` |

---

## 🛠️ Installation & Building

### Prerequisites
* **Rust Toolchain**: You must have `cargo` and `rustc` installed.
* **Dependencies**: The project utilizes the `chrono` crate for time formatting.

### Building the Project
1. Clone this repository to your local machine.
2. Build the optimized binary:
 
   ```bash
   cargo build --release
   ```

   
📖 Usage
The application uses a sub-command structure defined in main.rs. Run the tool followed by the command name and its specific arguments:

 ```bash
cargo run -- <command> [arguments]
```

Examples
View Files: ```cargo run -- cat document.txt```

Check UTC Time: ```cargo run -- date --utc```

Custom Head: ```cargo run -- head -n 5 data.log``` (reads the first 5 lines)

Recursive Delete: ```cargo run -- rm -r target_folder```

Echo text: ```cargo run -- echo "Hello World"```

## 📂 Project Structure

``main.rs``: The entry point that uses a match statement to route arguments to the correct utility module.

``cat.rs``: Uses BufReader to efficiently read and print file lines.

``date.rs``: Interfaces with the chrono crate to format system time in Local or UTC.

``echo.rs``: Handles simple output logic and newline suppression flags.

``env.rs``: Accesses the environment via std::env::vars().

``head.rs``: Implements line-limited file reading using .take(n).

``rm.rs``: Handles filesystem deletions, including logic for empty directories and recursive removal using remove_dir_all.

## ⚠️ Important Note on Error Handling
The current implementation utilizes ``panic!`` and ``unwrap()`` for error handling in several modules, such as when a file cannot be opened or a command is unknown. Users should use the ``rm`` command with caution as it will panic on unauthorized access or missing metadata.


