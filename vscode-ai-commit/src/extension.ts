import * as vscode from 'vscode';
import { exec } from 'child_process';
import * as os from 'os';

async function checkAndInstallRust(): Promise<void> {
    return new Promise((resolve, reject) => {
        exec('command -v rustc', async (error, stdout) => {
            if (!stdout) {
                const userChoice = await vscode.window.showInformationMessage(
                    'Rust is not installed. Do you want to install it now?',
                    'Yes', 'No'
                );

                if (userChoice === 'Yes') {
                    vscode.window.showInformationMessage('Installing Rust (this may take a few minutes)...');

                    const installScript = os.platform() === 'win32'
                        ? 'powershell -Command "Start-Process -Verb RunAs -FilePath Invoke-WebRequest -Uri https://sh.rustup.rs -OutFile rustup-init.exe; .\\rustup-init.exe -y"'
                        : 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y';

                    exec(installScript, (installErr, installStdout, installStderr) => {
                        if (installErr) {
                            vscode.window.showErrorMessage('Failed to install Rust: ' + installStderr);
                            reject(new Error('Rust installation failed'));
                            return;
                        }
                        vscode.window.showInformationMessage('Rust installed successfully! You may need to restart your terminal.');
                        resolve();
                    });
                } else {
                    vscode.window.showWarningMessage('Rust installation skipped.');
                    reject(new Error('Rust installation skipped'));
                }
            } else {
                resolve();
            }
        });
    });
}

async function checkAndInstallDependencies(): Promise<void> {
    try {
        await checkAndInstallRust();

        return new Promise((resolve, reject) => {
            exec('command -v ai-commit', (err, aiCommitOut) => {
                if (!aiCommitOut) {
                    vscode.window.showInformationMessage('Installing AI Commit CLI...');

                    exec('cargo install ai-commit', (installErr, installStdout, installStderr) => {
                        if (installErr) {
                            vscode.window.showErrorMessage('Failed to install AI Commit CLI: ' + installStderr);
                            reject(new Error('AI Commit installation failed'));
                            return;
                        }
                        vscode.window.showInformationMessage('AI Commit CLI installed successfully!');
                        resolve();
                    });
                } else {
                    resolve();
                }
            });
        });
    } catch (err) {
        console.error(err);
    }
}

async function generateCommitMessage(): Promise<void> {
    try {
        await checkAndInstallDependencies();

        exec('ai-commit', (error, stdout) => {
            if (error) {
                vscode.window.showErrorMessage('Error generating commit message!');
                return;
            }
            vscode.window.showInformationMessage(`Generated Commit: ${stdout}`);
        });
    } catch (err) {
        console.error(err);
    }
}

export function activate(context: vscode.ExtensionContext): void {
    let disposable = vscode.commands.registerCommand('aiCommit.generate', generateCommitMessage);
    context.subscriptions.push(disposable);
}

export function deactivate(): void {}
