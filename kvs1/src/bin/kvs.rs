use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("get value from provided key")
                .arg(Arg::with_name("get").value_name("KEY").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("set key-value pair")
                .arg(
                    Arg::with_name("set")
                        .value_names(&["KEY", "VALUE"])
                        .takes_value(true)
                        .number_of_values(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove key-value pair")
                .arg(Arg::with_name("rm").value_name("KEY").takes_value(true)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("get") => panic!("unimplemented"),
        Some("set") => panic!("unimplemented"),
        Some("rm") => panic!("unimplemented"),
        _ => panic!("no args"),
    }
}
