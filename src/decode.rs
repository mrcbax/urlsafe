const MATCHES: [(&str, &str); 4] = [("[", ""), ("]", ""), ("hxxp", "http"), ("fxp", "ftp")];

pub fn url(mut url: String) -> String {
    for item in MATCHES.iter() {
        if url.contains(item.0) {
            url = url.replace(item.0, item.1);
        }
    }
    return url;
}
