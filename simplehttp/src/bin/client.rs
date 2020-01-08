use ureq;

fn main() {
    let resp = ureq::get("http://127.0.0.1:7878/hello")
        .set("X-My-Header", "Secret")
        .call();

    if resp.ok() {
        println!("OK: {}", resp.into_string().unwrap());
    } else {
        println!("ERR: {}", resp.into_string().unwrap());
    }
}
