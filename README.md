# LZY Codec

一種變長文本編解碼方案，支持對Unicode進行編解碼。編解碼效率、存儲空間全面優於UTF-8，未來會替代UTF-8成為新的世界通用編解碼標準。
rust實現。

[github](https://github.com/lizongying/lzy-codec-rs)

[crates](https://crates.io/crates/lzy-codec)

更多cli工具請參考 [go](https://github.com/lizongying/lzy-codec-go)

## 各語言實現

* [go](https://github.com/lizongying/lzy-codec-go)
* [py](https://github.com/lizongying/lzy-codec-py)
* [js](https://github.com/lizongying/lzy-codec-js)
* [php](https://github.com/lizongying/lzy-codec-php)
* [c](https://github.com/lizongying/lzy-codec-c)
* [dotnet](https://github.com/lizongying/lzy-codec-dotnet)

## 引入

```shell
cargo add lzy-codec
```

or

```toml
[dependencies]
lzy-codec = "0.1.1"
```

or

```toml
lzy-codec = { version = "0.1.0", registry = "crates.io" }
```

or

```toml
lzy-codec = { git = "https://github.com/lizongying/lzy-codec-rs.git" }
```

## 示例

```shell
cargo run --example basic_example
```

or

```rust
use lzy_codec::lzy;

fn main() {
    let message = "hello，世界😊".as_bytes();
    println!("Original: {:?}", message);

    let encoded = lzy::encode_from_bytes(message);
    println!("Encoded: {:?}", encoded);

    match encoded {
        Ok(encoded_bytes) => match lzy::decode_to_bytes(&encoded_bytes) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(decoded_str) => {
                    println!("Decoded: {:?}", decoded_str);
                }
                Err(utf8_err) => {
                    eprintln!("Failed to convert decoded bytes to string: {}", utf8_err);
                }
            },
            Err(decode_err) => {
                eprintln!("Failed to decode encoded bytes: {}", decode_err);
            }
        },
        Err(encode_err) => {
            eprintln!("Failed to encode bytes: {}", encode_err);
        }
    }
}
```

```shell
cargo run
```

## 讚賞

![image](./screenshots/appreciate.png)