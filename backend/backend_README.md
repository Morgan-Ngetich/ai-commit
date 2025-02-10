
# AI Commit Backend

This is the backend service for the **AI Commit** project, responsible for generating commit messages using AI models. It integrates with models like **Mistral**, **LLaMA 2**, and **OpenAI** to provide AI-driven commit message suggestions based on code diffs and file changes. The backend also handles the **Command-Line Interface (CLI)** for interacting with the service.

## Features

- **Commit Message Generation**: Generates commit messages based on changed files and diffs using AI models.
- **Offline AI Models**: Uses Mistral 7B and LLaMA 2 models for offline commit message generation.
- **OpenAI API Integration**: Can also use OpenAI's GPT-4 model for cloud-based commit message generation (requires API key).
- **CLI Integration**: The backend is accessed and operated through a CLI, allowing users to generate commit messages directly from their terminal.

## Installation

To install the backend, it's recommended to install it via **Cargo**:

```bash
cargo install ai-commit
```

This is the preferred method for installing the backend as it ensures you're using the latest stable version. After running this command, the `ai-commit` CLI will be available for you to use.

Alternatively, if you prefer a manual installation, you can run the following command, which will download and run the `install.sh` script located in the **backend** folder:

```bash
curl -sSL https://github.com/Morgan-Ngetich/ai-commit/raw/main/backend/install.sh | bash
```

This command will:
- Install the necessary dependencies for the backend.
- Download and set up the required AI models (Mistral, LLaMA 2, or configure OpenAI API).

## Folder Structure

- `install.sh`: The installation script to set up the backend environment and dependencies.
- `src/ai_model.rs`: Contains logic for handling AI model interaction, generating commit messages, and running Ollama.
- `src/main.rs`: Main entry point that integrates Git commands with AI model-driven commit message generation. This is where the CLI commands are executed.

## Running the Backend

Once the backend is installed, you can generate commit messages using the CLI:

```bash
ai-commit
```

This command will analyze the staged files in your Git repository and generate a commit message based on the changes. You'll then be prompted to confirm or edit the suggested message before committing.
