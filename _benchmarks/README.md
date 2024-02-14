# Shuttle Example project benchmarks

This folder and document contains the materials that were used to test the performance (such as request throughput) of various example apps at different CPU limits.

## General Info

Date: 2024-02-14
Server: Shuttle production server with deployer v0.39.0
Client to Server Latency: 1 ms (same AWS region as Shuttle server)

## Methodology

For testing HTTP request throughput, [drill](https://github.com/fcsonline/drill) 0.8.2 was used.

Each project was deployed,
the URL in the `drill` test file was updated with the deployment URL,
and the container CPU quota was adjusted ahead of each test with

```bash
docker container update --cpu-quota=${N} shuttle_${project}_run
```

The value for `N` is 100000 for 100% CPU (1 whole core), so for a test with a 5% CPU limit `N` was set to 5000.

The tests were then run with:

```bash
drill --quiet --stats --benchmark [FILE]
```

## Test 1: Small static file server

This test used the [axum static file server](../axum/static-files/) to fetch 3 small web files.

Test file: [axum-static-files.yml](./axum-static-files.yml)

### Results

```text

```

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
