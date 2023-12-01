use anyhow::Result;
use actix::prelude::*;
use crate::display::Display;

/// Define message
#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct FlushMsg {
    pub buffer: Vec<u8>
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct RefreshMsg {}

pub struct DisplayActor {
    display: Display,
}

impl Default for DisplayActor {
    fn default() -> Self {
        Self {
            display: Display::new().unwrap()
        }
    }
}

// Provide Actor implementation for our actor
impl Actor for DisplayActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

impl Handler<FlushMsg> for DisplayActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: FlushMsg, _: &mut Self::Context) -> Self::Result {
        self.display.flush(msg.buffer)?;
        Ok(())
    }
}

impl Handler<RefreshMsg> for DisplayActor {
    type Result = Result<()>;

    fn handle(&mut self, _: RefreshMsg, _: &mut Self::Context) -> Self::Result {
        self.display.refresh()?;
        Ok(())
    }
}
