use rusoto_core::Region;
use rusoto_s3::{S3, S3Client};

fn main() {
    let s3_client = S3Client::new(Region::UsEast1);
    let res = s3_client.list_buckets().sync().unwrap();
    for bucket in res.buckets.unwrap() {
        println!("{}", bucket.name.unwrap());
    }
}
