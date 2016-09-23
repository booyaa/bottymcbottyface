# bottymcbottyface

A great example (see what I did there) of how to misuse examples in Rust Crates.

Also a collection of IRC bots:

- mouthpiece - one shot irc client, you give it a message to send and it quits afterwards.


More to come!

# Installation

All bots can be installed using

```
cargo install --example <bot> bottymcbottyface
```

e.g. to install `mouthpiece` you would type:

```
cargo install --example mouthpiece bottymcbottyface
```

# Usage

## mouthpiece

```
USAGE:
    mouthpiece [OPTIONS] --nick <nick> --server <server> --channel <channel> --message <message>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --channel <channel>       Set the channel to join (required)
    -m, --message <message>       The message to send (required)
    -n, --nick <nick>             Set the nick of your bot (required)
    -p, --port <port>             Set the port to connect to (defaults to 6667)
    -r, --realname <real name>    Sets your real name (appears when someone runs /whois on your bot)
    -s, --server <server>         Set the server to connect to (required)
    -u, --username <user name>    Sets your username (appears next to your conecting host when someone runs /whois on your bot)
```

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.
