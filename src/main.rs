mod api;
mod core;
mod db;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() {
	println!(
		"doc_fn binary entry point. Run integration tests with `cargo test -- --ignored`."
	);
}
