use std::collections::HashMap;

pub trait GenericController<U: User> {
	fn connect_to(url: &str) -> Self;

	fn get_user_by_name(&self, name: &str) -> Vec<U>;
	fn get_user_by_id(&self, uid: u64) -> Vec<U>;
	fn get_all_users(&self) -> Vec<U>;
}

pub trait User {
	fn id(&self) -> i32;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>;
	fn scores(&self) -> HashMap<i32, f64>; //pelicula id, score
}

pub trait Item {
	fn id(&self) -> i32;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>; //extra
}