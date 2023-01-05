

pub fn get_preview(body: String) -> String {
    if body.len() > 600 {
        body[..550].to_string()
    } else {
        body
    }
}