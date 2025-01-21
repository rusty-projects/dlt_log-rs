extern crate dlt_log;
extern crate log;

fn main() {
    // initialize DLT logger, ignore errors
    let _ = dlt_log::init("TEST", "Rust tests", "EXPL", "Smoke test example");

    log::trace!("Tracing the untraceable!");
    log::debug!("Debugging the debugger!");
    log::info!("Information overload: {} + {} = {}", 2, 2, 4);
    log::warn!("Warning: Low on coffee!");
    log::error!("Error: Something went terribly right!");
}
