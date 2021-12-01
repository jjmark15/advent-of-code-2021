use std::path::PathBuf;

pub(crate) fn sample_data_file_name(day: u8) -> String {
    format!("day_{:02}.txt", day)
}

pub(crate) fn assert_challenge_result(day: u8, part: u8, result: &str) {
    let sample_data_path: PathBuf = ["sample_data", sample_data_file_name(day).as_str()]
        .iter()
        .collect();

    let mut cmd = assert_cmd::Command::cargo_bin("advent-of-code-2021").unwrap();

    cmd.args(&["-d", day.to_string().as_str()])
        .args(&["-p", part.to_string().as_str()])
        .args(&["-i", sample_data_path.as_os_str().to_str().unwrap()]);

    cmd.assert().success().stdout(format!("{}\n", result));
}
