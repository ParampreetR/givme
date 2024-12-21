use givme::{commands::CommandStruct, config, utils};

fn main() {
    let mut app = utils::args::parse_args();
    let args = app.clone().get_matches();
    // if givme::utils::io::is_first_run().unwrap() {
    //     run_setup().unwrap();
    //     std::process::exit(0);
    // }

    // Initilize Services and Command Handler
    let services = config::loader::init_services();

    if let Err(err) = services.as_ref() {
        eprintln!("{}", err)
    }

    let services = services.unwrap();

    let mut command_handler = CommandStruct::new(givme::models::services::Services {
        encryption: services.services.encryption,
        sql: services.services.sql,
    });

    let mut result: String = String::new();
    // Handle command
    match args.subcommand() {
        ("store", Some(_)) => {
            let cred = utils::io::ask_user_for_value(args.value_of("store").unwrap()).unwrap();
            result = command_handler.save_encrypt_value(cred.key).expect("dsf");
        }
        ("get", Some(sub_m)) => {
            result = command_handler
                .get_decrypt_value(sub_m.value_of("key").unwrap())
                .unwrap();
        }
        (_, _) => {
            app.print_help().unwrap();
        }
    };

    if !result.is_empty() {
        println!("{}", result);
    }
    // if let Some(store_subcommand) = args.subcommand_matches("store") {}

    // if let Some(file_subcommand) = args.values_of("encrypt-file") {
    //     let paths: Vec<&str> = file_subcommand.collect();
    //     let in_path = paths[0];
    //     let out_path = paths[1];
    //     command_handler.encrypt_file(in_path, out_path);
    // }

    // if let Some(get_subcommand) = args.value_of("get") {}

    // if args.is_present("decrypt-file") {
    //     arg_hit = true;
    //     if ask_pass_and_extract_key(&mut handle).unwrap() {
    //         let paths: Vec<&str> = args.values_of("decrypt-file").unwrap().collect();
    //         let in_path = paths[0];
    //         let out_path = paths[1];
    //         match decrypt_file(in_path.to_string(), out_path.to_string(), &mut handle) {
    //             Ok(_) => {
    //                 println!("{} Decrypted Successfully to {}", in_path, out_path);
    //             }
    //             Err(err) => {
    //                 eprintln!("-- Error in Decryption of file '{}'", in_path);
    //                 eprintln!("{}", err);
    //             }
    //         };
    //     }
    // }

    // if args.is_present("delete") {
    //     arg_hit = true;
    //     if ask_pass_and_extract_key(&mut handle).unwrap() {
    //         let key_to_delete = args.value_of("delete").unwrap().to_string();
    //         match delete_credentails(key_to_delete.clone(), &mut handle) {
    //             Ok(_) => {
    //                 println!("'{}' deleted successfully", key_to_delete);
    //             }
    //             Err(err) => {
    //                 eprintln!("-- Error in deleting '{}'", key_to_delete);
    //                 eprintln!("{}", err);
    //             }
    //         };
    //     }
    // }
    // if args.is_present("store") {
    //     arg_hit = true;
    //     if ask_pass_and_extract_key(&mut handle).unwrap() {
    //         let cred = ask_user_for_value(args.value_of("store").unwrap()).unwrap();
    //         match save_credentials(cred, &mut handle) {
    //             Ok(_) => {
    //                 println!("Saved Successfully");
    //             }
    //             Err(er) => {
    //                 eprintln!("-- Error");
    //                 eprintln!("{}", er);
    //             }
    //         }
    //     }
    // } else if args.is_present("key") {
    //     arg_hit = true;
    //     if ask_pass_and_extract_key(&mut handle).unwrap() {
    //         match give_credentials(args.value_of("key").unwrap().to_string(), &mut handle) {
    //             Some(cred) => {
    //                 if args.is_present("raw") {
    //                     print!("{}", cred.value);
    //                     io::stdout().flush().unwrap();
    //                 } else {
    //                     show_credentials(&cred);
    //                 }
    //             }
    //             None => {
    //                 eprintln!("-- Error: '{}' not found! ", args.value_of("key").unwrap());
    //             }
    //         };
    //     }
    // }

    // if args.is_present("get-secret-key") {
    //     arg_hit = true;
    //     if ask_pass_and_extract_key(&mut handle).unwrap() {
    //         let secret_key = get_secret_key(&mut handle);
    //         println!("{}", secret_key);
    //     }
    // } else if args.is_present("set-secret-key") {
    //     arg_hit = true;
    //     if ask_pass_and_extract_key(&mut handle).unwrap() {}
    // }

    // if !arg_hit {
    // }
}
