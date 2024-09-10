use std::{env, fs, thread};

use indicatif::{ProgressBar, ProgressStyle};
use v2m3u8::transcoder::Transcoder;

fn main() {
    if let Some(input_path) = env::args().into_iter().last() {
        if let Ok(reader) = fs::read_dir(input_path) {
            let mut transcoders = reader.fold(vec![], |mut acc, item| {
                if let Ok(entry) = item {
                    let transcoder = Transcoder::new(entry.path().to_str().unwrap().to_owned());
                    acc.push(transcoder);
                }
                acc
            });

            let data = transcoders.clone();
            let handler = thread::spawn(move || {
                data.iter().for_each(|transcoder| {
                    transcoder.to_m3u8().expect("Transcode error");
                })
            });

            let mut data = transcoders.clone();
            let listener = thread::spawn(move || {
                let progress_bar = ProgressBar::new(data.len() as u64);
                println!("Transcoding in progress, this may take a moment.");
                data.iter_mut().enumerate().for_each(|(idx, transcoder)| {
                    transcoder.listen_progress(|transcoder| {
                        transcoders[idx].progress = transcoder.progress;
                        if transcoder.progress == 100 {
                            progress_bar.inc(1);
                        }
                    });
                });
                progress_bar.set_style(ProgressStyle::with_template("{msg}").unwrap());
                progress_bar.finish_with_message("Done");
            });

            handler.join().unwrap();
            listener.join().unwrap();
        }
    }
}
