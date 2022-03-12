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
