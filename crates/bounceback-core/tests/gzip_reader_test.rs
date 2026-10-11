use std::path::PathBuf;
use bounceback_core::AbletonGzipReader;

fn get_fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push("ableton");
    path.push(name);
    if !path.exists() {
        let mut root_path = PathBuf::from("tests/fixtures/ableton");
        root_path.push(name);
        return root_path;
    }
    path
}

#[test]
fn test_gzip_reader_extracts_valid_xml() {
    let path = get_fixture_path("minimal_v11.als");
    let xml = AbletonGzipReader::read_to_string(&path).expect("Falha na descompressão do Gzip");

    assert!(xml.contains("<?xml"), "O arquivo descompactado deve iniciar com header XML");
    assert!(xml.contains("<Ableton"), "O arquivo deve conter o nó raiz <Ableton>");
    assert!(xml.contains("<LiveSet>"), "O arquivo deve conter o elemento <LiveSet>");
    assert!(xml.contains("Audio 1"), "O arquivo deve conter o nome do canal");
}
