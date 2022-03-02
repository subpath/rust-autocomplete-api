use multimap::MultiMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::str;
use warp::{http, Filter};

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Query {
    term: String,
}
fn load_suggestions() -> MultiMap<String, String> {
    let path = Path::new("resources/weighted_strings.txt");
    let mut autocomplete: MultiMap<String, String> = MultiMap::new();
    let contens: String = fs::read_to_string(&path).unwrap();
    for line in contens.lines() {
        let line_splitted: Vec<&str> = line.split('\t').collect();
        let string = line_splitted[0];
        let chars: Vec<char> = string.chars().collect();
        for idx in 1..chars.len() {
            let slice: String = chars[0..idx].iter().collect();
            autocomplete.insert(slice.to_string(), string.to_string());
        }
    }
    autocomplete
}
lazy_static! {
    static ref SUGGESTIONS: MultiMap<String, String> = load_suggestions();
}

async fn get_suggestions(query: Query) -> Result<impl warp::Reply, warp::Rejection> {
    let suggested_terms = get_suggestion(&query.term);
    let top_n_suggestions = take_slice(suggested_terms, 0, 10);
    Ok(warp::reply::with_status(
        format!("{:?}", top_n_suggestions),
        http::StatusCode::OK,
    ))
}

fn post_json() -> impl Filter<Extract = (Query,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn get_suggestion(query: &str) -> Vec<String> {
    let suggestion = SUGGESTIONS.get_vec(query);
    match suggestion {
        Some(values) => values.to_vec(),
        None => Vec::<String>::new(),
    }
}

fn take_slice(vec: Vec<String>, from_index: usize, to_index: usize) -> Vec<String> {
    if vec.get(from_index).is_none() || vec.get(to_index).is_none() {
        Vec::<String>::new()
    } else {
        vec[from_index..to_index].to_vec()
    }
}

#[tokio::main]
async fn main() {
    let suggestions_endpoint = warp::post()
        .and(warp::path("autocomplete"))
        .and(warp::path("v1"))
        .and(warp::path::end())
        .and(post_json())
        .and_then(get_suggestions);

    let routes = suggestions_endpoint;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
// curl --location --request POST 'localhost:3030/autocomplete/v1' \
// --header 'Content-Type: application/json' \
// --header 'Content-Type: text/plain' \
// --data-raw '{
//     "term": "su"
// }'
