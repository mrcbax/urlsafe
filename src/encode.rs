const MATCHES: [(&str, &str); 3] = [(".", "[.]"), ("http", "hxxp"), ("ftp", "fxp")];

pub fn url(mut url: String) -> String {
    for item in MATCHES.iter() {
        if url.contains(item.0) {
            url = url.replace(item.0, item.1);
        }
    }
    return url;
}
