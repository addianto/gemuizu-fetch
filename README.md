# Gemuizu Fetch

Gemuizu Fetch is a utility that fetches web pages from GameWith ([EN](https://gamewith.net)/[JP](https://gamewith.jp))
and presents them as plain text. It has been tested with [Palworld Wiki Guide (EN)](https://gamewith.net/palworld/)
and [Umamusume Strategy Wiki (JP)](https://gamewith.jp/uma-musume/).

## Usage

```shell
$ gemuizu_fetch [OPTIONS] <URL1> <URL2> ... <URLn>
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
