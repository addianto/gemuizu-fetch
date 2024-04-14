# Gemuizu Fetch

Gemuizu Fetch is a utility that fetches web pages from [GameWith Wiki Site](https://gamewith.jp) and presents them as plain text.
It has been tested with the [Umamusume Strategy Wiki](https://gamewith.jp/uma-musume/), which is hosted on GameWith.

## Usage

```shell
$ gemuizu_fetch [OPTIONS] <URL>
```

Options:

- `-o FILE`, `--output FILE`
  
  Write the output to a `FILE`.
- `-h`, `--help`

  Print a help guide.
- `-v`, `--version`

  Print the current version.

## Why?

I develop this utility to gather data sets for developing a Retrieval-Augmented Generation (RAG) agent.
In addition, I want to practice writing code in Rust.

## License

This project is licensed under the [MIT](./LICENSE) license.
