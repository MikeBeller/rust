use std::env;
use std::thread;
use ureq;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nthreads = args[1].parse::<i32>().unwrap();
    let nreqs = args[2].parse::<i32>().unwrap();
    let mut children = vec![];

    for ti in 0..nthreads {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", ti);
            let mut total_fail = 0;
            let mut total_ok = 0;
            let agent = ureq::agent();
            for _i in 0..nreqs {
                let resp = agent.get("http://127.0.0.1:7878/hello")
                    .set("X-My-Header", "Secret")
                    .call();

                if resp.ok() {
                    total_ok += 1;
                } else {
                    total_fail += 1;
                }
            }
            println!("Ok: {} Fail: {}", total_ok, total_fail);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
