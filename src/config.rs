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
            ignore_proper: false,
            fuzzy: None,
            global_threshold: None,
            ignore: String::new(),
        }
    }

    /// Sets a field according to hashmap entry
    fn set_field(&mut self, key: &str, value: &str) -> Result<(), String> {
        // We're really laxist but well
        match key {
            "ignore_proper" => self.ignore_proper = true,
            "ignore_words" => self.ignore = value.to_string(),
            "max_distance" => self.max_distance = match value.parse::<u32>() {
                Ok(x) => x,
                Err(_) => return Err("Max distance must be a positive integer".to_string())
            },
            "threshold" => self.threshold = match value.parse::<f32>() {
                Ok(x) => x,
                Err(_) => return Err("Local threshold must be a float".to_string())
            },
            "language" => self.lang = value.to_string(),
            _ => {}
        }
        Ok(())
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
                        match v[0].parse::<f32>() {
                            Ok(x) => config.fuzzy = Some(x),
                            Err(_) => return Err("Fuzzy threshold must be a float".to_string())
                        }
                    }
                }
                if hashmap.contains_key("activate_global") {
                    if let Some(v) = hashmap.get("global_threshold") {
                        config.global_threshold = match v[0].parse::<f32>() {
                            Ok(x) => Some(x),
                            Err(_) => return Err("Global threshold must be a float".to_string())
                        };
                    }

                }
                for (key, value) in hashmap {
                    try!(config.set_field(key, &value[0]));
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
   
