use v2m3u8::transcoder::Transcoder;

#[test]
fn it_can_transcode() {
    let transcoder = Transcoder::new("/tmp/mp4/111.mp4".to_string(), "/tmp/m3u8".to_string());

    let result = if let Ok(_output) = transcoder.to_m3u8() {
        true
    } else {
        false
    };

    assert!(result);
}
