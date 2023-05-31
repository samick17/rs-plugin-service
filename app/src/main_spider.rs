use req_utils;
use spider;

pub fn main() {
    let client = req_utils::create_client();
    // let url = "https://www.wikipedia.org";
    let url = "https://www.google.com";
    let raw_data = req_utils::req_get(url, &client);
    spider::capture(url, &raw_data);
}
