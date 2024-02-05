use crate::{
    git,
    items::{self, Item},
    util, Config, Res,
};
use ansi_to_tui::IntoText;
use std::iter;

use super::Screen;

pub(crate) fn create(config: &Config, args: Vec<String>) -> Res<Screen> {
    let path_buf = config.dir.clone();
    Screen::new(Box::new(move || {
        let str_args = util::str_vec(&args);
        let summary = git::show_summary(&path_buf, &str_args)?;
        let show = git::show(&path_buf, &str_args)?;

        Ok(iter::once(Item {
            display: summary.into_text()?,
            ..Default::default()
        })
        .chain(items::create_diff_items(&show, &0))
        .collect())
    }))
}
