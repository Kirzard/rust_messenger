pub struct Message{
	sender_id: u32,
	content: String
}

impl	Message{
	pub fn new(user_id :u32, content :&str) -> Message{
		let message: Message = Message{
			sender_id : user_id,
			content : content.to_string()
		};
		return message;
	}
	pub fn content(&self) -> &str{
		&self.content
	}
	pub fn sender_id(&self) -> u32{
		self.sender_id
	}
	
}