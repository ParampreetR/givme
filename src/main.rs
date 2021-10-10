use clap::{App, Arg};
use givme::*;

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
        );
    // .arg(
    //     Arg::with_name("v")
    //         .short("v")
    //         .multiple(true)
    //         .help("Sets the level of verbosity"),
    // )
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

    let mut handle = GivMe::new();
    get_os_and_username(&mut handle);
    if is_first_run(&handle).unwrap() {
        run_setup(&mut handle).unwrap();
        std::process::exit(0);
    } else {
        get_sql_con(&mut handle);
    }
    if args.is_present("store") {
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
        if ask_pass_and_extract_key(&mut handle).unwrap() {
            let cred_to_give =
                give_credentials(args.value_of("key").unwrap().to_string(), &mut handle).unwrap();
            show_credentials(&cred_to_give);
        }
    } else {
        app.print_help().unwrap();
    }
}
