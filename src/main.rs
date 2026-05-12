
struct AgentConfig {
    name: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    tools_enabled: bool,
}

impl AgentConfig {
    fn default() -> Self {
        Self {
            name: "OpenClaw Mini RS".to_string(),
            model: "llama3.2:3b".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            tools_enabled: true,
        }
    }

    fn print(&self) {
        println!("Agent name: {}", self.name);
        println!("  - Model: {}", self.model);
        println!("  - Temperature: {}", self.temperature);
        println!("  - Max Tokens: {}", self.max_tokens);
        println!("  - Tools enabled: {}", self.tools_enabled);
    }

    fn change_model(&mut self, new_model: &str) {
        self.model = new_model.to_string();
    }

    fn summary(&self) -> String {
       format!("{} using {}, with temperature {}, using max tokens of {}, and tools are set to {}.", self.name, self.model, self.temperature, self.max_tokens, self.tools_enabled) 
    }

    fn disable_tools(&mut self) {
       self.tools_enabled = false; 
    }

}

fn main() {
    let mut config = AgentConfig::default();

    println!("Before: \n");
    println!("{}", config.summary());

    config.change_model("qwuen2.5:7b");
    config.disable_tools();

    println!("\nAfter:\n");
    println!("{}", config.summary());
}
