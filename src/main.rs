mod conversation;

use std::io::Write;

use chatgpt::{client::ChatGPT, config::ChatGPTEngine};
use clap::Parser;
use reqwest::Proxy;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Set socks proxy
    #[arg(short, long)]
    proxy: Option<String>,
    /// An OpenAI API key
    #[arg(short = 'k', long = "key", required = true)]
    key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = {
        let mut client = match args.proxy {
            Some(proxy) => ChatGPT::new_with_proxy(args.key, Proxy::all(proxy).unwrap()).unwrap(),
            None => ChatGPT::new(args.key).unwrap(),
        };

        client.config.engine = ChatGPTEngine::Custom("gpt-4o-mini");
        client
    };

    loop {
        let mut string = String::new();
        print!(">> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut string).unwrap();
    
        // Call the send_message function asynchronously
    
        println!(
            "{}",
            client.send_message(string).await.unwrap().message().content
        );
        
    
    }

}
