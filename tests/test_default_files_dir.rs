extern crate flexi_logger;

#[macro_use]
extern crate log;

use flexi_logger::{default_format,init,LogConfig};

#[test]
fn files_dir() {
    assert_eq!(
        (),
        init( LogConfig {
                 format: default_format,
                 log_to_file: true,
                 directory: Some("log_files".to_string()),
                 .. LogConfig::new()
               }, Some("info".to_string()) ).unwrap()
    );

    error!("This is an error message");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message - you must not see it!");
    trace!("This is a trace message - you must not see it!");
}
