use gemuizu_fetch::generate_file_name;

#[test]
fn generate_file_name_gamewith_jp_ok() -> Result<(), String> {
    // Setup
    let url: &str = "https://gamewith.jp/uma-musume/article/show/447601";

    // Exercise
    let output: String = generate_file_name(url);

    // Verify
    let expected: &str = "gamewith.jp_uma-musume_447601.html";
    assert_eq!(expected, output);

    Ok(())
}

#[test]
fn generate_file_name_gamewith_net_ok() -> Result<(), String> {
    // Setup
    let url: &str = "https://gamewith.net/palworld/43489";

    // Exercise
    let output: String = generate_file_name(url);

    // Verify
    let expected: &str = "gamewith.net_palworld_43489.html";
    assert_eq!(expected, output);

    Ok(())
}
