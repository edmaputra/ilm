#!/bin/bash

# Rust ILM Setup Script

set -e

echo "🦀 Setting up Rust ILM Application..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust found"

# Install required tools
echo "📦 Installing required tools..."
cargo install sqlx-cli --no-default-features --features postgres || echo "sqlx-cli might already be installed"
cargo install cargo-watch || echo "cargo-watch might already be installed"

# Check if Docker is running
if ! docker info &> /dev/null; then
    echo "⚠️  Docker is not running. Please start Docker to use the database."
    echo "   Alternatively, set up PostgreSQL manually."
else
    echo "🐳 Starting PostgreSQL with Docker..."
    docker-compose up -d postgres
    
    # Wait for PostgreSQL to be ready
    echo "⏳ Waiting for PostgreSQL to be ready..."
    sleep 5
fi

# Run migrations
echo "🗄️  Running database migrations..."
export DATABASE_URL="postgres://ilm_user:ilm_password@localhost:5432/ilm"
sqlx migrate run || echo "⚠️  Migration failed. Make sure PostgreSQL is running."

# Build the application
echo "🔨 Building the application..."
cargo build

echo "🎉 Setup complete!"
echo ""
echo "To run the application:"
echo "  cargo run"
echo ""
echo "To run in development mode with auto-reload:"
echo "  cargo watch -x run"
echo ""
echo "To run tests:"
echo "  cargo test"
echo ""
echo "The application will be available at: http://localhost:10001"
echo "API endpoint: http://localhost:10001/api/v1/projects?id=<uuid>"
