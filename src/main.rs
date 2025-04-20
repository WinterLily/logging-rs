mod logging;
use logging::Level;
use std::time::Duration;
use tokio::time::sleep;

/**
Example implementation of a logging system using the `logging` crate.
*/
#[tokio::main]
async fn main() {
    // 1. Initialize the logging system
    logging::init(Level::Debug);

    // 2. Use the logging macros
    info!("Application starting up...");
    dbug!("This is a debug message. Verbose details here.");
    okay!("Something succeeded. Task completed.");
    warn!("This is a warning. Something might be wrong.");
    fail!("A failure occurred! Critical error.");

    info!("Logging demonstration complete. Shutting down shortly.");

    // 3. Add a small delay
    //      This is just to give some time for the logs to finish writing before we exit.
    sleep(Duration::from_millis(100)).await;
}
