use clap::{App, Arg};

pub fn parse_args() -> clap::App<'static, 'static> {
    App::new("givme")
        .about("A Simple and Secure Password Manager")
        .arg(
            Arg::with_name("store")
                .short("s")
                .long("store")
                .value_name("KEY")
                .help("Asks value for key to store safely")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .help("Key to query")
                .required(false)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("raw")
                .short("r")
                .required(false)
                .takes_value(false)
                .help("Outputs only value for a key"),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .required(false)
                .value_name("KEY")
                .long("delete")
                .takes_value(true)
                .help("Deletes key value pair of given key"),
        )
        .arg(
            Arg::with_name("encrypt-file")
                .required(false)
                .value_name("SOURCE and DESTINATION FILE")
                .long("enc-file")
                .takes_value(true)
                .number_of_values(2)
                .help("Encrypts specified file"),
        )
        .arg(
            Arg::with_name("decrypt-file")
                .required(false)
                .value_name("SOURCE and DESTINATION FILE")
                .long("dec-file")
                .takes_value(true)
                .number_of_values(2)
                .help("Decrypts specified encrypted file"),
        )
        .arg(
            Arg::with_name("get-secret-key")
                .required(false)
                .long("get-secret-key")
                .takes_value(false)
                .help("Retrieve secret key from database"),
        )
        .arg(
            Arg::with_name("set-secret-key")
                .required(false)
                .value_name("secretkey")
                .long("set-secret-key")
                .takes_value(true)
                .number_of_values(1)
                .help("Sets the secret key of one encryption standard"),
        )
    // .subcommand(
    //     SubCommand::with_name("test")
    //         .about("controls testing features")
    //         .version("1.3")
    //         .author("Someone E. <someone_else@other.com>")
    //         .arg(
    //             Arg::with_name("debug")
    //                 .short("d")
    //                 .help("print debug information verbosely"),
    //         ),
    // )
}
