use crate::game::Game;

pub struct Connection {
    pub(crate) sender: ws::Sender,
}

impl ws::Handler for Connection {
    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        println!("Player connected");
        Ok(())
    }

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        //self.ws.send(message);
        println!("received: {}", message);
        // if let msg = message.as_text() {
        //     let parsed = serde_json::from_str(msg.unwrap());
        // }
        Ok(())
    }
}
