struct AgentConfig {
    name: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    tools_enabled: bool,
}

fn create_default_config() -> AgentConfig {
    AgentConfig {
        name: "OpenClaw Mini RS".to_string(),
        model: "llama3.2:3b".to_string(),
        temperature: 0.7,
        max_tokens: 2048,
        tools_enabled: true,
    }
}

fn print_config(config: &AgentConfig) {
    println!("Agent Name: {}", config.name);
    println!("Model: {}", config.model);
    println!("Temperature: {}", config.temperature);
    println!("Max Tokens: {}", config.max_tokens);
    println!("Tools Enabled: {}", config.tools_enabled);
}

fn change_model(config: &mut AgentConfig, new_model: &str) {
    config.model = new_model.to_string();
}

fn main() {
    let mut config = create_default_config();

    println!("Before: ");
    print_config(&config);

    change_model(&mut config, "qwen2.5:7b");

    println!();
    println!("After: ");
    print_config(&config);
}
