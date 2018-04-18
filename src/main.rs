extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod gist;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use telegram_bot::types::requests::SendMessage;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("Failed to read bot token");
    let api = Api::configure(token).build(core.handle()).unwrap();

    let client = reqwest::Client::new();

    let gist_body = serde_json::to_string(
        &gist::GistCreateRequest::new_single_file(
            "test.txt".into(),
            "yoloswag".into()
        )
    ).expect("Failed to encode gist body");

    let response = client.post("https://api.github.com/gists")
        .body(gist_body)
        .send().expect("Failed to send request");

    println!("response: {:?}", response);

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ref entities} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                println!("entities: {:?}", entities);

                let mut code_blocks = vec!();
                for e in entities {
                    if e.kind == MessageEntityKind::Pre {
                        println!("Got pre {:?}", e);
                        code_blocks.push(
                            format!("Found code starting at {} with length {}", e.offset, e.length)
                        );
                    }
                }

                api.spawn(SendMessage::new(message.chat, format!("{:?}", code_blocks)));
                


                //// Answer message with "Hi".
                //api.spawn(
                //    //SendMessage::new(message.chat, "```rust\nfn main(){}\n```")
                //    SendMessage::new(message.chat, data.to_string())
                //        .parse_mode(ParseMode::Markdown)
                //);
                //api.spawn(message.text_reply(
                //    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                //));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
