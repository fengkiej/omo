use clap::{App, load_yaml, crate_version};

fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let _matches = App::from(cli_yaml)
                        .version(crate_version!())
                        .get_matches();
}
