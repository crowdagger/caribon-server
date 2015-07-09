mod config;
extern crate iron;
extern crate router;
extern crate caribon;
extern crate urlencoded;
extern crate hyper;

use config::Config;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::error::HttpResult;
use hyper::server::Listening;
use caribon::Parser;
use router::Router;
use std::error::Error;

macro_rules! main_html {
    ("english") => (include_str!("html/main.html.in"));
    ("french") => (include_str!("html/main.fr.html.in"));
}


fn main() {
    fn router() -> Router {
        let mut router = Router::new();
        router.get("/", show_en);
        router.get("/en", show_en);
        router.get("/fr", show_fr);
//        router.get("/url", show_url);
        router.get("/style.css", show_css);
        router.get("/serialize.js", show_js);
        router.post("/result", show_result);
        router.get("/foundation.css", show_foundation_css);
        router.get("/normalize.css", show_normalize_css);
        router.get("/foundation.js", show_foundation_js);
        router.get("/caribon.png", show_logo);
        router
    }

    // fn show_url(_: &mut Request) -> IronResult<Response> {
    //     let url = "http://linuxfr.org/";
    //     let mut html:String = String::new();

    //     let client = Client::new();
    //     let mut res = client.get(url).send().unwrap();

    //     if res.status != hyper::Ok {
    //         html = html + "Error fetching URL";
    //         println!("status:{:?}", res.status);
    //     } else {
    //         res.read_to_string(&mut html).unwrap();
    //     }

    //     let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
    //     Ok(Response::with((content_type, status::Ok, html)))
    // }

    fn show_logo(_: &mut Request) -> IronResult<Response> {
        let img:&'static[u8] = include_bytes!("html/caribon.png");
        let content_type = "image/png".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, img)))
    }

    fn show_js(_: &mut Request) -> IronResult<Response> {
        let js = include_str!("html/serialize-0.2.js");
        let content_type = "text/javascript".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, js)))
    }

    fn show_foundation_js(_: &mut Request) -> IronResult<Response> {
        let js = include_str!("html/foundation.min.js");
        let content_type = "text/javascript".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, js)))
    }

    
    fn show_css(_: &mut Request) -> IronResult<Response> {
        let css = include_str!("html/main.css");
        let content_type = "text/css".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, css)))
    }

    fn show_foundation_css(_: &mut Request) -> IronResult<Response> {
        let css = include_str!("html/foundation.min.css");
        let content_type = "text/css".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, css)))
    }

    fn show_normalize_css(_: &mut Request) -> IronResult<Response> {
        let css = include_str!("html/normalize.css");
        let content_type = "text/css".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, css)))
    }

    fn display_list_languages(lang: &str) -> String {
        Parser::list_languages().iter()
            .map(|s| format!("<option value = '{}' {}>{}</option>",
                             s,
                             if s == &lang {"selected = 'selected'"} else {""},
                             s))
            .fold(String::new(), |s1, s2| s1 + &s2)
    }

    fn get_form(lang: &str, text: &str) -> IronResult<Response> {
        let parser = Parser::new(lang).unwrap();
        let mut ast = parser.tokenize(text).unwrap();
        parser.detect_local(&mut ast, 1.9);
        let result = parser.ast_to_html(&mut ast, false);
        let s = match lang {
            "english" => format!(main_html!("english"), text, display_list_languages(lang), result),
            "french" => format!(main_html!("french"), text, display_list_languages(lang), result),
            _ => panic!("Unknown lang")
        };
        let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, s)))
    }

    fn show_fr(_: &mut Request) -> IronResult<Response> {
        let default_text = "Entrez du texte dans ce champ et s'il y a des répétitions dans le texte elles seront soulignées ci-dessous";
        get_form("french", default_text)
    }

    fn show_en(_: &mut Request) -> IronResult<Response> {
        let default_text = "Enter some text in this field and if there are some repetitions we will show them to you!";
        get_form("english", default_text)
    }

    // Try to parse
    fn try_parse(config:Config) -> Result<String, Box<Error>> {
        let mut parser = try!(Parser::new(&config.lang));
        parser = parser
            .with_max_distance(config.max_distance)
            .with_fuzzy(config.fuzzy)
            .with_more_ignored(&config.ignore)
            .with_html(config.html);
        let mut ast = try!(parser.tokenize(&config.text));
        parser.detect_local(&mut ast, config.threshold);
        if let Some(threshold) = config.global_threshold {
            parser.detect_global(&mut ast, threshold);
        }
        let html = parser.ast_to_html(&mut ast, false);
        Ok(html)
    }

    // Receive a message by POST and play it back.
    fn show_result(request: &mut Request) -> IronResult<Response> {
        // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
        fn compute_output(request: &mut Request) -> String {
            let result:Result<Config,String> = Config::new_from_request(request);
            match result {
                Ok(config) => {
                    match try_parse(config) {
                        Ok(s) => s,
                        Err(e) => e.description().to_string()
                    }
                }
                Err(s) => s,
            }
        }
        
        let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
        let html = compute_output(request);
        Ok(Response::with((content_type, status::Ok, html)))        
    }

    let ips = config::ips_from_args();
    let mut res:Vec<HttpResult<Listening>> = vec!();
    
    for ip in ips {
        res.push(Iron::new(router()).http(&*ip));
    }
}
