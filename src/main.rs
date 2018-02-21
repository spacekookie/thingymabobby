extern crate egg_mode;
extern crate chrono;
extern crate tokio_core;
extern crate futures;

mod common;

use tokio_core::reactor;
use egg_mode::tweet::DraftTweet;


fn main() {
    let mut core = reactor::Core::new().unwrap();
    let config = common::Config::load(&mut core);
    let handle = core.handle();

    let draft = DraftTweet::new("Did you know you can make vegan cookies with flour, bananas and HUMAN LOVE!");
    core.run(draft.send(&config.token, &handle)).unwrap();
}