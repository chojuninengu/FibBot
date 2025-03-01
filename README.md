# FibBot

FibBot is a GitHub bot that scans pull requests for numbers, calculates their Fibonacci numbers, and posts a comment with the results. It is built in Rust and uses the GitHub API to interact with pull requests.

## Features

- **Fibonacci Calculation**: Computes Fibonacci numbers for numbers found in pull request content.
- **GitHub Integration**: Posts comments on pull requests with the Fibonacci results.
- **Customizable Threshold**: Allows setting a maximum threshold for Fibonacci calculations.
- **Docker Support**: Can be run in a Docker container for easy deployment.

## Prerequisites

Before using FibBot, ensure you have the following installed:

- **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
- **Docker**: Install Docker from [docker.com](https://www.docker.com/).
- **GitHub Token**: Generate a personal access token with `repo` scope from GitHub.

## Setup

### 1. Clone the Repository

```bash
git clone https://github.com/chojuninengu/FibBot.git
cd FibBot
