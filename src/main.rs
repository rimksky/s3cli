use aws_sdk_s3::primitives::ByteStream;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    match args.get(1).map(String::as_str) {
        Some("upload") => {
            // upload <bucket> <key> <file>
            let (bucket, key, file) = (&args[2], &args[3], &args[4]);
            let body = ByteStream::from_path(file).await?;
            client.put_object().bucket(bucket).key(key).body(body).send().await?;
            println!("Uploaded: {file} -> s3://{bucket}/{key}");
        }
        Some("download") => {
            // download <bucket> <key> [output]
            let (bucket, key) = (&args[2], &args[3]);
            let output = args.get(4).map(String::as_str).unwrap_or(
                key.rsplit('/').next().unwrap_or(key),
            );
            let resp = client.get_object().bucket(bucket).key(key).send().await?;
            let data = resp.body.collect().await?.into_bytes();
            std::fs::write(output, &data)?;
            println!("Downloaded: s3://{bucket}/{key} -> {output}");
        }
        Some("list") => {
            // list <bucket> [prefix]
            let bucket = &args[2];
            let mut req = client.list_objects_v2().bucket(bucket);
            if let Some(prefix) = args.get(3) {
                req = req.prefix(prefix);
            }
            let resp = req.send().await?;
            for obj in resp.contents() {
                println!("{:12}  {}", obj.size().unwrap_or(0), obj.key().unwrap_or(""));
            }
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  s3cli upload   <bucket> <key> <file>");
            eprintln!("  s3cli download <bucket> <key> [output]");
            eprintln!("  s3cli list     <bucket> [prefix]");
        }
    }

    Ok(())
}
