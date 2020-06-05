use crate::my::cli;
use crate::my::structs;


lazy_static! {
    pub static ref CONFIG: structs::Config = {
        return cli::do_cli();
    };
}
