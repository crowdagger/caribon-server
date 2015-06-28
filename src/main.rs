extern crate iron;
extern crate router;
extern crate caribon;
extern crate urlencoded;

use urlencoded::UrlEncodedBody;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use caribon::Parser;
use router::Router;
use std::error::Error;

fn main() {
    let mut router = Router::new();

    router.get("/", show_form);
    router.post("/result", show_result);

    fn show_form(_: &mut Request) -> IronResult<Response> {
        let s = format!("
<html>
<body>
<h1>Caribon (online version)</h1>
<p>For more information see <a href = 'https://github.com/lady-segfault/caribon'>Caribon's github page</a></p>
<form method = 'post' action = '/result'>
<textarea name = 'text' rows = '20' cols = '60'>Enter text here</textarea>
<p>Contains HTML?
<input type='checkbox' name='html' value='true' checked/></p>
<p>Ignore proper nouns?
<input type='checkbox' name='ignore_proper' value='true'/></p>
<p>Max distance to consider a repetition: 
<input type='text' name='max_distance' value='50'/></p>
<p>Threshold to underline a word:
<input type='text' name='threshold' value='1.9'/></p>
<p>Language: 
<select name = 'language'>
{}
</select>
</p>

<p>
<input type='submit' value='OK'></p>
</form>
</body>
</html>",
                        Parser::list_languages().iter()
                        .map(|s| format!("<option value = '{}' {}>{}</option>",
                                         s,
                                         if s == &"french" {"selected = 'selected'"} else {""},
                                         s))
                        .fold(String::new(), |s1, s2| s1 + &s2));
        let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, s)))
    }

    // Receive a message by POST and play it back.
    fn show_result(request: &mut Request) -> IronResult<Response> {
        // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
        let config:Result<String,String> = match request.get_ref::<UrlEncodedBody>() {
            Ok(ref hashmap) => {
                println!("{:?}", hashmap);
                match hashmap.get("text") {
                    Some(v) => Ok(v[0].to_string()),
                    None => {
                        Err("Didn't find 'text' in POST hashmap".to_string())
                    }
                }
            },
            Err(ref e) => {
                Err(e.description().to_string())
            }
        };


        let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
        match config {
            Ok(s) => {
                let parser = Parser::new("french").unwrap();
                let words = parser.tokenize(&s);
                let repetitions = parser.detect_local(words);
                let html = caribon::words_to_html(&repetitions, 1.9);
                println!("html: {},", html);
                Ok(Response::with((content_type, status::Ok, html)))
            }
            Err(s) => Ok(Response::with((content_type, status::Ok, s)))
        }
    }

    Iron::new(router).http("192.168.0.32:3000").unwrap();
}
