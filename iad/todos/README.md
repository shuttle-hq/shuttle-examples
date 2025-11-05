# Todo API - Infrastructure as Data with Shuttle

A Rust-based Todo API demonstrating **Shuttle's Infrastructure as Data (IaD)** approach. This project showcases how Shuttle provisions and manages infrastructure resources through simple function annotations, eliminating the need for complex configuration files or manual cloud setup.

## What is Infrastructure as Data?

**Infrastructure as Data (IaD)** is Shuttle's approach to infrastructure provisioning where you declare your infrastructure needs directly in your code using Rust attributes. Instead of writing YAML files, Terraform scripts, or clicking through cloud consoles, you simply annotate your function parameters, and Shuttle handles the rest.

### Key Benefits

- **No Configuration Files**: Infrastructure defined in code, not YAML or JSON
- **Type-Safe**: Leverage Rust's type system for infrastructure
- **Automatic Provisioning**: Resources created on deployment
- **Zero DevOps**: No manual cloud console configuration
- **Instant Local Development**: Same code works locally and in production

## Infrastructure as Data in Action

This project demonstrates IaD with a PostgreSQL database:

```rust
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool
) -> shuttle_axum::ShuttleAxum {
    // Your database is ready to use!
}
```

**That's it!** With just one annotation (`#[shuttle_shared_db::Postgres]`), Shuttle:
1. ✅ Provisions a PostgreSQL database
2. ✅ Configures connection credentials
3. ✅ Injects a connection pool into your app
4. ✅ Manages the database lifecycle

No environment variables, no connection strings, no manual setup.
