// A list of tokens that we want to replace.
const MATCHES: [(&str, &str); 3] = [(".", "[.]"), ("http", "hxxp"), ("ftp", "fxp")];

pub fn url(mut url: String) -> String {
    // Main replacing loop.
    for item in MATCHES.iter() {
        // String replace is expensive when the token does not exist. Contains
        // is *marginally* better so test first.
        if url.contains(item.0) {
            url = url.replace(item.0, item.1);
        }
    }
    return url;
}
