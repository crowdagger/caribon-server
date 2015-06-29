use urlencoded::UrlEncodedBody;
use iron::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub text: String,
    pub lang: String,
    pub threshold: f32,
    pub max_distance: u32,
    pub html: bool,
    pub ignore_proper: bool,
}

impl Config {
    /// New default config
    pub fn new(text: &str) -> Config {
        Config {
            text: text.to_string(),
            lang: "french".to_string(),
            threshold: 1.9,
            max_distance: 50,
            html: false,
            ignore_proper: false
        }
    }

    /// Sets a field according to hashmap entry
    fn set_field(&mut self, key: &str, value: &str) {
        // We're really laxist but well
        match key {
            "text" => {}, // do nothing since it has been handled by new
            "html" => self.html = true,
            "ignore_proper" => self.ignore_proper = true,
            "max_distance" => if let Ok(x) = value.parse() { self.max_distance = x },
            "threshold" => if let Ok(x) = value.parse() { self.threshold = x },
            "language" => self.lang = value.to_string(),
            _ => {}
        }
    }

    pub fn new_from_request(request: &mut Request)
                            -> Result<Config,String> {
        match request.get_ref::<UrlEncodedBody>() {
            Ok(hashmap) => {
                match hashmap.get("text") {
                    Some(v) => {
                        let mut config = Config::new(&v[0]);
                        for (key, value) in hashmap {
                            config.set_field(key, &value[0]);
                        }
                        Ok(config)
                    },
                    None =>  Err("Didn't find 'text' in POST hashmap".to_string())
                }
            },
            Err(ref e) => Err(e.description().to_string())
        }
    }
}
        


