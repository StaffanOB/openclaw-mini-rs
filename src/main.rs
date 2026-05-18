struct Message {
    role: String,
    content: String,
}

struct Session {
    messages: Vec<Message>,
}

impl Session {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    fn add_message(&mut self, role: &str, content: &str) {
        let message = Message {
            role: role.to_string(),
            content: content.to_string(),
        };

        self.messages.push(message);
    }

    fn print_hitstory(&self) {
        for message in &self.messages {
            println!("{}: {}:", message.role, message.content);
        }

        println!("Message Coumnt: {}", self.message_count());
    }

    fn last_message_content(&self) -> Option<&str> {
        self.messages.last().map(|message| message.content.as_str())
    }

    fn message_count(&self) -> usize {
        self.messages.len()
    }
}

fn main() {
    let mut session = Session::new();

    session.add_message("system", "You are OpenClaw MINI");
    session.add_message("user", "Hello agent.");
    session.add_message("assistant", "Hello! How can i help?");

    session.print_hitstory();

    if let Some(content) = session.last_message_content() {
        println!();
        println!("Last message was: {}", content);
    }
}
