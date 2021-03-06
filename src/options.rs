use consts::SocketOption;

use std::time::duration::Duration;


pub struct Options {
    //  Socket identity.
    pub identity_size: u8,
    //identity: [u8,..256],

    //  Socket type.
    pub type_: int,

    //  Minimum interval between attempts to reconnect
    //  Default 100ms
    pub reconnect_ivl: Duration,

    //  Maximum interval between attempts to reconnect
    //  Default 0 (unused)
    pub reconnect_ivl_max: Duration,

    //  Maximal size of message to handle.
    pub maxmsgsize: i64,
}

impl Options {
    pub fn new() -> Options {
        Options {
            identity_size: 0,
            type_: -1,
            maxmsgsize: -1,
            reconnect_ivl: Duration::milliseconds(100),
            reconnect_ivl_max: Duration::zero(),
        }
    }

    pub fn getsockopt(&self, option: SocketOption) -> int {
        match option {
            SocketOption::TYPE => self.type_ as int,
        }
    }
}
