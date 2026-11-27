use crate::models::Message;

pub struct Chat{
	id :u32,
	messages :Vec<Message>
}

impl Chat{
	pub fn new(new_id :u32) -> Chat{
		Chat{
			id : new_id,
			messages : Vec::new()
		}
	}

	pub fn add_message(&mut self, message :Message){
		self.messages.push(message);
	}

	pub fn messages(&self) -> &[Message]{
		&self.messages
	}

	pub fn id(&self) -> u32{
		self.id
	}
}