use reqwest::header::COOKIE;
use std::fs;
use std::path::Path;

pub fn get_input(year: u16, day: u8) -> String {
    let file_path = format!("inputs/{}/day{}.txt", year, day);
    let path = Path::new(&file_path);

    if path.exists() {
        return fs::read_to_string(path)
            .expect("\n\x1b[91mFailed to read cached input file.\n\x1b[0m");
    }

    println!(
        "\n\x1b[93mDownloading input for Year-{} Day-{} ...\n\n\x1b[0m",
        year, day
    );

    let session = fs::read_to_string("cookie.txt").expect("\n\x1b[91mFailed to read cookie.txt file.\nif it does not exist,\nPlease create it in your CWD\nAnd put you session cookie string inside it\x1b[0m\n\n");
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header(COOKIE, format!("session={}", session.trim()))
        .send()
        .expect("\n\x1b[91mFailed to send request\n\n\x1b[0m")
        .error_for_status()
        .expect("\n\x1b[91mHTTP request failed. Is your session cookie vaild?\n\n\x1b[0m");

    let input = response
        .text()
        .expect("\n\x1b[91mFailed to read response text\n\n\x1b[0m");

    // 3. Save it to the cache folder for next time
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("\n\x1b[91mFailed to create inputs directory\n\n\x1b[0m");
    }
    fs::write(path, &input).expect("\n\x1b[91mFailed to write input to cache\n\n\x1b[0m");

    input
}
