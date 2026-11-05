use std::io::Write;
use tempfile::NamedTempFile;
use tam_agent::logs::tailer::LogTailer;

#[test]
fn test_log_tailer_reads_latest_lines() {
    let mut file = NamedTempFile::new().expect("create temp file");
    for i in 1..=20 {
        writeln!(file, "Log line number {}", i).expect("write line");
    }

    let tailer = LogTailer::new(file.path().to_path_buf(), 5);
    let lines = tailer.read_latest_lines().expect("read latest lines");

    assert_eq!(lines.len(), 5);
    assert_eq!(lines[0], "Log line number 16");
    assert_eq!(lines[4], "Log line number 20");
}

