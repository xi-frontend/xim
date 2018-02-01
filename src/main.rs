#[macro_use]
extern crate clap;

#[macro_use]
extern crate error_chain;

extern crate log4rs;
#[macro_use]
extern crate log;

extern crate futures;
extern crate termion;
extern crate tokio_core;
extern crate xrl;

mod xim;
mod errors;
mod terminal;
mod view;

use futures::{Future, Stream};
use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use tokio_core::reactor::Core;
use xrl::spawn;

use errors::*;
use xim::{Xim, XimServiceBuilder};

fn configure_logs(logfile: &str) {
    let xim = FileAppender::builder().build(logfile).unwrap();
    let rpc = FileAppender::builder()
        .build(&format!("{}.rpc", logfile))
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("xim", Box::new(xim)))
        .appender(Appender::builder().build("rpc", Box::new(rpc)))
        .logger(
            Logger::builder()
                .appender("xim")
                .additive(false)
                .build("xi_micro", LogLevelFilter::Debug),
        )
        .logger(
            Logger::builder()
                .appender("xim")
                .additive(false)
                .build("xrl", LogLevelFilter::Info),
        )
        .logger(
            Logger::builder()
                .appender("rpc")
                .additive(false)
                .build("xrl::protocol::codec", LogLevelFilter::Trace),
        )
        .build(Root::builder().appender("xim").build(LogLevelFilter::Info))
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();

        writeln!(stderr, "error: {}", e).unwrap();
        error!("error: {}", e);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).unwrap();
            error!("error: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).unwrap();
            error!("error: {}", e);
        }
        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let xi = clap_app!(
        xi =>
        (about: "The Xi Editor")
        (@arg core: -c --core +takes_value "Specify binary to use for the backend")
        (@arg logfile: -l --logfile +takes_value "Log file location")
        (@arg file: +required "File to edit"));

    let matches = xi.get_matches();
    if let Some(logfile) = matches.value_of("logfile") {
        configure_logs(logfile);
    }

    info!("starting the event loop");
    let mut core = Core::new().chain_err(|| "failed to create event loop")?;

    info!("starting xi-core");
    let (xim_builder, core_events_rx) = XimServiceBuilder::new();
    let (client, core_stderr) = spawn(
        matches.value_of("core").unwrap_or("xi-core"),
        xim_builder,
        &core.handle(),
    );

    let error_logging = core_stderr
        .for_each(|msg| {
            error!("core: {}", msg);
            Ok(())
        })
        .map_err(|_| ());
    core.handle().spawn(error_logging);

    info!("starting logging xi-core errors");

    info!("initializing the xim");
    let mut xim =
        Xim::new(core.handle(), client, core_events_rx).chain_err(|| "failed initialize the xim")?;
    xim.open(matches.value_of("file").unwrap_or("").to_string());
    xim.set_theme("base16-eighties.dark");

    info!("spawning the xim on the event loop");
    core.run(xim)
        .chain_err(|| "an error occured while running the xim")?;
    Ok(())
}
