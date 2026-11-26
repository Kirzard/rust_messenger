pub struct User{
	id: u32,
	username: String
}

impl User{
	pub fn name(&self) -> &str{
		&self.username
	}

	pub fn id(&self) -> u32{
		self.id
	}

	pub fn find_user(id: u32, list: &[User]) -> Option<&User>{
		for user in list.iter() {
			if user.id == id{
				return Some(user)
			}
		}
		None
	}

	pub fn new(new_name: String, new_id: u32) -> User{
		let new_user = User{
			id : new_id,
			username : new_name
		};
		return new_user;
	}

	pub fn rename(&mut self, new_name: &str) -> Result<(), String>{
		let result = User::validate_name(new_name);
		match result{
			Ok(()) => {
				self.username = new_name.to_string();
				return Ok(());
			}
			Err(error) => Err(error)
		}
	}

	pub fn validate_name(name: &str) -> Result<(), String>{
		if name.chars().count() > 3{
			return Ok(());
		}
		Err(String::from("name too short"))
	}
}