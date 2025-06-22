# Benchmarking

## Requirements

Install [`wrk`](https://github.com/wg/wrk) to use as a benchmarking tool.

## Running

Run `wrk` pointing to the appropriate benchmarking script.

```sh
wrk -t12 -c100 -d10 -s subcribe.lua http://localhost:8080
```
