# s3cli

AWS S3 へのファイルアップロード・ダウンロードを行うシンプルなCLIツールです。

## ビルド

```bash
cargo build --release
# バイナリ: ./target/release/s3cli
```

---

## 認証

AWS の認証情報は以下の方法で設定します。優先順位の高い順に記載しています。

### 1. 環境変数（最優先）

```bash
export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
export AWS_REGION=ap-northeast-1
```

セッショントークンが必要な場合（IAMロール、SSO等）:

```bash
export AWS_SESSION_TOKEN=AQoDYXdzEJr...
```

### 2. 認証情報ファイル

`~/.aws/credentials` と `~/.aws/config` を使用します。

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

プロファイルを切り替えるには環境変数で指定します:

```bash
export AWS_PROFILE=myprofile
```

### 3. IAMロール（EC2 / ECS / Lambda）

EC2インスタンスやECSタスクにIAMロールがアタッチされている場合、追加設定なしで自動的に認証されます。

### 必要なIAMポリシー

| コマンド | 必要な権限 |
|---|---|
| upload | `s3:PutObject` |
| download | `s3:GetObject` |
| list | `s3:ListBucket` |

---

## コマンド

### upload — ファイルをアップロード

```
s3cli upload <bucket> <key> <file>
```

| 引数 | 説明 |
|---|---|
| `bucket` | S3バケット名 |
| `key` | S3上のオブジェクトキー（保存先パス） |
| `file` | アップロードするローカルファイルのパス |

```bash
# ファイルをアップロード
./s3cli upload my-bucket images/photo.jpg ./photo.jpg

# ディレクトリ階層を持つキーに保存
./s3cli upload my-bucket logs/2024/01/app.log ./app.log
```

---

### download — ファイルをダウンロード

```
s3cli download <bucket> <key> [output]
```

| 引数 | 説明 |
|---|---|
| `bucket` | S3バケット名 |
| `key` | S3上のオブジェクトキー |
| `output` | 保存先ファイル名（省略時はキーのファイル名部分） |

```bash
# カレントディレクトリに photo.jpg として保存
./s3cli download my-bucket images/photo.jpg

# 保存先を指定
./s3cli download my-bucket images/photo.jpg ./downloaded.jpg
```

---

### list — オブジェクト一覧を表示

```
s3cli list <bucket> [prefix]
```

| 引数 | 説明 |
|---|---|
| `bucket` | S3バケット名 |
| `prefix` | キーのプレフィックスでフィルタ（省略時は全件） |

```bash
# バケット内の全オブジェクトを表示
./s3cli list my-bucket

# プレフィックスでフィルタ
./s3cli list my-bucket images/
./s3cli list my-bucket logs/2024/
```

出力例:

```
      1048576  images/photo.jpg
        20480  logs/2024/01/app.log
```

左列がバイト数、右列がオブジェクトキーです。
