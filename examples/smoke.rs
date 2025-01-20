extern crate dlt_log;
extern crate log;

fn main() {
    dlt_log::init("TEST", "Rust tests", "EXPL", "Smoke test example").unwrap();
    log::error!("Test error");
    log::warn!("Test warn");
    log::info!("Test info");
    log::debug!("Test debug");
    log::trace!("Test trace");
}
