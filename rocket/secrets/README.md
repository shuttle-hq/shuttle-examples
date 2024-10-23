## Using Shuttle Secrets with Rocket

This example shows how to use secrets using [rocket](https://github.com/rwf2/rocket) and Shuttle.

The secrets resource requires a `Secrets.toml` file to be present in your crate. Each like in this file
should be a key-value pair that you can access using `SecretStore::get(&self, key)`.

## How to use

Rename `Secrets.toml.example` to `Secrets.toml` to use this example.

Once done, run the example and visit <http://localhost:8000>.
