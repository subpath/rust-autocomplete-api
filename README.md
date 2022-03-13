# rust-autocomplete-api
fastest* autocomplete API written in rust  🦀

<sub><sub>*probably</sub></sub>

## Run it locally
1. `cargo build --release`
2. `./target/release/autocomplete-api-poc`

## Run it in Docker
1. `docker build -f Dockerfile . -t rust-autocomplete:latest `
2. `docker run -d -p 0.0.0.0:3030:3030  --name rust-autocomplete-api rust-autocomplete`

## Curl example: 
```bash
curl  --location --request POST 'localhost:3030/autocomplete/v1' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "term": "sug"
}'
```
```bash
>>["sugar", "sugarfree", "suggest", "suggested", "suggests", "suggestions", "suggestion", "sugars", "sugared", "sugary"]
```

## How does it work?
You can read about HashMap-based autocomplete implementation [here](https://github.com/subpath/rust-autocomplete-poc), where I've made terminal version of the same autocomplete as a proof of concept.


## Performance: 
I used [hey](https://github.com/rakyll/hey) to measure response time:
```bash
Summary:
  Total:	0.9591 secs
  Slowest:	0.0960 secs
  Fastest:	0.0018 secs
  Average:	0.0185 secs
  Requests/sec:	10426.6399
```
<details>
  <summary>More details</summary>

command:
```bash
hey -n 10000 -c 200 http://0.0.0.0:3030/autocomplete/v1 -H "Content-Type: application/json" -X POST -d '{"term":"ap"}'
```

output:
```bash
Summary:
  Total:	0.9591 secs
  Slowest:	0.0960 secs
  Fastest:	0.0018 secs
  Average:	0.0185 secs
  Requests/sec:	10426.6399

  Total data:	230000 bytes
  Size/request:	23 bytes

Response time histogram:
  0.002 [1]	|
  0.011 [2222]	|■■■■■■■■■■■■■■■■■■■
  0.021 [4684]	|■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.030 [2032]	|■■■■■■■■■■■■■■■■■
  0.039 [717]	|■■■■■■
  0.049 [169]	|■
  0.058 [26]	|
  0.068 [43]	|
  0.077 [74]	|■
  0.087 [25]	|
  0.096 [7]	|


Latency distribution:
  10% in 0.0085 secs
  25% in 0.0119 secs
  50% in 0.0163 secs
  75% in 0.0222 secs
  90% in 0.0306 secs
  95% in 0.0360 secs
  99% in 0.0705 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0001 secs, 0.0018 secs, 0.0960 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0000 secs
  req write:	0.0001 secs, 0.0000 secs, 0.0149 secs
  resp wait:	0.0180 secs, 0.0018 secs, 0.0763 secs
  resp read:	0.0001 secs, 0.0000 secs, 0.0126 secs

Status code distribution:
  [405]	10000 responses

```

</details>

## Also:
Take a look at the same api but [in scala + comparison with WFST](https://github.com/subpath/scala-autocomplete-api)