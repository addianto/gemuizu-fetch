/// Generates a unique file name based on the given URL.
///
/// The generated file name is formatted as `<domain>_<game>_<identifier>.html`.
///
/// Example:
///
/// ```
/// let filename = gemuizu_fetch::generate_file_name("https://gamewith.net/palworld/43489");
/// assert_eq!("gamewith.net_palworld_43489.html", filename);
/// ```
pub fn generate_file_name(url: &str) -> String {
    let url_parts: Vec<&str> = url.split('/').collect();
    let length = url_parts.len();

    let domain: &str = url_parts[2];
    let game: &str = url_parts[3];
    let identifier: &str = url_parts[length - 1];

    format!("{}_{}_{}.html", domain, game, identifier)
}
