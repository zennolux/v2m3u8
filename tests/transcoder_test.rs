use std::thread;

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

#[test]
fn it_can_listen_progress() {
    let mut transcoder =
        Transcoder::new("/tmp/mp4/daolang.mp4".to_string(), "/tmp/m3u8".to_string());

    let data = transcoder.clone();
    let handler = thread::spawn(move || data.to_m3u8().unwrap());

    let listener = thread::spawn(move || {
        transcoder.listen_progress(|this| {
            println!("{}", this.progress);
        });
    });

    handler.join().unwrap();
    listener.join().unwrap();
}
