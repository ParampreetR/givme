use clap::{App, Arg};
use givme::*;
use std::{io, io::Write};

fn main() {
    let mut app = App::new("givme")
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
        );
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
    let args = app.clone().get_matches();
    let mut arg_hit = false;
    let mut handle = GivMe::new();
    get_os_and_username(&mut handle);
    if is_first_run(&handle).unwrap() {
        run_setup(&mut handle).unwrap();
        std::process::exit(0);
    } else {
        get_sql_con(&mut handle);
    }

    if args.is_present("delete") {
        arg_hit = true;
        if ask_pass_and_extract_key(&mut handle).unwrap() {
            let key_to_delete = args.value_of("delete").unwrap().to_string();
            match delete_credentails(key_to_delete.clone(), &mut handle) {
                Ok(_) => {
                    println!("'{}' deleted successfully", key_to_delete);
                }
                Err(err) => {
                    eprintln!("-- Error in deleting '{}'", key_to_delete);
                    eprintln!("{}", err);
                }
            };
        }
    }
    if args.is_present("store") {
        arg_hit = true;
        if ask_pass_and_extract_key(&mut handle).unwrap() {
            let cred = ask_user_for_value(args.value_of("store").unwrap()).unwrap();
            match save_credentials(cred, &mut handle) {
                Ok(_) => {
                    println!("Saved Successfully");
                }
                Err(er) => {
                    eprintln!("-- Error");
                    eprintln!("{}", er);
                }
            }
        }
    } else if args.is_present("key") {
        arg_hit = true;
        if ask_pass_and_extract_key(&mut handle).unwrap() {
            match give_credentials(args.value_of("key").unwrap().to_string(), &mut handle) {
                Some(cred) => {
                    if args.is_present("raw") {
                        print!("{}", cred.value);
                        io::stdout().flush().unwrap();
                    } else {
                        show_credentials(&cred);
                    }
                }
                None => {
                    eprintln!("-- Error: '{}' not found! ", args.value_of("key").unwrap());
                }
            };
        }
    }
    if !arg_hit {
        app.print_help().unwrap();
    }
}
