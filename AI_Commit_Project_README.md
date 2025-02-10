
# AI Commit Project

AI Commit is a tool designed to help developers generate commit messages using AI. It integrates a Rust-powered backend and a VS Code extension, making it easy to generate AI-powered commit messages directly from within your development environment.

## Features

- **AI-Powered Commit Messages**: Automatically generates commit messages based on your Git changes.
- **Offline Support**: Uses **Mistral 7B** and **LLaMA 2** models for offline AI processing.
- **OpenAI Integration**: Uses the **OpenAI API** for commit message generation when offline models are not available.
- **VS Code Extension**: A VS Code extension to easily generate commit messages from within the editor.

## Installation

To install the project and set up both the backend and VS Code extension, run the following single command:

```bash
curl -sSL https://github.com/Morgan-Ngetich/ai-commit/raw/main/backend/install.sh | bash
```

This command does the following:
- Installs the required backend dependencies, including **Rust** and AI models.
- Sets up the VS Code extension dependencies.

### AI Models Used

AI Commit utilizes the following AI models to generate commit messages:

- **Mistral 7B**: An offline AI model used for generating commit messages. If you select the "mistral" model, the script will download and use Mistral 7B for offline generation:

    ```bash
    echo "🔽 Downloading Mistral 7B model..."
    ollama pull mistral
    ```

- **LLaMA 2**: Another offline model option. If you select the "llama2" model, it will download and use the LLaMA 2 model:

    ```bash
    echo "🔽 Downloading LLaMA 2 model..."
    ollama pull llama2
    ```

- **OpenAI**: When offline models are unavailable or you prefer using OpenAI's API, the tool will prompt you for your OpenAI API key:

    ```bash
    echo "🌐 Using OpenAI API for commit message generation. Ensure you set your API key."
    read -p "Enter your OpenAI API key: " OPENAI_KEY
    echo "OPENAI_API_KEY=$OPENAI_KEY" > ~/.ai_commit_config
    ```

## How to Clone the Project (Optional)

If you wish to clone the repository to access or contribute to the project, you can do so with the following command:

```bash
git clone https://github.com/Morgan-Ngetich/ai-commit.git
```

---

### Backend Folder (`backend/install.sh`)

The `install.sh` script is located in the `backend` folder. It automates the installation process for both the backend (Rust) and the VS Code extension. It also manages AI model downloads (Mistral 7B, LLaMA 2, and OpenAI API setup).

---

### Troubleshooting

Ensure that the following tools are installed on your system for smooth operation:
- **Git**: To clone the repository if needed.
- **Curl**: To fetch the `install.sh` script.
- **Rust**: Required to run the backend (`cargo`).
- **Node.js**: Required for the VS Code extension.
