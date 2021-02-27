// Track attributes of a single client

pub struct Client {
    nick: String
}

impl Client {
    pub fn get_nick(&self) -> &String {
        &self.nick
    }

    pub fn update_nick(&mut self, nick: String) {
        self.nick = nick
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        println!("{} signing off", self.nick);
    }
}

pub fn build_client(nick: String) -> Client {
    Client {
        nick
    }
}
