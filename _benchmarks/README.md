# Shuttle Example project benchmarks

This folder and document contains the materials that were used to test the performance (such as request throughput) of various example apps at different CPU limits.

## General Info

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

The value for `N` is 100000 for 100% CPU (1 whole core), so for a test with a 0.05 vCPU limit `N` was set to 5000, and so on.

The tests were then run with:

```bash
drill --quiet --stats --benchmark [FILE]
```

## Test 1: Small static file server

This test used the [axum static file server](../axum/static-files/) to fetch 3 small web files.

Test file: [axum-static-files.yml](./axum-static-files.yml)

### Results

| vCPU limit | req/s    | req/s / vCPU |
|------------|----------|--------------|
| 0.05 |  274 | 5480 |
| 0.1  |  550 | 5500 |
| 0.25 | 1068 | 4272 |
| 0.5  | 1117 | 2234 |

---

**vCPU Limit:** 0.05

```text
Time taken for tests      546.5 seconds
Total requests            150000
Successful requests       150000
Failed requests           0
Requests per second       274.45 [#/sec]
Median time per request   45ms
Average time per request  58ms
Sample standard deviation 67ms
99.0'th percentile        299ms
99.5'th percentile        301ms
99.9'th percentile        498ms
```

**vCPU Limit:** 0.1

```text
Time taken for tests      272.8 seconds
Total requests            150000
Successful requests       150000
Failed requests           0
Requests per second       549.92 [#/sec]
Median time per request   4ms
Average time per request  29ms
Sample standard deviation 37ms
99.0'th percentile        99ms
99.5'th percentile        99ms
99.9'th percentile        105ms
```

**vCPU Limit:** 0.25

```text
Time taken for tests      140.4 seconds
Total requests            150000
Successful requests       145391
Failed requests           4609
Requests per second       1068.36 [#/sec]
Median time per request   5ms
Average time per request  14ms
Sample standard deviation 18ms
99.0'th percentile        52ms
99.5'th percentile        54ms
99.9'th percentile        61ms
```

**vCPU Limit:** 0.5

```text
Time taken for tests      134.2 seconds
Total requests            150000
Successful requests       138972
Failed requests           11028
Requests per second       1117.46 [#/sec]
Median time per request   4ms
Average time per request  12ms
Sample standard deviation 18ms
99.0'th percentile        51ms
99.5'th percentile        53ms
99.9'th percentile        57ms
```

## Test 2: SaaS fullstack template with 500 users

This test used the [SaaS fullstack template](../fullstack-templates/saas/) with a Shuttle shared Postgres database.

Test file: [saas-template.yml](./saas-template.yml)

To prepare for the test, the authentication middleware was disabled,
a user with the email `test@example.com` was signed up,
and 500 customers were inserted with this script:

```sh
for i in {1..500}; do
    curl 'http://[URL]/api/customers/create' \
        -H 'Content-Type: application/json' \
        -d '{"firstName":"Test","lastName":"User","email":"test@example.com","phone":"123456789","priority":1,"userEmail":"test@example.com"}'
done
```

The test then ran against the `/api/customers` endpoint for getting those 500 customers.

### Results

**vCPU Limit:** 0.05

```text
```
