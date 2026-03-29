# s3cli

A simple CLI tool for uploading, downloading, and listing files on AWS S3.

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

### 3. IAM role (EC2 / ECS / Lambda)

If an IAM role is attached to the instance or task, credentials are resolved automatically with no additional configuration.

### Required IAM permissions

| Command | Permission |
|---|---|
| upload | `s3:PutObject` |
| download | `s3:GetObject` |
| list | `s3:ListBucket` |

---

## Commands

### upload

```
s3cli upload <bucket> <key> <file>
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
s3cli download <bucket> <key> [output]
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
s3cli list <bucket> [prefix]
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
