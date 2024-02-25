use discord::api::DiscordApiInterface as DiscordApi;
use std::env;

mod discord;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let channel_id = "1211170430023368724";
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let discord = DiscordApi::new();

    let message_id = discord.send_message(&channel_id, &token, "rust lol").expect("Failed to send message");
    println!("Message sent with id: {}", message_id);
    
    let message = discord.get_message(&channel_id, &token, &message_id).expect("Failed to get message");
    println!("Message retrieved: {:?}", message);

}