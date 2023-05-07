fn main() {
    let token = dbg!(std::env::var("API_TOKEN").expect("Need a token to run the program"));
    let mut args_it = std::env::args();
    args_it.next();
    let args: String = args_it.collect();

    let client = reqwest::blocking::Client::new();
    let rsp = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", token), ("keyword", args)])
        .send()
        .expect("a successful return")
        .json::<serde_json::Value>()
        .expect("a json response");
    dbg!(rsp);
}
