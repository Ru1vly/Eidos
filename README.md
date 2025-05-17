# Eidos

This Rust project introduces an AI-driven command-line interface (CLI) for Linux, Empowering users to interact with their system using natural language. It synergizes the capabilities of large language models with the efficiency and inherent safety of Rust programming language.

## Features

* **Natural Language Interface:** Engage with the terminal through intuitive, human-like language 
* **Intelligent Command Execution:** The AI interprets user request and translates them into thje appropriate shell commands.
* **Web Search Integration:** Seamlessly perform internet searches directly from the command line.

## General Architecture

The project is structured with the following key components:

* **Interaction Interface:** Serves as the main entry point for the CLI application. It manages user input and output.
* **Translation Agent:** Handles language detection, prompt optimization and translation progress.
* **Decider Bridge:** Reads optimized prompt and declares which AI Agent suitable for the given task.
* **Executioner:** Encompasses the core logic for command execution, including input sanitization and interaction with the operating system.
* **Explorer:** Provides the functionality to conduct internet searches utilizing the Llama API.


## Getting Started

### Prerequisites
