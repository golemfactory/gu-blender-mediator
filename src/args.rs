use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short = "p", long = "listen-port", default_value = "0")]
    pub listen_port: u16,

    // Address for gateway server (example: http://127.0.0.1:55001/)
    #[structopt(long = "gw", default_value = "")]
    pub gw_addr: String,

    // Address for webdav server, (example: http://127.0.0.1:55011)
    #[structopt(long = "dav", default_value = "")]
    pub dav_addr: String,

    #[structopt(long = "local")]
    pub local: bool,
}
