bitflags! {
    #[derive(Default)]
    struct ChannelMode: u8 {
        const PRIVATE = 0b00001;
        const SECRET  = 0b00010;
        const INVITE_ONLY = 0b00100;
        const MODERATED = 0b01000;
        const NO_MSGS_OUTSIDE = 0b10000;
    }
}


#[derive(Debug)]
pub struct Channel {
    users: Vec<String>,
    operators: Vec<String>,
    ban_masks: Vec<String>,
    topic: String,
    key: String,
    mode: ChannelMode,
    user_limit: u32
}

pub fn build_channel(users: Vec<String>) -> Channel {
    Channel {
        users,
        operators: Vec::new(),
        ban_masks: Vec::new(),
        topic: "topic".to_string(),
        key: "supersecretpassword".to_string(),
        mode: ChannelMode { bits: 0 },
        user_limit: 1
    }
}