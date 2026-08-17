//! Generates the OpenAPI spec and writes it to stdout as JSON.
//!
//! Run as:
//!
//! ```sh
//! cargo run --bin export-spec > openapi.json
//! ```

use server::docs::openapi_spec;

fn main() {
    let json =
        serde_json::to_string_pretty(&openapi_spec()).expect("failed to serialize OpenAPI spec");
    println!("{json}");
}
