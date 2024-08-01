use std::{io, time};
use std::env;

use rustdds::*;
use rustdds::no_key::{DataReader, DataWriter, DataSample}; // We use a NO_KEY topic here
use serde::{Serialize, Deserialize};

use log4rs::{
  append::console::ConsoleAppender,
  config::{Appender, Root},
  Config,
};
use log::LevelFilter;
use futures::StreamExt;
use smol::Timer;

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
          // println!("Server is Subscriber");
        },
        "Pub" => {
          // println!("Server is Publisher");
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

  // Create a DDS DomainParticipant
  let domain_participant = DomainParticipant::new(0).unwrap();
  info!("Created the DomainParticipant");
  
  // Create a DDS Quality of Service
  let qos = QosPolicyBuilder::new()
    .reliability(policy::Reliability::Reliable {max_blocking_time: rustdds::Duration::ZERO})
    .build();
  info!("Created the Quality of Service");
  
  // Create a DDS Subscriber 
  let subscriber = domain_participant.create_subscriber(&qos).unwrap();
  info!("Created the DDS Subscriber");
  
  // Create a DDS Publisher
  let publisher = domain_participant.create_publisher(&qos).unwrap();
  info!("Created the DDS Publisher");
  
  // Create the DDS Topic
  let some_topic = domain_participant.create_topic("some_topic".to_string(), "SomeType".to_string(), &qos, TopicKind::NoKey).unwrap();
  info!("Created the DDS Topic");
  
  #[derive(Serialize, Deserialize, Debug)]
  struct SomeType {
    a: i32
  }
  
  let mut reader = subscriber
    .create_datareader_no_key::<SomeType, CDRDeserializerAdapter<SomeType>>(
      &some_topic,
      None)
    .unwrap();
  info!("Created the DDS DataReader");
  
  let mut writer = publisher
    .create_datawriter_no_key::<SomeType, CDRSerializerAdapter<SomeType>> (
      &some_topic,
      None)
    .unwrap();
  info!("Created the DDS DataWriter");
  
  // ---
  
  let pub_or_sub = "Sub";
  
  info!("pub_or_sub = {:?}", pub_or_sub);
  if "Sub" == pub_or_sub {
    println!("Server is Subscriber");
  } else if "Pub" == pub_or_sub {
    println!("Server is Publisher");
  } else {
    error!("Invalid argument: {:?}", pub_or_sub);
    return;
  }
  
}


fn configure_logging() {
  println!("Enter: configure_logging");
  
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
