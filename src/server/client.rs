// Track attributes of a single client
use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct UserMode: u8 {
        const INVISIBLE = 0b00001;
        const RECEIVES_NOTICES  = 0b00010;
        const RECEIVES_WALLOPS = 0b00100;
        const OPERATOR = 0b01000;
    }
}

impl fmt::Display for UserMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}

pub struct Client {
    nick: String,
    mode: UserMode,
}

impl Client {
    pub fn get_nick(&self) -> &String {
        &self.nick
    }

    // complains about unused fns
    //pub fn update_nick(&mut self, nick: String) {
    //    self.nick = nick
    //}

    pub fn get_mode(&self) -> UserMode {
        self.mode
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        println!("{} signing off", self.nick);
    }
}

pub fn build_client(nick: String) -> Client {
    Client {
        nick,
        mode: UserMode { bits: 0 },
    }
}
