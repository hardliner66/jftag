# jftag

CLI and library to encode JSON into a short, URL-safe text string and decode it back.

Encoding minifies the JSON, compresses it with deflate, and encodes it as base64 (URL-safe, no padding).

## Build

```
cargo build --release
```

## Usage

```
jftag encode [FILE]   # JSON -> encoded text
jftag decode [FILE]   # encoded text -> pretty-printed JSON
```

Input is read from `FILE`, or from stdin if omitted or `-`.

```
$ echo '{"a": [1,2,3], "name": "hello hello hello"}' | jftag encode
q1ZKVLKKNtQx0jGO1VHKS8xNVbJSykjNyclXQCKVagE

$ echo 'q1ZKVLKKNtQx0jGO1VHKS8xNVbJSykjNyclXQCKVagE' | jftag decode
{
  "a": [
    1,
    2,
    3
  ],
  "name": "hello hello hello"
}
```

Decoding always pretty-prints, so original formatting is not preserved.
