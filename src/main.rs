use warp::{Filter,reply,http::Uri};
use serde::{Deserialize, Serialize};
use rust_embed::RustEmbed;
use std::env;
use std::env::args;
use std::process::Command;

// macro_rules! find {
//     (@,$e:expr) => {println!("{}",$e)};
// }
#[derive(Serialize, Deserialize)]
struct Rsp {
    code: u8,
    data: String,
}

#[derive(RustEmbed)]
#[folder = "src"]
struct Asset;

#[tokio::main]
async fn main() {
    let args : Vec<String> = args().collect();
    let pre_cmd = ["ping","host","mtr"];
    let mut port = 8080u16 ;
    let mut name = "Rust";
   // let mut cmd = vec!();
    println!("{:?}",args);
    for (i,v) in args.iter().enumerate(){
        if v == "-p" {
            port = args[i+1].parse().unwrap();
        }
        if v == "-n" {
            name = &args[i+1];
        }

    }

    // for c in &pre_cmd {
    //   let r = Command::new("command").args(&["-v",c]).status();
    //     match r {
    //         Ok(v) => cmd.push(c),
    //         Err(e) => {println!("{}",e);continue}
    //     };
    // }

    //println!("{:?} ",cmd);
    // println!("Please call {} at the number {}", p.name, p.phones[0]);
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hi = warp::path::end().map(|| {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        //let p: Person = serde_json::from_str(data).unwrap();
        //reply::json(&p)
        let index = Asset::get("index.html").unwrap();
        reply::html(index)
    });
    let index = warp::path("index").map(||{
            warp::redirect(Uri::from_static("/"))
    }
    );
    let css = warp::path!("index.css").map(||{
        let css = Asset::get("index.css").unwrap();
        reply::html(css)
    }).with(reply::with::header("content-type","text/css"));
    let hello_from_warp = warp::path!("hello" ).map(|| "Hello from warp!");
    let a = hi.or(hello_from_warp).or(index).or(css);
   // let g =warp::get().and(hi.or(hello_from_warp));
   // println!("{:?}",hi);
    warp::serve(a)
        .run(([0,0,0,0], port))
        .await;
}