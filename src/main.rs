// Bloom
//
// HTTP REST API caching middleware
// Copyright: 2017, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

#[macro_use] extern crate log;
#[macro_use] extern crate clap;
extern crate ini;
extern crate hyper;
extern crate farmhash;
extern crate futures;

mod config;
mod header;
mod proxy;
mod cache;
mod control;
mod server;

use clap::{App, Arg};
use config::logger::ConfigLogger;
use config::reader::ConfigReaderBuilder;
use cache::store::CacheStoreBuilder;
use proxy::serve::ProxyServeBuilder;
use control::listen::ControlListenBuilder;
use server::listen::ServerListenBuilder;

fn main() {
    let _logger = ConfigLogger::init();

    info!("starting up");

    let app = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!("\n"))
                .about(crate_description!())
                .arg(Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .help("Path to configuration file")
                    .default_value("./config.cfg")
                    .takes_value(true));

    let args = app.get_matches();
    let conf = ConfigReaderBuilder::new().read(
        args.value_of("config").unwrap());

    // Bind to cache store
    let cache_store = CacheStoreBuilder::new(conf.memcached);
    cache_store.bind();

    // Create serve manager
    let proxy_serve = ProxyServeBuilder::new(conf.proxy);

    // Run control interface (in its own thread)
    ControlListenBuilder::new(conf.control).run();

    // Run server (in main thread)
    ServerListenBuilder::new(conf.server).run(proxy_serve, cache_store);

    error!("could not start");
}
