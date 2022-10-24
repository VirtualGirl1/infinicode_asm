struct section_map {
    name: Vec<String>,
    sec: Vec<section>,
}

struct section {
    name: String,
    tokens: Vec<token>,
}