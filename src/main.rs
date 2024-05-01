use clap::{Arg, ArgAction, ArgMatches, Command};
use scraper::{Html, Selector};
use std::error::Error;
use std::path::PathBuf;

fn build_cli() -> Command {
    Command::new(clap::crate_name!())
        .bin_name(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("url")
                .help("URL(s) to a web page on GameWith")
                .required(true)
                .value_name("URL")
                .value_parser(clap::value_parser!(String))
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Write the output to a FILE")
                .required(false)
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Debug mode")
                .required(false)
                .value_name("DEBUG")
                .value_parser(clap::value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
}

fn parse_title(document: &Html) -> Result<String, Box<dyn Error>> {
    let selector: Selector = Selector::parse("head > title:nth-child(1)")?;

    let element = match document.select(&selector).next() {
        Some(element) => element,
        None => return Err("title element not found".into()),
    };

    let text: String = match element.text().next() {
        Some(text) => String::from(text),
        None => return Err("title text not found".into()),
    };

    Ok(text)
}

fn parse_article_body(document: &Html) -> Result<String, Box<dyn Error>> {
    let selector: Selector = Selector::parse("div#article-body p")?;

    let text = document
        .select(&selector)
        .map(|e| e.text())
        .map(|t| t.collect::<Vec<_>>())
        .map(|t| t.join(" "))
        .reduce(|acc, t| acc + " " + &t);

    match text {
        Some(text) => Ok(text),
        None => Err("cannot parse article body".into()),
    }
}

fn get_urls(args: &ArgMatches) -> Result<Vec<String>, Box<dyn Error>> {
    match args.get_many::<String>("url") {
        Some(urls) => Ok(urls.map(|url| url.to_string()).collect()),
        None => return Err("url is required".into()),
    }
}

fn handle_debug(args: &ArgMatches, urls: &Vec<String>) {
    if args.contains_id("debug") {
        let debug_flag: bool = args.get_flag("debug");

        if debug_flag {
            println!("url: {}", urls.join(" "));
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Command = build_cli();
    let args: ArgMatches = cli.get_matches();

    let urls: Vec<String> = get_urls(&args)?;

    handle_debug(&args, &urls);

    for url in urls {
        let response_body: String = ureq::get(&url)
            .set(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; rv:125.0) Gecko/20100101 Firefox/125.0",
            )
            .call()?
            .into_string()?;

        let html_document: Html = Html::parse_document(&response_body);
        let title: String = parse_title(&html_document)?;
        let article_body: String = parse_article_body(&html_document)?;
        let content: String = format!("{} {}", title, article_body);

        if args.contains_id("output") {
            let output_file: PathBuf = match args.get_one::<PathBuf>("output") {
                Some(output) => output.into(),
                None => return Err("missing output file path".into()),
            };

            std::fs::write(output_file, content.as_bytes())?;
        } else {
            println!("{}", content);
        }
    }

    Ok(())
}
