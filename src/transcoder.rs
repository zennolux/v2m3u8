use regex::Regex;

use std::{
    error::Error,
    fs,
    num::ParseFloatError,
    path::Path,
    process::{Command, Output, Stdio},
    thread::sleep,
    time::Duration,
};

#[derive(Debug, Clone)]
pub struct Transcoder {
    pub input_file: String,
    pub output_file: String,
    pub progress: u32,
    log_file: String,
}

impl Transcoder {
    pub fn new(input_file: String, output_path: String) -> Self {
        let file_name = Path::new(&input_file)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let suffix = file_name.split(".").last().unwrap();
        let file_name = file_name.replace(&format!(".{}", suffix), "");
        let file_path = format!("{}/{}", output_path, &file_name);
        let _ = fs::create_dir_all(&file_path);

        return Self {
            output_file: format!("{}/{}.m3u8", &file_path, file_name),
            log_file: format!("{}/{}.log", &file_path, file_name),
            progress: 0,
            input_file,
        };
    }

    pub fn to_m3u8(&self) -> Result<Output, std::io::Error> {
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&self.input_file)
            .arg("-codec:")
            .arg("copy")
            .arg("-start_number")
            .arg("0")
            .arg("-hls_time")
            .arg("0")
            .arg("-hls_list_size")
            .arg("0")
            .arg("-f")
            .arg("hls")
            .arg("-progress")
            .arg(&self.log_file)
            .arg(&self.output_file)
            .output()?;
        Ok(output)
    }

    pub(crate) fn get_duration(&self) -> Result<f64, Box<dyn Error>> {
        let duration = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(&self.input_file)
            .stdout(Stdio::piped())
            .output();

        if let Ok(dur) = duration {
            let duration = format!(
                "{:?}.00",
                String::from_utf8(dur.stdout)?
                    .trim()
                    .replace(".", "")
                    .parse::<usize>()?
            )
            .parse::<f64>()?;
            return Ok(duration);
        }
        Ok(0.0)
    }

    pub(crate) fn parse_progress_time<'a>(
        &'a self,
        content: &'a str,
    ) -> Result<f64, Box<dyn Error>> {
        let reg = Regex::new(r"out_time_ms=(\d+)").unwrap();

        if let Some(caps) = reg.captures_iter(&content).last() {
            let progress_time = format!(
                "{:?}.00",
                caps.get(0)
                    .unwrap()
                    .as_str()
                    .split("=")
                    .last()
                    .unwrap()
                    .parse::<usize>()?
            )
            .parse::<f64>()?;
            return Ok(progress_time);
        }
        Ok(0.0)
    }

    pub(crate) fn calc_progress(progress_time: f64, duration: f64) -> Result<f64, ParseFloatError> {
        if progress_time == 0.0 {
            return Ok(0.0);
        }

        let progress = format!("{:.2}", progress_time / duration).parse::<f64>()?;
        Ok(progress)
    }

    pub fn listen_progress<F>(&mut self, mut notifier: F)
    where
        F: FnMut(&Self),
    {
        match self.get_duration() {
            Ok(duration) => {
                let mut end = false;
                while !end {
                    let Ok(content) = fs::read_to_string(&self.log_file) else {
                        continue;
                    };
                    let Ok(progress_time) = self.parse_progress_time(&content) else {
                        break;
                    };
                    let Ok(mut percent) = Transcoder::calc_progress(progress_time, duration) else {
                        break;
                    };
                    if content.contains("progress=end") {
                        percent = 1.0;
                        end = true;
                    }

                    self.progress = (percent * 100.0) as u32;
                    notifier(&self);
                    sleep(Duration::from_millis(300));
                }
            }
            Err(err) => {
                panic!("An error happened while get duration: {}", err);
            }
        }
    }
}
