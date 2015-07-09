use urlencoded::UrlEncodedBody;
use iron::prelude::*;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub text: String,
    pub lang: String,
    pub threshold: f32,
    pub max_distance: u32,
    pub html: bool,
    pub ignore_proper: bool,
    pub fuzzy: Option<f32>,
    pub global_threshold: Option<f32>,
    pub ignore: String,
        
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
            ignore_proper: false,
            fuzzy: None,
            global_threshold: None,
            ignore: String::new(),
        }
    }

    /// Sets a field according to hashmap entry
    fn set_field(&mut self, key: &str, value: &str) {
        // We're really laxist but well
        match key {
            "html" => self.html = true,
            "ignore_proper" => self.ignore_proper = true,
            "ignore_words" => self.ignore = value.to_string(),
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
                let mut config = match hashmap.get("text") {
                    Some(v) => Config::new(&v[0]),
                    None =>  return Err("Didn't find 'text' in POST hashmap".to_string())
                };
                if hashmap.contains_key("activate_fuzzy") {
                    if let Some(v) = hashmap.get("fuzzy") {
                        if let Ok(x) = v[0].parse::<f32>() {
                            config.fuzzy = Some(x);
                        }
                    }
                }
                if hashmap.contains_key("activate_global") {
                    if let Some(v) = hashmap.get("global_threshold") {
                        if let Ok(x) = v[0].parse::<f32>() {
                            config.global_threshold = Some(x);
                        }
                    }

                }
                for (key, value) in hashmap {
                    config.set_field(key, &value[0]);
                }
                Ok(config)
            },
            Err(ref e) => Err(e.description().to_string())
        }
    }
}
        
pub fn ips_from_args() -> Vec<String> {
    let mut args = env::args().into_iter();
    if args.len() < 2 {
        vec!("localhost:3000".to_string())
    } else {
        args.next();
        args.collect()
    }
}
   
