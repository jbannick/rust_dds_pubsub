use std::{io, time};
use std::env;

use log4rs::{
  append::console::ConsoleAppender,
  config::{Appender, Root},
  Config,
};
use log::LevelFilter;
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
struct Structure(i32);
  
#[derive(Debug)]
struct Deep(Structure);
  
fn help() {
  println!("Usage: cargo run --example rust_dds_pubsub: [Sub or Pub]");
  println!("Required: Server Type: [Sub or Pub]");
}  

fn main() {
  println!("== Starting Rust DDS PubSub");
  let args: Vec<String> = env::args().collect();
  
  println!("args {:?}", args);
  
  match args.len() {
    2 => { // one argument
      let argval = &args[1];
      match argval.as_str() {
        "Sub" => {
          println!("Server is Subscriber");
        },
        "Pub" => {
        println!("Server is Publisher");
        },
        _ => {
        println!("***ERROR: Invalid argument {:?}", argval);
        help();
        return;
        }
      }
    }
    _ => {
      help();
      return;
      }
  }
  
  configure_logging();
  
  println!("Structure: {:?}", Structure(3));
  
  info!("Deep: {:?}", Deep(Structure(0)));
  debug!("Deep: {:?}", Deep(Structure(7)));
  trace!("Deep: {:?}", Deep(Structure(77)));
  warn!("Deep: {:?}", Deep(Structure(999)));    
  error!("Deep: {:?}", Deep(Structure(666)));
}


fn configure_logging() {
  debug!("Enter: configure_logging");
  
  log4rs::init_file(
    "logging-config.yaml",
    log4rs::config::Deserializers::default(),
  )
  .unwrap_or_else(|e| {
    match e.downcast_ref::<io::Error>() {
      Some(os_err) if os_err.kind() == io::ErrorKind::NotFound => {
        println!("No logging-config.yaml file found.");
        let stdout = ConsoleAppender::builder().build();
        let conf = Config::builder()
          .appender(Appender::builder().build("stdout", Box::new(stdout)))
          .build(Root::builder().appender("stdout").build(LevelFilter::Error))
          .unwrap();
        log4rs::init_config(conf).unwrap();
      }
      other_error => panic!("Logging config problem {other_error:?}"),
    }
  });
}
