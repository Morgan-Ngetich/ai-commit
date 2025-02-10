
# AI Commit - VS Code Extension

## Overview

The **AI Commit VS Code Extension** integrates the **AI Commit** CLI tool into VS Code, enabling users to automatically generate Git commit messages based on changes in their project files.

### Key Features

- **Automatic Dependency Installation**: The extension ensures that both Rust and the `ai-commit` CLI are installed on the user's machine.
  - If **Rust** is not found, the extension prompts the user to install it, using a platform-specific installation process.
  - If the **AI Commit CLI** is not installed, the extension automatically installs it via the `cargo install ai-commit` command.
  
- **Commit Message Generation**: 
  - When the user runs the `aiCommit.generate` command, the extension triggers the `ai-commit` CLI to generate a commit message for the user's changes in the repository.
  - The generated commit message is then displayed in VS Code as an information message.

## Functionality Breakdown

### 1. **Dependency Check and Installation**
The extension first checks if Rust is installed. If Rust is not found:
- The user is prompted with the option to install Rust.
- Depending on the operating system (Windows, macOS, or Linux), the appropriate installation command is run automatically.

After ensuring Rust is installed, the extension checks if the **AI Commit CLI** (`ai-commit`) is installed:
- If not installed, it installs it using the `cargo install ai-commit` command.

### 2. **Generating a Commit Message**
Once all dependencies are installed:
- The user can run the `aiCommit.generate` command from the **Command Palette**.
- The extension calls the **AI Commit CLI** to generate a commit message based on the current changes in the Git repository.
- The generated message is displayed as an information message in VS Code, allowing the user to review and use it.

### 3. **Command Registration**
- The extension registers the `aiCommit.generate` command in VS Code, which can be invoked from the **Command Palette** or assigned to a keyboard shortcut.
  
## Installation and Setup

To use this extension, follow these steps:

1. **Install the Extension**: 
   - Install the extension from the VS Code marketplace or build it locally.
  
2. **Run the Command**:
   - Open the **Command Palette** in VS Code.
   - Type and select the `aiCommit.generate` command.
   
3. **Automated Installation**:
   - If necessary, the extension will prompt you to install Rust and the AI Commit CLI.
  
4. **Generate a Commit Message**:
   - The extension will automatically generate a commit message for your changes.

### Requirements
- **Rust**: The extension requires Rust to be installed in order to use the **AI Commit CLI**. The extension will prompt you to install it if not present.
- **AI Commit CLI**: The extension installs the **AI Commit CLI** automatically using `cargo install ai-commit` if not already installed.

## Contributing

If you'd like to contribute to the development of this extension, feel free to fork the repository, make improvements, and create a pull request.

---

For further details about the **AI Commit CLI**, refer to the [backend README](/ai-commit-cli/backend/backend_README.md).
