use std::str;

#[derive(Debug)]
pub enum Message {
    Nickname(String),
    Quit,
    User(String, String, String, String),
    PrivateMessage(String, String),
    Join(Vec<String>, Vec<String>),
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
        Some("JOIN") => {
            let channels: Vec<_> = match it.next() {
                Some(chans) => chans.split(',').collect(),
                _ => [].to_vec(),
            };
            let keys: Vec<_> = match it.next() {
                Some(ks) => ks.split(',').collect(),
                _ => [].to_vec(),
            };
            if channels.is_empty() {
                Message::Err
            } else {
                Message::Join(
                    channels.iter().map(|x| x.to_string()).collect(),
                    keys.iter().map(|x| x.to_string()).collect(),
                )
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
