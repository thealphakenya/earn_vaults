Earn Vault

Earn Vault is a Rust-powered web application built with Actix Web that enables users to earn money through surveys, microtasks, freelancing, and other opportunities. The platform offers a secure, scalable, and AI-enhanced environment for task execution and earnings management.

🚀 Features

User Authentication – Signup, Login, and Forgot Password functionality.

Task Management – Track and verify microtasks automatically.

AI-Powered Diagnostics – Detect and resolve task execution errors.

Admin Panel – Real-time analytics and user management.

Integrated Browser – Execute tasks within a secure in-app browser.

Webhook Handling – Secure processing of external task data.

Database Backups – Automated periodic backups of SQLite data.


🛠️ Installation

1. Clone the Repository

git clone https://github.com/thealphakenya/earn_vaults.git
cd earn_vaults

2. Set Up Environment Variables

Create a .env file in the root directory:

touch .env

Add the following variables to .env:

# Database Connection
DATABASE_URL=sqlite://earn_vault.db

# Railway Deployment Token (Keep this secure)
RAILWAY_TOKEN=your-railway-token

# AI API Key (Replace with actual key)
API_KEY=your-api-key

# Deployment Environment
RAILWAY_ENVIRONMENT=production

# Backup Directory
BACKUP_DIR=./backups

# Server Port
PORT=8080

3. Build & Run the Server

cargo build --release
cargo run

Your API will be accessible at http://localhost:8080.

📡 Deploy on Railway

Click the button below to deploy this project instantly on Railway:



🔌 API Endpoints

📜 License

This project is open-source and licensed under the MIT License.
!

