use std::str;

#[derive(Debug)]
pub enum Message {
    Nickname(String),
    Quit,
    User(String, String, String, String),
    PrivateMessage(String, String),
    Err,
}

pub fn parse_message(msg: &str) -> Message {
    let newline_loc = match msg.find(|x| x == '\n' || x == '\r') {
        Some(idx) => idx,
        None => msg.len(),
    };
    let mut it = msg[..newline_loc].split(' ');
    match it.next() {
        Some("NICK") => match it.next() {
            Some(uname) => Message::Nickname(uname.to_string()),
            None => Message::Err,
        },
        Some("QUIT") => Message::Quit,
        Some("USER") => {
            let args: Vec<_> = it.collect();
            match args[..] {
                [a, b, c, d] => {
                    Message::User(a.to_string(), b.to_string(), c.to_string(), d.to_string())
                }
                _ => Message::Err,
            }
        }
        Some("PRIVMSG") => {
            let args: Vec<_> = it.collect();
            match args[..] {
                [a, b] => Message::PrivateMessage(a.to_string(), b.to_string()),
                _ => Message::Err,
            }
        }
        _ => Message::Err,
    }
}
