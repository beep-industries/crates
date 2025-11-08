# Beep Crates 🔊

A modular Rust library for Beep.


## Crates

### 🔐 beep-auth
A comprehensive authentication library that handles JWT token validation and user identity management.

**Features:**
- JWT token parsing and validation
- Keycloak integration via OpenID Connect
- Manual JWT decoding capabilities
- Clean domain-driven design
- Comprehensive error handling

**Key Components:**
- **Domain Models**: User, Token, Claims, Identity
- **Ports**: Authentication repository interfaces
- **Infrastructure**: Keycloak repository implementation


**Usage:**

Add the dependency to your `Cargo.toml`:

```toml
beep-auth = "0.1.0"
```

Import and use the library in your Rust project:

```rust
let auth_repo = KeycloakAuthRepository::new("issuer", Some("audience".to_string()));

let identity: Identity = auth_repo.identity("token").await?;
```
