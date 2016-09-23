#![allow(unused_must_use)]
extern crate hiirc;
extern crate clap;

use std::sync::Arc;
use hiirc::*;
use clap::{Arg, App};

// this code is largely inspired by the peekaboo example in hiirc.

const APP_VERSION: &'static str = concat!("(",
                                          env!("CARGO_PKG_NAME"),
                                          " v",
                                          env!("CARGO_PKG_VERSION"),
                                          ")");
const APP_NAME: &'static str = "mouthpiece";
const APP_DESCRIPTION: &'static str = "Single shot irc client";
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

const AD_BANNER: &'static str = concat!(env!("CARGO_PKG_NAME"),
                                        " v",
                                        env!("CARGO_PKG_VERSION"),
                                        env!("CARGO_PKG_HOMEPAGE"));

const DEFAULT_PORT: &'static str = "6667";


/// This is the struct on which the listener is implemented.
struct Mouthpiece<'a> {
    channel: &'a str,
    message: String,
}

impl<'a> Mouthpiece<'a> {
    pub fn new(channel: &str, message: String) -> Mouthpiece {
        // adds two more errors <'a> {
        // pub fn new(channel: &str) -> Mouthpiece {
        Mouthpiece {
            channel: channel,
            message: message,
        }
    }
}

impl<'a> Listener for Mouthpiece<'a> {
    /// On any event we receive, print the Debug of it.
    fn any(&mut self, _: Arc<Irc>, event: &Event) {
        println!("{:?}", &event);
    }

    /// When the welcome message is received, join the channel.
    fn welcome(&mut self, irc: Arc<Irc>) {
        irc.join(self.channel, None);
    }

    /// When the channel is joined, say message and quit.
    fn channel_join(&mut self, irc: Arc<Irc>, channel: Arc<Channel>) {
        irc.privmsg(channel.name(), &self.message);
        irc.quit(Some(AD_BANNER));
    }
}

fn main() {
    let matches = App::new(APP_NAME)
                      .version(APP_VERSION)
                      .author(APP_AUTHOR)
                      .about(APP_DESCRIPTION)
                      .arg(Arg::with_name("nick")
                               .short("n")
                               .long("nick")
                               .help("Set the nick of your bot (required)")
                               .takes_value(true)
                               .required(true))
                      .arg(Arg::with_name("server")
                               .short("s")
                               .long("server")
                               .help("Set the server to connect to (required)")
                               .takes_value(true)
                               .required(true))
                      .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .help("Set the port to connect to (defaults to 6667)")
                               .takes_value(true))
                      .arg(Arg::with_name("channel")
                               .short("c")
                               .long("channel")
                               .help("Set the channel to join (required)")
                               .takes_value(true)
                               .required(true))
                      .arg(Arg::with_name("message")
                               .short("m")
                               .long("message")
                               .help("The message to send (required)")
                               .takes_value(true)
                               .required(true))
                      .arg(Arg::with_name("real name")
                               .short("r")
                               .long("realname")
                               .help("Sets your real name (appears when someone runs /whois on \
                                      your bot)")
                               .takes_value(true))
                      .arg(Arg::with_name("user name")
                               .short("u")
                               .long("username")
                               .help("Sets your username (appears next to your conecting host \
                                      when someone runs /whois on your bot)")
                               .takes_value(true))
                      .get_matches();

    let nick = matches.value_of("nick").unwrap();
    let server = matches.value_of("server").unwrap();
    let port = matches.value_of("port").unwrap_or(DEFAULT_PORT);
    let channel = matches.value_of("channel").unwrap();
    let message = matches.value_of("message").unwrap().to_string();

    let real_name = matches.value_of("real name").unwrap_or(AD_BANNER);
    let user_name = matches.value_of("user name").unwrap_or(&nick);

    let server_port = format!("{}:{}", server, port);
    Settings::new(&server_port, &nick)
        .username(&user_name)
        .realname(&real_name)
        .dispatch(Mouthpiece::new(channel, message))
        .unwrap();
}
