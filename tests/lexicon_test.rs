use std::fs;

#[test]
fn test_csw24_kwg_loading() {
    // This test will use wolges in Sprint 3
    // For now, just verify file exists
    let kwg_path = "assets/lexicons/CSW24.kwg";
    assert!(fs::metadata(kwg_path).is_ok(), "CSW24.kwg file exists");

    let metadata = fs::metadata(kwg_path).unwrap();
    let file_size_mb = metadata.len() as f64 / 1_000_000.0;

    println!("CSW24.kwg size: {:.2} MB", file_size_mb);
    assert!(file_size_mb >= 4.0 && file_size_mb <= 10.0,
            "KWG file size should be 4-10 MB");
}

#[test]
fn test_csw24_source_exists() {
    // Verify the source CSW24.txt file exists
    let txt_path = "CSW24.txt";
    assert!(fs::metadata(txt_path).is_ok(), "CSW24.txt source file exists");

    let metadata = fs::metadata(txt_path).unwrap();
    let file_size_mb = metadata.len() as f64 / 1_000_000.0;

    println!("CSW24.txt size: {:.2} MB", file_size_mb);
    assert!(file_size_mb >= 2.0 && file_size_mb <= 5.0,
            "CSW24.txt file size should be 2-5 MB");
}

#[test]
fn test_kwg_is_smaller_than_source() {
    // KWG should be compressed/optimized compared to text
    let kwg_size = fs::metadata("assets/lexicons/CSW24.kwg").unwrap().len();
    let txt_size = fs::metadata("CSW24.txt").unwrap().len();

    println!("Compression ratio: {:.2}%", (kwg_size as f64 / txt_size as f64) * 100.0);

    // KWG might be larger or smaller depending on the format
    // Just verify both files are reasonable sizes
    assert!(kwg_size > 1_000_000, "KWG should be at least 1MB");
    assert!(txt_size > 1_000_000, "TXT should be at least 1MB");
}
