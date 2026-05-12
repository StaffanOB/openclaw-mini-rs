struct AgentConfig {
    name: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    tools_enabled: bool,
}

impl AgentConfig {
    fn default() -> AgentConfig {
        AgentConfig {
            name: "OpenClaw Mini RS".to_string(),
            model: "llama3.2:3b".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            tools_enabled: true,
        }
    }

    fn print(&self) {
        println!("Agent: {}", self.name);
        println!("Agent: {}", self.model);
        println!("Agent: {}", self.temperature);
        println!("Agent: {}", self.max_tokens);
        println!("Agent: {}", self.tools_enabled);
    }
}

fn main() {
    let config = AgentConfig::default();

    config.print();
}
