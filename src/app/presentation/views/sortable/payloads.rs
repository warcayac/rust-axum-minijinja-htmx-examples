use axum::extract::RawForm;

#[derive(Debug)]
pub struct Payload {
    pub items: Vec<u8>,
}

impl Payload {
    pub fn from_raw(raw: RawForm) -> Self {
        let items = String::from_utf8_lossy(&raw.0)
            .split('&')
            .filter_map(|pair| {
                pair.split('=')
                    .nth(1)
                    .and_then(|value| {
                        urlencoding::decode(value)
                            .ok()
                            .and_then(|decoded| {
                                decoded
                                    .trim_end_matches('/')
                                    .parse::<u8>()
                                    .ok()
                            })
                    })
            })
            .collect::<Vec<_>>()
        ;
        Self { items }
    }
}
