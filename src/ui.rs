use cli_table::{format::Justify, Cell, CellStruct, Color, Style, Table, TableDisplay};
use indicatif::ProgressBar;

use crate::transcoder::Transcoder;

pub struct Ui<'a> {
    transcoders: &'a Vec<Transcoder>,
    pub progress_bar: ProgressBar,
}

impl<'a> Ui<'a> {
    pub fn from(transcoders: &'a Vec<Transcoder>) -> Self {
        Self {
            transcoders,
            progress_bar: ProgressBar::new(transcoders.len() as u64),
        }
    }

    pub fn render_as_table(&self) -> Result<TableDisplay, std::io::Error> {
        self.create_table_rows()
            .table()
            .title(vec!["Id".cell(), "Output file".cell(), "Progress".cell()])
            .bold(true)
            .display()
    }

    pub(crate) fn get_progress_color(&self, progress: u32) -> Option<Color> {
        if progress == 100 {
            return Some(Color::Magenta);
        }
        match progress.cmp(&0) {
            std::cmp::Ordering::Equal => Some(Color::Yellow),
            _ => Some(Color::Green),
        }
    }

    pub(crate) fn create_table_rows(&self) -> Vec<Vec<CellStruct>> {
        self.transcoders
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (idx, transcoder)| {
                acc.push(vec![
                    (idx + 1).cell().foreground_color(Some(Color::Cyan)),
                    transcoder
                        .output_file
                        .clone()
                        .cell()
                        .foreground_color(Some(Color::Rgb(163, 148, 128))),
                    format!("{}%", transcoder.progress)
                        .cell()
                        .foreground_color(self.get_progress_color(transcoder.progress))
                        .justify(Justify::Right),
                ]);
                acc
            })
    }
}
