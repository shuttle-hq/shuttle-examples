## Shuttle Cron Template
This repository is an example of how you can deploy a cronjob to [Shuttle](https://www.shuttle.rs), the Rust-native dev cloud platform.

### Usage
Make sure you have `cargo-shuttle` installed! If not, you can use `cargo install cargo-shuttle` to do so. Requires Rust 1.70+ installed.

Run the following to initialise this project:
```bash
cargo shuttle init --from joshua-mo-143/shuttle-cron-template
```

Follow the prompt, then make any changes you want. Once done, you can deploy:
```bash
cargo shuttle deploy 

# use this if on a dirty Git branch
cargo shuttle deploy --allow-dirty
```
