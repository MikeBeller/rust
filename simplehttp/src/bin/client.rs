use ureq;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let nreqs = args[1].parse::<i32>().unwrap();

    let mut total_bytes = 0;
    for _i in 0..nreqs {
        let resp = ureq::get("http://127.0.0.1:7878/hello")
            .set("X-My-Header", "Secret")
            .call();

        if resp.ok() {
            total_bytes += resp.into_string().unwrap().len();
        } else {
            println!("ERR: {}", resp.into_string().unwrap());
        }
    }

    println!("Total: {}", total_bytes);
}
