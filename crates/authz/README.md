# 🔒 beep-authz

**A Rust authorization library with SpiceDB integration for fine-grained permissions.**

[![Crates.io](https://img.shields.io/crates/v/beep-authz.svg)](https://crates.io/crates/beep-authz)
[![Documentation](https://docs.rs/beep-authz/badge.svg)](https://docs.rs/beep-authz)
[![Rust](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/beep-industries/crates/blob/main/LICENSE)

*Powerful, flexible authorization with Google Zanzibar-inspired permission checks*

[📖 Documentation](https://docs.rs/beep-authz) | [🚀 Getting Started](#quick-start) | [💡 Examples](#examples)

## ✨ Features

🔐 **SpiceDB Integration**
- Native support for SpiceDB/AuthZed
- Fine-grained permission checks
- Relationship-based access control (ReBAC)

⚡ **High Performance**
- Async/await support with Tokio
- Connection pooling
- gRPC-based communication

🎯 **Type-Safe Permissions**
- Strongly-typed permission system
- Object-based resource modeling
- Compile-time safety

🛡️ **Enterprise Ready**
- Production-tested
- Comprehensive error handling
- Token-based authentication

## 🚀 Quick Start

### Installation

Add `beep-authz` to your `Cargo.toml`:

```toml
[dependencies]
beep-authz = "0.1.0"
tokio = { version = "1.48", features = ["full"] }
```

### Basic Usage

```rust
use authz::{SpiceDbRepository, SpiceDbConfig, SpiceDbObject, Permissions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 🔧 Configure SpiceDB connection
    let config = SpiceDbConfig {
        endpoint: "localhost:50051".to_string(),
        token: Some("your-preshared-key".to_string()),
    };

    // 🔌 Connect to SpiceDB
    let authz = SpiceDbRepository::new(config).await?;

    // 🔍 Check if user can view a channel
    let result = authz.check_permissions(
        SpiceDbObject::Channel("channel-123".to_string()),
        Permissions::ViewChannels,
        SpiceDbObject::User("user-456".to_string()),
    ).await;

    if result.has_permissions() {
        println!("✅ User has permission to view channel");
    } else {
        println!("❌ Access denied");
    }

    Ok(())
}
```

## 📋 Supported Permissions

The library includes built-in permissions for common scenarios:

- **Administrator** - Full access to all resources
- **ManageServer** - Update server settings
- **ManageRoles** - Create and manage roles
- **CreateInvitation** - Generate invite links
- **ManageChannels** - Full channel management
- **ManageWebhooks** - Webhook CRUD operations
- **ViewChannels** - Read channel contents
- **SendMessages** - Post messages
- **ManageNicknames** - Update user nicknames
- **ChangeNickname** - Update own nickname
- **ManageMessages** - Moderate messages
- **AttachFiles** - Upload files

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│           SpiceDbRepository             │
│  ┌───────────────────────────────────┐  │
│  │   check_permissions()             │  │
│  │   check_permissions_raw()         │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
                    │
                    │ gRPC
                    ▼
┌─────────────────────────────────────────┐
│             SpiceDB Server              │
│  ┌───────────────────────────────────┐  │
│  │  Permission Engine                │  │
│  │  • Check relationships            │  │
│  │  • Evaluate permissions           │  │
│  │  • Return authorization result    │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## 🔧 Configuration

Configure SpiceDB connection via environment variables or command-line arguments:

```bash
# Environment variables
export SPICEDB_ENDPOINT="grpc.authzed.com:443"
export SPICEDB_TOKEN="your-preshared-key"

# Or use command-line arguments
cargo run -- --spicedb-endpoint localhost:50051 --spicedb-token your-key
```

## 🌐 SpiceDB Setup

This library works with:
- [SpiceDB](https://github.com/authzed/spicedb) - Open-source authorization system
- [AuthZed](https://authzed.com/) - Managed SpiceDB service

## 📚 Learn More

- [SpiceDB Documentation](https://authzed.com/docs)
- [Zanzibar Paper](https://research.google/pubs/pub48190/) - Google's authorization system
- [Permission System Design](https://authzed.com/blog/what-is-rebac)

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## 📄 License

Licensed under Apache License 2.0. See [LICENSE](../../LICENSE) for details.