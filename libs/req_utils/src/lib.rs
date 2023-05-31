use reqwest::blocking::Client;

pub fn create_client() -> Client {
    Client::new()
}
pub fn req_get(url: &str, client: &Client) -> String {
    let resp = client.get(url)
    .send()
    .unwrap();
    // let status = resp.status();
    let text = resp.text().unwrap();
    text
}
