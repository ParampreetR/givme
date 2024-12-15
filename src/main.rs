use std::{io, io::Write};

use givme::{
    ask_pass_and_extract_key, ask_user_for_value, decrypt_file, delete_credentails, encrypt_file,
    get_os_and_username, get_secret_key, get_sql_con, give_credentials, givme::GivMe, is_first_run,
    parse_args, run_setup, save_credentials, show_credentials,
};

fn main() {
    let mut app = parse_args();
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

    if args.is_present("encrypt-file") {
        arg_hit = true;
        if ask_pass_and_extract_key(&mut handle).unwrap() {
            let paths: Vec<&str> = args.values_of("encrypt-file").unwrap().collect();
            let in_path = paths[0];
            let out_path = paths[1];
            match encrypt_file(in_path.to_string(), out_path.to_string(), &mut handle) {
                Ok(_) => {
                    println!("{} Encrypted Successfully to {}", in_path, out_path);
                }
                Err(err) => {
                    eprintln!("-- Error in Encryption of file '{}'", in_path);
                    if err.kind() == io::ErrorKind::InvalidData {
                        eprintln!("[!>] Only pure text files can be encrypted");
                    }
                    eprintln!("{:?}", err);
                }
            };
        }
    }

    if args.is_present("decrypt-file") {
        arg_hit = true;
        if ask_pass_and_extract_key(&mut handle).unwrap() {
            let paths: Vec<&str> = args.values_of("decrypt-file").unwrap().collect();
            let in_path = paths[0];
            let out_path = paths[1];
            match decrypt_file(in_path.to_string(), out_path.to_string(), &mut handle) {
                Ok(_) => {
                    println!("{} Decrypted Successfully to {}", in_path, out_path);
                }
                Err(err) => {
                    eprintln!("-- Error in Decryption of file '{}'", in_path);
                    eprintln!("{}", err);
                }
            };
        }
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

    if args.is_present("get-secret-key") {
        arg_hit = true;
        if ask_pass_and_extract_key(&mut handle).unwrap() {
            let secret_key = get_secret_key(&mut handle);
            println!("{}", secret_key);
        }
    } else if args.is_present("set-secret-key") {
        arg_hit = true;
        if ask_pass_and_extract_key(&mut handle).unwrap() {}
    }

    if !arg_hit {
        app.print_help().unwrap();
    }
}
