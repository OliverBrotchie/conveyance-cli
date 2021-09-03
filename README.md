<img src='https://image.flaticon.com/icons/png/512/1275/1275584.png' height="200" width="200" style="margin-bottom:50px" />

# Conveyancing CLI

Fill out word document fields with JSON data.

## Installation

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
cargo install conveyance
```

## Usage


```shell
conveyance --file example.docx --json example.json --output new-document.docx
```

**OR**

```shell
conveyance -f example.docx -j example.json -o new-document.docx
```

If you are confused about how to use the program at any point, type in the following:

```shell
conveyance --help
```
