use aws_config::SdkConfig;
use aws_sdk_s3::primitives::ByteStream;
use std::error::Error;

struct CliOptions {
    endpoint_url: Option<String>,
    command: Command,
}

enum Command {
    Upload {
        bucket: String,
        key: String,
        file: String,
    },
    Download {
        bucket: String,
        key: String,
        output: Option<String>,
    },
    Buckets,
    List {
        bucket: String,
        prefix: Option<String>,
    },
    Help,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = parse_args(std::env::args().skip(1).collect())?;
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = build_s3_client(&config, options.endpoint_url.as_deref());

    match options.command {
        Command::Upload { bucket, key, file } => {
            let body = ByteStream::from_path(&file).await?;
            client
                .put_object()
                .bucket(&bucket)
                .key(&key)
                .body(body)
                .send()
                .await?;
            println!("Uploaded: {file} -> s3://{bucket}/{key}");
        }
        Command::Download {
            bucket,
            key,
            output,
        } => {
            let output =
                output.unwrap_or_else(|| key.rsplit('/').next().unwrap_or(&key).to_owned());
            let resp = client.get_object().bucket(&bucket).key(&key).send().await?;
            let data = resp.body.collect().await?.into_bytes();
            std::fs::write(&output, &data)?;
            println!("Downloaded: s3://{bucket}/{key} -> {output}");
        }
        Command::Buckets => {
            let resp = client.list_buckets().send().await?;
            for bucket in resp.buckets() {
                println!("{}", bucket.name().unwrap_or(""));
            }
        }
        Command::List { bucket, prefix } => {
            let mut req = client.list_objects_v2().bucket(&bucket);
            if let Some(prefix) = prefix {
                req = req.prefix(prefix);
            }
            let resp = req.send().await?;
            for obj in resp.contents() {
                println!(
                    "{:12}  {}",
                    obj.size().unwrap_or(0),
                    obj.key().unwrap_or("")
                );
            }
        }
        Command::Help => print_usage(),
    }

    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<CliOptions, Box<dyn Error>> {
    let mut endpoint_url = None;
    let mut position = 0;

    while let Some(arg) = args.get(position) {
        match arg.as_str() {
            "--endpoint-url" => {
                let Some(value) = args.get(position + 1) else {
                    return Err(format!("Error: {arg} requires <url>").into());
                };
                validate_endpoint_url(value)?;
                endpoint_url = Some(value.clone());
                position += 2;
            }
            "--help" | "-h" => {
                return Ok(CliOptions {
                    endpoint_url,
                    command: Command::Help,
                });
            }
            _ if arg.starts_with("--") => {
                return Err(format!("Error: unknown option {arg}").into());
            }
            _ => break,
        }
    }

    let command_args = &args[position..];
    let command = match command_args.first().map(String::as_str) {
        Some("upload") => {
            let (Some(bucket), Some(key), Some(file)) = (
                command_args.get(1),
                command_args.get(2),
                command_args.get(3),
            ) else {
                return Err("Error: upload requires <bucket> <key> <file>".into());
            };
            Command::Upload {
                bucket: bucket.clone(),
                key: key.clone(),
                file: file.clone(),
            }
        }
        Some("download") => {
            let (Some(bucket), Some(key)) = (command_args.get(1), command_args.get(2)) else {
                return Err("Error: download requires <bucket> <key>".into());
            };
            Command::Download {
                bucket: bucket.clone(),
                key: key.clone(),
                output: command_args.get(3).cloned(),
            }
        }
        Some("buckets") => Command::Buckets,
        Some("list") => {
            let Some(bucket) = command_args.get(1) else {
                return Err("Error: list requires <bucket>".into());
            };
            Command::List {
                bucket: bucket.clone(),
                prefix: command_args.get(2).cloned(),
            }
        }
        Some(command) => return Err(format!("Error: unknown command {command}").into()),
        None => Command::Help,
    };

    Ok(CliOptions {
        endpoint_url,
        command,
    })
}

fn validate_endpoint_url(url: &str) -> Result<(), Box<dyn Error>> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("Error: endpoint URL must start with http:// or https://".into());
    }
    Ok(())
}

fn build_s3_client(config: &SdkConfig, endpoint_url: Option<&str>) -> aws_sdk_s3::Client {
    let mut builder = aws_sdk_s3::config::Builder::from(config);

    if let Some(endpoint_url) = endpoint_url {
        builder = builder.endpoint_url(endpoint_url);
    }

    aws_sdk_s3::Client::from_conf(builder.build())
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  s3cli buckets");
    eprintln!("  s3cli upload   <bucket> <key> <file>");
    eprintln!("  s3cli download <bucket> <key> [output]");
    eprintln!("  s3cli list     <bucket> [prefix]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --endpoint-url <url>    Override the S3 endpoint URL for all commands");
    eprintln!("  -h, --help              Show this help");
}
