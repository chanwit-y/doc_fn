use diesel::RunQueryDsl;


mod core;
mod db;
fn main() {

	use self::db::schema::users::dsl::*;

	let db = core::auth::db::Database::from_env();
	let  conn = &mut db.get_connection().unwrap();
	let res: Vec<db::model::User> = users.load(conn).expect("Error loading posts");

	println!("{:?}", res);
}
