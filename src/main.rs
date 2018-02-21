extern crate egg_mode;
extern crate chrono;
extern crate tokio_core;
extern crate futures;
extern crate daumdic;
extern crate rand;

mod common;

use tokio_core::reactor;
use egg_mode::tweet::DraftTweet;

use std::fs::File;
use std::io::prelude::*;

use std::{thread, time};


fn main() {
    let mut f = File::open("src/data").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let words: Vec<&str> = contents.split("\n").collect();
    let length = words.len();

    loop {
        let (product, first, second) = (rand::random::<usize>() % length, rand::random::<usize>() % length, rand::random::<usize>() % length);

        let mut core = reactor::Core::new().unwrap();
        let config = common::Config::load(&mut core);
        let handle = core.handle();

        let thing = format!("Did you know you can make {} with {}, {} and HUMAN LOVE!", words[product], words[first], words[second]);
        println!("{}", thing);
        
        let draft = DraftTweet::new(thing);
        core.run(draft.send(&config.token, &handle)).unwrap();

        let ten_millis = time::Duration::from_millis(600000);
        thread::sleep(ten_millis);
    }
}