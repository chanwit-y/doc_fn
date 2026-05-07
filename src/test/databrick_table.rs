use sql_builder::{SqlBuilder, quote};

use crate::api;

const TABLE: &str = "amigo_dv.brnz_sd.diamonds";

#[tokio::test]
#[ignore = "requires live Databricks credentials in .env"]
async fn get_warehouses_returns_list() {
	let warehouses = api::databricks::table::get_warehouses()
		.await
		.expect("get_warehouses should succeed");

	for w in &warehouses {
		println!("{:?} - {:?} ({:?})", w.id, w.name, w.state);
	}

	assert!(!warehouses.is_empty(), "expected at least one warehouse");
}

#[tokio::test]
#[ignore = "requires live Databricks credentials in .env"]
async fn insert_diamond_row() {
	let sql = SqlBuilder::insert_into(TABLE)
		.field("clarity")
		.field("color")
		.field("cut")
		.field("carat")
		.values(&[
			quote("SI3"),
			quote("E"),
			quote("Good"),
			"0.123".to_string(),
		])
		.sql()
		.expect("failed to build INSERT SQL");

	println!("Executing: {sql}");
	api::databricks::table::exec(&sql)
		.await
		.expect("INSERT should succeed");
}

#[tokio::test]
#[ignore = "requires live Databricks credentials in .env"]
async fn update_diamond_row() {
	let sql = SqlBuilder::update_table(TABLE)
		.set("color", quote("J"))
		.and_where_eq("clarity", quote("SI2"))
		.sql()
		.expect("failed to build UPDATE SQL");

	println!("Executing: {sql}");
	api::databricks::table::exec(&sql)
		.await
		.expect("UPDATE should succeed");
}

#[tokio::test]
#[ignore = "requires live Databricks credentials in .env"]
async fn delete_diamond_row() {
	let sql = SqlBuilder::delete_from(TABLE)
		.and_where_eq("clarity", quote("SI3"))
		.sql()
		.expect("failed to build DELETE SQL");

	println!("Executing: {sql}");
	api::databricks::table::exec(&sql)
		.await
		.expect("DELETE should succeed");
}

#[tokio::test]
#[ignore = "requires live Databricks credentials in .env"]
async fn select_diamond_rows() {
	let sql = SqlBuilder::select_from(TABLE)
		.field("*")
		.limit(100)
		.sql()
		.expect("failed to build SELECT SQL");

	let rows = api::databricks::table::exec(&sql)
		.await
		.expect("SELECT should succeed");

	let json = serde_json::to_string_pretty(&rows)
		.expect("failed to serialize rows");
	println!("{json}");

	assert!(rows.is_array(), "expected JSON array result");
}
