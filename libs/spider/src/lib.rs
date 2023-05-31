use regex::Regex;

pub fn capture(url: &str, html_content: &str) {
    let url_pattern = Regex::new(r"(https?://[^\s<>']+)").unwrap();

    for capture in url_pattern.captures_iter(&html_content) {
        let url_match = &capture[0];
        if url_match.starts_with(url) {
            println!("[O] URL: {}", url_match);
        } else {
            println!("[X] URL: {}", url_match);
        }
    }
}