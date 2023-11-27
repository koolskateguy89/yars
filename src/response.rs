use std::collections::HashMap;

// TODO: type builder thingy so status is required
pub struct HTTPResponse {
    status: Option<u32>, // idk if u32 is right
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HTTPResponse {
    // TODO: .header(key: String, value: String)
    // TODO: .headers(/* probably not a map, maybe Iter<(String, String)>? */)
    // TODO: .status(status: u32)

    // TODO: .json(json: String) (final)
    // TODO: .xml(xml: String) (final)
    // TODO: .html(html: String) (final)
    // TODO: .text(text: String) (final)
    // TODO: .body(body: String) (final - doesnt set content type)

    // TODO?: some way to send binary data
}

// TODO: tests
