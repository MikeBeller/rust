use actix_http::Error;
use awc;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let nreq = args[1].parse::<i32>().expect("invalid number");
    let client = awc::Client::new();
    let mut tot = 0;

    for _i in 0..nreq {
        let mut response = client
            .get("http://127.0.0.1:7878/342/fooblooglebloot/")
            .header("User-Agent", "Actix-web")
            .send()
            .await?;
        let body = response.body().await?;
        tot += body.len();
    }
    println!("total: {}", tot);

    Ok(())
}
