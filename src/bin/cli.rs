use clap::{Arg, Command};
use cr8s;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let matches = Command::new("cr8s")
        .version("0.1.0")
        .author("okre")
        .about("CLI for cr8s")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User administration command")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create user")
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("List users"))
                .subcommand(Command::new("update").about("Update user"))
                .subcommand(
                    Command::new("delete")
                        .about("Delete user")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("id")
                                .required(true)
                                .value_parser(clap::value_parser!(i32)),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => {
                cr8s::commands::create_user(
                    sub_matches
                        .get_one::<String>("username")
                        .unwrap()
                        .to_owned(),
                    sub_matches
                        .get_one::<String>("password")
                        .unwrap()
                        .to_owned(),
                    sub_matches
                        .get_many::<String>("roles")
                        .unwrap()
                        .map(|v| v.to_owned())
                        .collect(),
                )
                .await
            }
            Some(("list", _)) => cr8s::commands::list_users().await,
            Some(("delete", sub_matches)) => {
                cr8s::commands::delete_user(sub_matches.get_one::<i32>("id").unwrap().to_owned())
                    .await
            }

            _ => {}
        },
        _ => {}
    }
}
