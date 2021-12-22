use clap::{App, Arg};

pub struct AppArgs {
    pub host: String,
    pub database: String,
    pub port: u32,
}

impl AppArgs {
    pub fn new() -> anyhow::Result<AppArgs> {
        let matches = App::new("web_server")
                        .version("1.0")
                        .author("t3mp")
                        .about("Simple HTTP Server")
                        .arg(Arg::with_name("port")
                            .short("p")
                            .long("port")
                            .value_name("PORT")
                            .help("using port number")
                            .takes_value(true))
                        .get_matches();
        
        let port = matches.value_of("port").unwrap_or("8080");

        Ok(
            AppArgs {
                port: port.parse().unwrap_or(8080),
            }
        )
    }
}
