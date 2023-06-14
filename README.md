# Project Installation and Execution Guide

This guide provides step-by-step instructions to install and run the projects in this repository. 

## Prerequisites

The projects in this repository are written in Rust. Thus, the Rust compiler and Cargo, Rust's package manager and build system, are required to build and run these projects.

### Rust and Cargo Installation

Follow these steps to install Rust and Cargo:

1. Open a terminal or command line interface.
2. Download and install `rustup`, Rust's toolchain installer, by running the following command:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. The above command will download a script and start the installation of the `rustup` toolchain installer. You can proceed with the default installation by simply pressing "1" and then "Enter".
4. Close the terminal and reopen it.
5. Verify the installation by checking the versions of Rust and Cargo with the following commands:

    ```bash
    rustc --version
    cargo --version
    ```

## Running the Projects

This repository contains three projects: `ask1`, `ask2`, and `ask3`. Each can be built and run with Cargo. Here are the general steps for each project:

1. Clone the repository with the following command:

    ```bash
    git clone https://github.com/Mauragkas/Project.git
    ```

2. Navigate to the directory of the project. For example, for `ask1`, run:

    ```bash
    cd Project/PartA/ask1
    ```

3. Build the project with Cargo:

    ```bash
    cargo build
    ```

   This command will download and compile the project's dependencies, and then compile the project itself.

4. Run the project with Cargo:

    ```bash
    cargo run
    ```

   This command will run the compiled project.

Repeat these steps for each project (`ask1`, `ask2`, and `ask3`), replacing the project directory in step 2 as necessary.

Please note that these instructions assume you're using a Unix-like environment (Linux, MacOS, WSL on Windows). If you're using native Windows, some steps might be slightly different.

### Additional Instructions for Windows Users

If you're using native Windows (not the Windows Subsystem for Linux), the installation of `rustup` is a bit different:

1. Visit the [Rust downloads page](https://www.rust-lang.org/tools/install) and download the `rustup-init.exe` executable for Windows.
2. Run the `rustup-init.exe` and follow the onscreen instructions. Just like in Unix environments, you can usually proceed with the default installation by pressing "1" and then "Enter".
3. After the installation completes, you might need to manually add the Rust and Cargo binaries to your system PATH. The installer will provide instructions for this if necessary.
4. Restart your command line or PowerShell window.
5. Verify the installation by running `rustc --version` and `cargo --version`.

For cloning the repository and navigating to the project directories, Windows users can use the command line or PowerShell in the same way as described above. However, you might need to adjust the file paths depending on where you clone the repository and your current directory.

For example, if you clone the repository into your `Documents` folder, the command to navigate to the `ask1` directory might look like this in PowerShell:

```powershell
cd .\Documents\Project\PartA\ask1\
```

The cargo build and cargo run commands work the same way in Windows as they do in Unix environments.