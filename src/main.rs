use paho_mqtt::message::Message;
use paho_mqtt::client::Client;
use paho_mqtt::errors::Error;
// use paho_mqtt::errors::Result;
// use paho_mqtt::create_options::CreateOptions;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::{thread, time};
// use std::{env, process};


fn main() -> Result<(), Error> {
    
    let dur = time::Duration::from_secs(5);
    let client = Client::new("tcp://localhost:1883").unwrap();
    let hostname = generate_hostname();
    let topic = generate_topic(hostname);
    loop {
        let payload = gen_payload();
        let message = Message::new_retained(&topic, payload, 2);
        println!("message: {:?}", message);
        client.publish(message)?;
        thread::sleep(dur)
    };
}

fn generate_hostname() -> String {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    String::from(random_string)
}

fn generate_topic(hostname: String) -> String {
    format!("/things/{}/temp", hostname)
}

fn gen_payload() -> Vec<u8> {
    vec![thread_rng().gen::<u8>()]
}