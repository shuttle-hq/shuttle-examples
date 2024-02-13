# Shuttle Example project benchmarks

This folder and document contains the materials that were used to test the performance (such as request throughput) of various example apps at different CPU limits.

For testing HTTP request throughput, [drill](https://github.com/fcsonline/drill) 0.8.2 was used.

## Test 1: Small static file server

This test used the [axum static file server](../axum/static-files/) to fetch 3 small web files.

### Setup

The `drill` file [axum-static-files.yml](./axum-static-files.yml) was used.

### Results

## Test 2: SaaS fullstack template with 500 users

Sign up a user with email `test@example.com`.

Add 500 customers:

```sh
for i in {1..500}; do
    curl 'http://127.0.0.1:8020/api/customers/create' \
        -H 'Content-Type: application/json' \
        -d '{"firstName":"Test","lastName":"User","email":"test@example.com","phone":"123456789","priority":1,"userEmail":"test@example.com"}'
done
```
