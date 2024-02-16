# Shuttle example project benchmarks

This folder and document contains the materials that were used to test the performance (such as request throughput) of various example apps at different CPU limits.

## Overview

The tests were chosen to represent 3 levels of CPU intensity:

- Light: A file server with very small responses
- Medium: A fullstack app with medium large JSON responses from a database call
- Heavy: An image processing server

## General info

- Date: 2024-02-15
- Server & Version: Shuttle production server with deployer v0.39.0
- Client to server ping latency: 1 ms (same AWS region as Shuttle server)

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

<details>
<summary>Full results</summary>

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

</details>

## Test 2: SaaS fullstack template database call with 500 users

This test used the [SaaS fullstack template](../fullstack-templates/saas/) with a Shuttle shared Postgres database.

Test file: [saas-template.yml](./saas-template.yml)

To prepare for the test, the session validation middleware was disabled in the API router,
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

| vCPU limit | req/s    | req/s / vCPU |
|------------|----------|--------------|
| 0.05 |  52 | 1040 |
| 0.1  |  99 |  990 |
| 0.25 | 263 | 1052 |
| 0.5  | 533 | 1066 |
| 1    | 775 |  775 |

<details>
<summary>Full results</summary>

**vCPU Limit:** 0.05

```text
Time taken for tests      193.3 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       51.72 [#/sec]
Median time per request   301ms
Average time per request  308ms
Sample standard deviation 63ms
99.0'th percentile        401ms
99.5'th percentile        498ms
99.9'th percentile        500ms
```

**vCPU Limit:** 0.1

```text
Time taken for tests      101.1 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       98.91 [#/sec]
Median time per request   195ms
Average time per request  160ms
Sample standard deviation 51ms
99.0'th percentile        299ms
99.5'th percentile        299ms
99.9'th percentile        301ms
```

**vCPU Limit:** 0.25

```text
Time taken for tests      38.0 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       262.92 [#/sec]
Median time per request   78ms
Average time per request  57ms
Sample standard deviation 34ms
99.0'th percentile        99ms
99.5'th percentile        100ms
99.9'th percentile        104ms
```

**vCPU Limit:** 0.5

```text
Time taken for tests      18.8 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       533.31 [#/sec]
Median time per request   18ms
Average time per request  26ms
Sample standard deviation 17ms
99.0'th percentile        64ms
99.5'th percentile        66ms
99.9'th percentile        72ms
```

**vCPU Limit:** 1

```text
Time taken for tests      12.9 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       774.70 [#/sec]
Median time per request   16ms
Average time per request  17ms
Sample standard deviation 3ms
99.0'th percentile        25ms
99.5'th percentile        26ms
99.9'th percentile        35ms
```

</details>

## Test 3: Image processing

This test used the [Salvo Image Rescaler](../salvo/image-rescaler/) to resize the Shuttle logo, a more CPU intensive task.

Test file: [salvo-image-rescaler.yml](./salvo-image-rescaler.yml)

### Results

| vCPU limit | req/s    | req/s / vCPU |
|------------|----------|--------------|
| 0.25 |  4.68 | 18.72 |
| 0.5  |  9.59 | 19.18 |
| 1    | 19.39 | 19.39 |
| 2    | 38.8  | 19.4  |
| 4    | 58.18 | 14.55 |

<details>
<summary>Full results</summary>

**vCPU Limit:** 0.25

```text
Time taken for tests      427.0 seconds
Total requests            2000
Successful requests       2000
Failed requests           0
Requests per second       4.68 [#/sec]
Median time per request   807ms
Average time per request  853ms
Sample standard deviation 380ms
99.0'th percentile        1810ms
99.5'th percentile        2007ms
99.9'th percentile        2310ms
```

**vCPU Limit:** 0.5

```text
Time taken for tests      208.6 seconds
Total requests            2000
Successful requests       2000
Failed requests           0
Requests per second       9.59 [#/sec]
Median time per request   408ms
Average time per request  416ms
Sample standard deviation 172ms
99.0'th percentile        803ms
99.5'th percentile        893ms
99.9'th percentile        999ms
```

**vCPU Limit:** 1

```text
Time taken for tests      103.2 seconds
Total requests            2000
Successful requests       2000
Failed requests           0
Requests per second       19.39 [#/sec]
Median time per request   208ms
Average time per request  206ms
Sample standard deviation 86ms
99.0'th percentile        393ms
99.5'th percentile        397ms
99.9'th percentile        412ms
```

**vCPU Limit:** 2

```text
Time taken for tests      51.5 seconds
Total requests            2000
Successful requests       2000
Failed requests           0
Requests per second       38.80 [#/sec]
Median time per request   108ms
Average time per request  102ms
Sample standard deviation 42ms
99.0'th percentile        191ms
99.5'th percentile        197ms
99.9'th percentile        216ms
```

**vCPU Limit:** 4

```text
Time taken for tests      34.4 seconds
Total requests            2000
Successful requests       2000
Failed requests           0
Requests per second       58.18 [#/sec]
Median time per request   75ms
Average time per request  68ms
Sample standard deviation 27ms
99.0'th percentile        132ms
99.5'th percentile        137ms
99.9'th percentile        145ms
```
