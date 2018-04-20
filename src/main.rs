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
use std::process::{Command,Stdio};
use std::fs::File;
use std::io::prelude::*;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use telegram_bot::types::requests::SendMessage;


fn md_to_png(md: &str) -> String {
    let mut command = Command::new("./md_to_png.sh")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn md_to_png");


    command.stdin.as_mut()
        .expect("Child did not have stdin")
        .write_all(md.as_bytes())
        .expect("Writing stdin to child failed");

    // Wait for command to finnish running
    command.wait().expect("Command failed to run");

    String::from("/tmp/cody.png")
}

fn send_file(filename: &str, chat_id: i64) {
    Command::new("./sendPhoto.sh")
        .arg(&format!("{}", chat_id))
        .arg(filename)
        .output()
        .expect("Failed to run upload script");

}

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("Failed to read bot token");
    let api = Api::configure(token).build(core.handle()).unwrap();

    let client = reqwest::Client::new();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ref entities} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                println!("{:?}", message.chat);

                let chars = data.chars().collect::<Vec<_>>();
                let mut markdown = String::new();
                for e in entities {
                    if e.kind == MessageEntityKind::Pre {
                        let offset = e.offset as usize;
                        let length = e.length as usize;
                        let substring = chars.get(offset..offset + length);
                        if let Some(substring) = substring {
                            markdown.push_str("```");
                            markdown.push_str(&substring.iter().collect::<String>());
                            markdown.push_str("\n```\n");
                        }
                        else {
                            println!(
                                "Substring ot of range offset: {} length: {}",
                                offset,
                                length
                            );
                        }
                    }
                }

                if markdown.len() > 0 {
                    let filename = md_to_png(&markdown);
                    send_file(&filename, message.chat.id().0);
                    //api.spawn(SendMessage::new(message.chat, markdown));
                }


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
