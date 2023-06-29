pub struct ConnectionContainer {}

impl ConnectionContainer {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum ConnectionContainerError {
    BadConnection,
}
