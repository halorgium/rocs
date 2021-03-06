extern crate clap;
extern crate rocs;

use clap::{App, AppSettings, Arg, SubCommand};
use rocs::apis::client::APIClient;
use rocs::apis::configuration::Configuration;
use rocs::cli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rocs")
        .version("0.1")
        .author("Rodrigo Vaz")
        .about("Rust OSB Client 'Super'")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("broker_url")
                .short("b")
                .long("broker")
                .env("ROCS_BROKER_URL")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("broker_user")
                .short("u")
                .long("username")
                .env("ROCS_BROKER_USERNAME")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("broker_pass")
                .short("a")
                .long("password")
                .env("ROCS_BROKER_PASSWORD")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("json")
                .help("Prints result in JSON format")
                .long("json")
                .takes_value(false)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("provision")
                .about("Service Instance provisioning")
                .arg(
                    Arg::with_name("service")
                        .short("s")
                        .long("service")
                        .takes_value(true)
                        .required(true)
                        .help("service offering to use for provision"),
                )
                .arg(
                    Arg::with_name("plan")
                        .short("p")
                        .long("plan")
                        .takes_value(true)
                        .required(true)
                        .help("service plan to use for provision"),
                )
                .arg(
                    Arg::with_name("parameters")
                        .short("P")
                        .long("params")
                        .takes_value(true)
                        .help("parameters to provision service instance"),
                )
                .arg(
                    Arg::with_name("wait")
                        .short("w")
                        .long("wait")
                        .takes_value(false)
                        .help("wait service instance provisioning to finish"),
                ),
        )
        .subcommand(
            SubCommand::with_name("deprovision")
                .about("Service Instance deprovisioning")
                .arg(
                    Arg::with_name("instance-id")
                        .short("i")
                        .long("instance-id")
                        .takes_value(true)
                        .required(true)
                        .help("service intance id to deprovision"),
                ),
        )
        .subcommand(
            SubCommand::with_name("bind")
                .about("Service Binding request")
                .arg(
                    Arg::with_name("instance-id")
                        .short("i")
                        .long("instance")
                        .takes_value(true)
                        .help("instance ID or name to bind")
                        .required(true),
                )
                .arg(
                    Arg::with_name("parameters")
                        .short("P")
                        .long("params")
                        .takes_value(true)
                        .help("parameters to provision service instance"),
                ),
        )
        .subcommand(
            SubCommand::with_name("unbind")
                .about("Service Binding removal")
                .arg(
                    Arg::with_name("binding-id")
                        .short("b")
                        .long("binding")
                        .takes_value(true)
                        .help("Binding ID to unbind")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("catalog")
                .about("Catalog request")
                .alias("cat"),
        )
        .subcommand(
            SubCommand::with_name("credentials")
                .alias("creds")
                .about("Binding credentials")
                .arg(
                    Arg::with_name("binding-id")
                        .short("b")
                        .long("binding")
                        .takes_value(true)
                        .help("Binding ID to fetch credentials")
                        .required(false),
                )
                .arg(
                    Arg::with_name("instance-id")
                        .short("i")
                        .long("instance")
                        .takes_value(true)
                        .help("Instance ID to fetch credentials")
                        .required(false),
                ),
        )
        .get_matches();

    let mut cfg = Configuration::new();
    cfg.basic_auth = Some((
        matches.value_of("broker_user").unwrap().to_owned(),
        Some(matches.value_of("broker_pass").unwrap().to_owned()),
    ));
    cfg.base_path = matches.value_of("broker_url").unwrap().to_owned();

    let options = cli::Options {
        json_output: matches.is_present("json"),
        synchronous: matches.is_present("sync"),
    };

    let client = APIClient::new(cfg);

    match matches.subcommand_name() {
        Some("catalog") => cli::catalog(
            matches.subcommand_matches("catalog").unwrap(),
            client,
            options,
        ),
        Some("provision") => cli::provision(
            matches.subcommand_matches("provision").unwrap(),
            client,
            options,
        ),
        Some("deprovision") => cli::deprovision(
            matches.subcommand_matches("deprovision").unwrap(),
            client,
            options,
        ),
        Some("bind") => cli::bind(matches.subcommand_matches("bind").unwrap(), client, options),
        Some("unbind") => cli::unbind(
            matches.subcommand_matches("unbind").unwrap(),
            client,
            options,
        ),
        Some("creds") => cli::creds(
            matches.subcommand_matches("creds").unwrap(),
            client,
            options,
        ),
        _ => Ok(()),
    }
}
