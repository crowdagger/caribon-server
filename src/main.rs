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

fn main() {
    fn router() -> Router {
        let mut router = Router::new();
        router.get("/", show_form);
        router.get("/style.css", show_css);
        router.get("/serialize.js", show_js);
        router.post("/result", show_result);
        router
    }

    fn show_js(_: &mut Request) -> IronResult<Response> {
        let js = include_str!("html/serialize-0.2.js");
        let content_type = "text/javascript".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, js)))
    }

    
    fn show_css(_: &mut Request) -> IronResult<Response> {
        let css = include_str!("html/main.css");
        let content_type = "text/css".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, css)))
    }

    fn show_form(_: &mut Request) -> IronResult<Response> {
        let default_text = "Enter some text in this field and if there are some repetitions we will show them to you!";
        let parser = Parser::new("english").unwrap();
        let html = caribon::words_to_html(&parser.detect_local(parser.tokenize(default_text)),
                                 1.9,
                                 false);
        let s = format!(include_str!("html/main.html.in"),
                        default_text,
                        Parser::list_languages().iter()
                        .map(|s| format!("<option value = '{}' {}>{}</option>",
                                         s,
                                         if s == &"french" {"selected = 'selected'"} else {""},
                                         s))
                        .fold(String::new(), |s1, s2| s1 + &s2),
                        html);
        let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, s)))
    }

    // Receive a message by POST and play it back.
    fn show_result(request: &mut Request) -> IronResult<Response> {
        // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
        fn compute_output(request: &mut Request) -> String {
            let result:Result<Config,String> = Config::new_from_request(request);
            match result {
                Ok(config) => {
                    let option = Parser::new(&config.lang);
                    if let Some(parser) = option {
                        let parser = parser
                            .with_max_distance(config.max_distance)
                            .with_html(config.html);
                        let words = parser.tokenize(&config.text);
                        let repetitions = parser.detect_local(words);
                        let html = caribon::words_to_html(&repetitions, config.threshold, false);
                        html
                    } else {
                        "Language not implemented".to_string()
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
    println!("ips: {:?}", ips);
    let mut res:Vec<HttpResult<Listening>> = vec!();
    
    for ip in ips {
        res.push(Iron::new(router()).http(&*ip));
    }
}
