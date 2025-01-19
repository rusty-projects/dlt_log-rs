#[macro_use]
extern crate log;
extern crate dlt_log;

fn main() {
    dlt_log::init("TEST", "Rust tests", "EXPL", "Smoke test example").unwrap();

    trace!("Initialized Rust");
    debug!("Address is {:p}", main as *const ());
    info!("Did you know? {} = {}", "1 + 1", 2);
    warn!("Don't log sensitive information!");
    error!("Nothing more to say");
}
