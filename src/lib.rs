#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate capnp;
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate tokio_io;
extern crate tokio_uds;
extern crate tokio_process;
#[macro_use]
extern crate capnp_rpc;
#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate tempdir;
extern crate memmap;
extern crate sysconf;
extern crate sys_info;
extern crate hyper;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;


pub mod common;
pub mod worker;
pub mod server;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const WORKER_PROTOCOL_VERSION: i32 = 0;
pub const CLIENT_PROTOCOL_VERSION: i32 = 0;
pub const SUBWORKER_PROTOCOL_VERSION: i32 = 0;



use std::sync::atomic::AtomicBool;
lazy_static! {
    // Init debug mode TODO: depend on opts
    pub static ref DEBUG_CHECK_CONSISTENCY: AtomicBool = AtomicBool::new(true);
}



// NOTE: Development solution to get type autocompletion and go-to-definition
pub mod capnp_gen;
pub use capnp_gen::*;

#[allow(unused_doc_comment)]
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        types {
            Error, ErrorKind, ResultExt;
        }
        foreign_links {
            Io(::std::io::Error);
            Capnp(::capnp::Error);
            CapnpNotInSchema(::capnp::NotInSchema);
            Timer(::tokio_timer::TimerError);
            SessionErr(::server::graph::SessionError);
            Utf8Err(::std::str::Utf8Error);
        }
    }
    // Explicit alias just to make the IDEs happier
    pub type Result<T> = ::std::result::Result<T, Error>;
}

impl std::convert::From<errors::Error> for capnp::Error {
    fn from(e: errors::Error) -> Self {
        capnp::Error::failed(e.description().to_string())
    }
}



/* // NOTE: PyCharm does not support feature switching, so using comments :'(
pub mod server_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/server_capnp.rs"));
}

pub mod client_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/client_capnp.rs"));
}

pub mod common_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/common_capnp.rs"));
}

pub mod worker_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/worker_capnp.rs"));
}

pub mod datastore_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/datastore_capnp.rs"));
}
*/
