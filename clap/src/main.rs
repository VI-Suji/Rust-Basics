use clap::{command, Arg, Command};

fn main() {
    let match_result= command!()
    .about("This is for first name and last name collections")
    .subcommand(
        Command::new("register-person")
                .arg(
                    Arg::new("first_name")
                        .short('f')
                        .long("first_name")
                        .aliases(["fname","firstname"])
                        // .required(true)
                        .help("The person's first name")
                        // .conflicts_with("last_name")
                )
                .arg(
                    Arg::new("last_name")
                        .short('l')
                        .long("last_name")
                        .aliases(["lname","lastname"])
                        .required(true)
                        .help("The person's last name")
                )
    ).arg(
        Arg::new("fluffy")
            .long("fluffy")
            .help("Is person wearing coat")
    )
    .subcommand(Command::new("register-pet")
        .arg(
            Arg::new("pet-name")
                .long("pet-name")
                .short('n')
        )
    ).get_matches();
    // println!("Fluffy : {:#?}",match_result.get_one::<String>("first_name").unwrap_or(&"Nothing found".to_string()));

    let person_args = match_result.subcommand_matches("register-person");
    println!("Hello {} ",person_args.unwrap().get_one::<String>("first_name").unwrap_or(&"Nothing found".to_string()));
}