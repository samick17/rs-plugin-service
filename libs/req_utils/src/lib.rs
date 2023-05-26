use reqwest::blocking::Client;

pub fn create_client() -> Client {
    Client::new()
}
pub fn req_get(url: &str, client: &Client) {
    let status = client.get(url)
    .send()
    .unwrap()
    .status();
    println!("{}", status);
}
