use std::io::{self,BufRead};

#[derive(Debug)]
struct Data {
    date: String,
    close: f64,
    prc: f64,
    vol: i64,
}

fn main() {
    let mut data: Vec<Data> = vec![];
    let stdin = io::stdin();
    for opline in stdin.lock().lines() {
        let line = opline.unwrap();
        let f = line.split(',').collect::<Vec<&str>>();
        if f[0] == "Date" {
            continue;
        }
        let d = Data{
            date: f[0].to_string(),
            close: f[4].parse().unwrap(),
            prc: f[5].parse().unwrap(),
            vol: f[6].parse().unwrap(),
        };
        data.push(d);
    }
    println!("{:?}", data);
}
