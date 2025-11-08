# 🔐 beep-auth

**A powerful Rust authentication library with JWT validation and Keycloak integration.**

[![Crates.io](https://img.shields.io/crates/v/beep-auth.svg)](https://crates.io/crates/beep-auth)
[![Documentation](https://docs.rs/beep-auth/badge.svg)](https://docs.rs/beep-auth)
[![Rust](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/beep-industries/crates/blob/main/LICENSE)

*Secure, fast, and easy-to-use authentication for modern Rust applciations*


[📖 Documentation(https://docs.rs/beep-auth) | [🚀 Getting Started](#quick-start) | [💡 Examples](#examples) | [🤝 Contributing](#contributing)

## ✨ Features


🔑 **JWT Token Validation**
- Parse and validate JWT tokens
- Comprehensive error handling
- Manual decoding capabilities

🔐 **Keycloak Integration**
- Seamless OpenID Connect support
- JWKS key rotation handling
- Real-time token validation

🏗️ **Clean Architecture**
- Domain-driven design
- Clear separation of concerns
- Testable and maintainable

🛡️ **Security First**
- Secure token validation
- Protection against common attacks
- Industry best practices


## 🚀 Quick Start

### Installation

Add `beep-auth` to your `Cargo.toml`:

```toml
[dependencies]
beep-auth = "0.1.0"
```

### Basic Usage

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 🔧 Setup authentication repository
    let auth = KeycloakAuthRepository::new(
        "https://your-keycloak.com/realms/your-realm",
        Some("your-audience".to_string())
    );

    // 🔍 Validate a JWT token
    let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...";

    match auth.validate_token(token).await {
        Ok(claims) => {
            println!("✅ Token valid for user: {}", claims.sub);
            println!("📧 Email: {:?}", claims.email);
        }
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    Ok(())
}

## 🔧 Custom Authentication Repository

```rs
pub struct CustomAuthRepository {
    api_key: String,
    base_url: String,
}

impl AuthRepository for CustomAuthRepository {
    async fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        // 🌐 Your custom validation logic
        // Make API call to your auth service
        todo!("Implement your custom token validation")
    }

    async fn identify(&self, token: &str) -> Result<Identity, AuthError> {
        let claims = self.validate_token(token).await?;
        Ok(Identity::from(claims))
    }
}
```


## 🏗️ Architecture
The library follows clean architecture principles for maximum flexibility and testability:


```
┌───────────────────────────────────────────────┐
│                Domain                         │
│  ┌─────────────┐    ┌───────────────────────┐ │
│  │   Models    │    │     Ports             │ │
│  │             │    │                       │ │
│  │ • User      │    │ • AuthRepository      │ │
│  │ • Token     │    │ • HasAuthRepository   │ │
│  │ • Claims    │ ◄──┤                       │ │
│  │ • Identity  │    │                       │ │
│  │ • Errors    │    │                       │ │
│  └─────────────┘    └───────────────────────┘ │
└───────────────────────────────────────────────┘
                       ▲
                       │
┌──────────────────────────────────────────┐
│            Infrastructure                │
│  ┌─────────────────────────────────────┐ │
│  │    KeycloakAuthRepository           │ │
│  │                                     │ │
│  │ • JWKS key fetching                 │ │
│  │ • Token validation                  │ │
│  │ • Claims extraction                 │ │
│  │ • Identity creation                 │ │
│  └─────────────────────────────────────┘ │
└──────────────────────────────────────────┘
```


## 🧪 Testing

Run the test suite:

```
cargo test
```