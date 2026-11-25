struct User{
	id: u32,
	username: String
}

struct Message{
	sender: User,
	content: String
}

impl User{
	fn name(&self) -> &str{
		&self.username
	}

	fn find_user(id: u32) -> Option<User>{
		
	}
}