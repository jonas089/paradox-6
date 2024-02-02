// use tokio as async client -> tbd!
use reqwest::blocking::Client;
use std::fmt;

pub struct Peer{
    pub host: String,
    pub port: u16
}
impl fmt::Display for Peer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.host.clone() + ":" + &self.port.to_string())
    }
}

pub struct Gossiper{
    pub peers: Vec<Peer>
}
impl Gossiper{
    pub fn gossip(&self, message: String) -> Vec<(String, Result<reqwest::blocking::Response, reqwest::Error>)>{
        let mut responses: Vec<(String, Result<reqwest::blocking::Response, reqwest::Error>)> = Vec::new();
        let data: Vec<(&str, &String)> = vec![("proof_serialized", &message)];
        let client = Client::new();
        for peer in &self.peers{
            let url = peer.to_string() + &String::from("/proof");
            let response: Result<reqwest::blocking::Response, reqwest::Error> = Ok(client.post(&url).form(&data).send().unwrap());
            println!("[Debug] Response from peer: {:?}", response);
            responses.push((peer.to_string(), response));
        };

        responses
    }
}

#[test]
fn test(){
    let peer = Peer{host: String::from("http://127.0.0.1"), port: 8000u16};
    assert_eq!(peer.to_string(), "http://127.0.0.1:8000");
}