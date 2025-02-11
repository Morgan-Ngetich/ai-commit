#!/bin/bash

set -e  # Stop on errors

INSTALL_MARKER="$HOME/.ai_commit_installed"

# Function to display a menu for selecting AI model
select_ai_model() {
    echo "🤖 Choose your AI model for commit message generation:"
    echo "1) Mistral 7B (Recommended) - Fast & lightweight."
    echo "2) LLaMA 2 - More powerful but requires higher system resources."
    echo "3) GPT-4 API - Uses OpenAI API (requires API key). Best for cloud-based AI."
    
    PS3="Enter the number of your choice: "
    select opt in "Mistral 7B" "LLaMA 2" "GPT-4 API"; do
        case $REPLY in
            1) MODEL_NAME="mistral"; break;;
            2) MODEL_NAME="llama2"; break;;
            3) MODEL_NAME="openai"; break;;
            *) echo "⚠️ Invalid choice, please select again.";;
        esac
    done
}

# Check if the install marker exists, indicating installation was already done
if [ -f "$INSTALL_MARKER" ]; then
    echo "✅ Installation already completed. Skipping setup."
    exit 0
fi

echo "🚀 Installing AI Commit CLI and VS Code Extension..."

# Step 1: Install Rust (if not installed)
if ! command -v cargo &> /dev/null; then
    echo "⚙️ Rust not found! Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Step 2: Clone the repository and build the Rust CLI
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "🔽 Cloning AI Commit CLI repository..."
git clone https://github.com/Morgan-Ngetich/ai-commit
cd ai-commit-cli/backend

echo "🔨 Building Rust project..."
cargo build --release

echo "🚛 Installing AI Commit CLI globally..."
mv target/release/ai-commit "$INSTALL_DIR/"
export PATH="$INSTALL_DIR:$PATH"

# Step 3: Install VS Code Extension
if command -v code &> /dev/null; then
    cd ../vscode-ai-commit  # Move to the correct directory
    echo "📦 Installing VS Code extension..."
    code --install-extension .
    cd ../backend  # Move back to the backend folder
else
    echo "⚠️ VS Code not found! Skipping extension installation."
fi

# Step 4: Allow user to pick AI model
select_ai_model

# Step 5: Install Ollama and AI model
if ! command -v ollama &> /dev/null; then
    echo "🧠 Installing Ollama for offline AI processing..."
    curl -fsSL https://ollama.com/install.sh | sh
fi

case $MODEL_NAME in
    "mistral")
        echo "🔽 Downloading Mistral 7B model..."
        ollama pull mistral
        ;;
    "llama2")
        echo "🔽 Downloading LLaMA 2 model..."
        ollama pull llama2
        ;;
    "openai")
        echo "🌐 Using OpenAI API for commit message generation. Ensure you set your API key."
        read -p "Enter your OpenAI API key: " OPENAI_KEY
        echo "OPENAI_API_KEY=$OPENAI_KEY" > ~/.ai_commit_config
        ;;
esac

# Step 6: Save AI model choice
echo "MODEL=$MODEL_NAME" >> ~/.ai_commit_config

# Create the install marker to indicate installation is complete
touch "$INSTALL_MARKER"

echo "✅ Installation complete! Run 'ai-commit' to use the AI Commit CLI."
