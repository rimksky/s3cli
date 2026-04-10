# s3cli

A simple CLI tool for uploading, downloading, and listing files on AWS S3, and listing buckets.

You can also override the S3 endpoint in the same three ways used by AWS CLI, which makes the CLI usable with S3 endpoint-based access such as PrivateLink and proxy-bypass setups.

## Build

```bash
cargo build --release
# binary: ./target/release/s3cli
```

---

## Authentication

Credentials are resolved in the following order of priority:

### 1. Environment variables

```bash
export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
export AWS_REGION=ap-northeast-1
```

With a session token (IAM role, SSO, etc.):

```bash
export AWS_SESSION_TOKEN=AQoDYXdzEJr...
```

### 2. Credentials file

Uses `~/.aws/credentials` and `~/.aws/config`:

```ini
# ~/.aws/credentials
[default]
aws_access_key_id = AKIAIOSFODNN7EXAMPLE
aws_secret_access_key = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY

[myprofile]
aws_access_key_id = AKIAI44QH8DHBEXAMPLE
aws_secret_access_key = je7MtGbClwBF/2Zp9Utk/h3yCo8nvbEXAMPLEKEY
```

```ini
# ~/.aws/config
[default]
region = ap-northeast-1

[profile myprofile]
region = us-east-1
```

Switch profiles with:

```bash
export AWS_PROFILE=myprofile
```

### S3 endpoint settings

`s3cli` follows the same S3 endpoint configuration order as AWS CLI:

1. `--endpoint-url <url>`
2. `AWS_ENDPOINT_URL_S3`
3. `~/.aws/config` service-specific S3 settings

Examples:

```bash
./s3cli --endpoint-url https://bucket.vpce-xxxxxxxx.s3.ap-northeast-1.vpce.amazonaws.com list my-bucket
./s3cli --endpoint-url https://bucket.vpce-xxxxxxxx.s3.ap-northeast-1.vpce.amazonaws.com upload my-bucket logs/app.log ./app.log
```

```bash
export AWS_ENDPOINT_URL_S3=https://s3-ap-northeast-1.amazonaws.com
./s3cli list my-bucket
```

```ini
# ~/.aws/config
[default]
region = ap-northeast-1
services = s3

[services s3]
s3 =
  endpoint_url = https://s3-ap-northeast-1.amazonaws.com
```

### 3. IAM role (EC2 / ECS / Lambda)

If an IAM role is attached to the instance or task, credentials are resolved automatically with no additional configuration.

### Required IAM permissions

| Command | Permission |
|---|---|
| buckets | `s3:ListAllMyBuckets` |
| upload | `s3:PutObject` |
| download | `s3:GetObject` |
| list | `s3:ListBucket` |

---

## Commands

### buckets

```
s3cli [--endpoint-url <url>] buckets
```

Lists all S3 buckets in the account.

```bash
./s3cli buckets
```

Output:

```
my-bucket
another-bucket
logs-bucket
```

---

### upload

```
s3cli [--endpoint-url <url>] upload <bucket> <key> <file>
```

| Argument | Description |
|---|---|
| `bucket` | S3 bucket name |
| `key` | Object key (destination path in S3) |
| `file` | Local file path to upload |

```bash
./s3cli upload my-bucket images/photo.jpg ./photo.jpg
./s3cli upload my-bucket logs/2024/01/app.log ./app.log
```

---

### download

```
s3cli [--endpoint-url <url>] download <bucket> <key> [output]
```

| Argument | Description |
|---|---|
| `bucket` | S3 bucket name |
| `key` | Object key to download |
| `output` | Local file path to save (defaults to the filename part of key) |

```bash
./s3cli download my-bucket images/photo.jpg
./s3cli download my-bucket images/photo.jpg ./downloaded.jpg
```

---

### list

```
s3cli [--endpoint-url <url>] list <bucket> [prefix]
```

| Argument | Description |
|---|---|
| `bucket` | S3 bucket name |
| `prefix` | Filter by key prefix (optional) |

```bash
./s3cli list my-bucket
./s3cli list my-bucket images/
./s3cli list my-bucket logs/2024/
```

Output:

```
      1048576  images/photo.jpg
        20480  logs/2024/01/app.log
```

Left column is size in bytes, right column is the object key.

## License

MIT License — Copyright (c) 2026 rimksky@gmail.com
