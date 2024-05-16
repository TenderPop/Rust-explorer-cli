use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use cursive::{
    align::Align,
    event::{Event, Key},
    theme::{Color, Palette, Theme},
    traits::*,
    views::{Button, Dialog, ListView, TextArea, TextContent, TextView},
    Cursive, XY,
};
use spinoff::{spinners, Spinner};

struct ExplorerPage<'a> {
    /// Creates Page which lists contents of following Path
    path: &'a PathBuf,
    /// required to create page
    runnable: &'a mut Cursive,
}
impl ExplorerPage<'_> {
    fn new(&mut self) {
        let path = self.path.clone();
        let a = fs::read_dir(path);

        let mut directories: Vec<fs::DirEntry> = Vec::new();

        let mut files: Vec<fs::DirEntry> = Vec::new();

        self.runnable
            .set_global_callback(cursive::event::Event::Key(Key::Backspace), |a| go_back(a));
        self.runnable
            .set_global_callback(cursive::event::Event::Char('h'), |a| help_screen(a));
        self.runnable
            .set_global_callback(cursive::event::Event::Char('s'), |a| search_screen(a));
        self.runnable
            .set_global_callback(cursive::event::Event::Char('q'), |a| a.quit());

        a.unwrap()
            .into_iter()
            .for_each(|entry| match entry.unwrap() {
                entry if entry.file_type().unwrap().is_dir() => directories.push(entry),
                entry if entry.file_type().unwrap().is_file() => files.push(entry),
                _ => {}
            });

        let len_files = files.len();
        let len_dir = directories.len();
        dbg!("dirs: {} files: {}", len_dir, len_files);

        self.runnable.add_layer(
            Dialog::new()
                .title("Your Directories")
                .button("Exit", |a| a.quit())
                .button("Help", |a| help_screen(a))
                .button("Search", |a| search_screen(a))
                .content(
                    ListView::new()
                        .with(|list: &mut ListView| {
                            for entry in directories {
                                // match list.on_event(cursive::event::Event::Char('d')) {
                                //     cursive::event::EventResult::Ignored => (),
                                //     cursive::event::EventResult::Consumed(_) => {
                                //         user_create_new_dir(&entry.path(), list)
                                //     }
                                // }

                                list.add_child(
                                    return_file_type_as_str(&entry),
                                    Button::new(
                                        format!("{}", entry.path().display()),
                                        move |mut action| {
                                            ExplorerPage {
                                                path: &entry.path(),
                                                runnable: &mut action,
                                            }
                                            .new()
                                        },
                                    ),
                                );
                            }
                        })
                        .with(|list| {
                            for entry in files {
                                // match list.on_event(cursive::event::Event::Char('d')) {
                                //     cursive::event::EventResult::Ignored => (),
                                //     cursive::event::EventResult::Consumed(_) => {
                                //         user_create_new_dir(&entry.path(), list)
                                //     }
                                // }

                                list.add_child(
                                    return_file_type_as_str(&entry),
                                    Button::new(
                                        format!("{}", entry.path().display()),
                                        move |action| action.quit(),
                                    ),
                                )
                            }
                            if self.runnable.screen().layer_sizes().len() >= 1 {
                                list.add_child("", Button::new("- Go Back -", |s| go_back(s)))
                            }
                        })
                        .scrollable()
                        .scroll_strategy(cursive::view::ScrollStrategy::StickToTop)
                        // .fixed_size(XY {
                        //     x: 150,
                        //     y: (len_dir + len_files),
                        // }),
                        .fixed_size(XY { x: 150, y: 140 }),
                ),
        );
    }
}

fn _user_create_new_dir(path: &PathBuf, s: &mut Cursive) {
    let mut user_input: String = String::new();
    let text_area = TextArea::new().with_name("text_area");

    s.add_layer(
        Dialog::new()
            .title(format!(
                "Create new Dir at {:?}",
                path.as_path().components().last().unwrap().as_os_str(),
            ))
            .content(text_area)
            .fixed_size(XY { x: 10, y: 20 }),
    );

    // match s.on_event(cursive::event::Event::Key(cursive::event::Key::Enter)) {
    //     cursive::event::EventResult::Ignored => (),
    //     cursive::event::EventResult::Consumed(_) => s
    //         .call_on_name("text_area", |view: &mut views::TextArea| {
    //             user_input.push_str(view.get_content())
    //         })
    //         .unwrap(),
    // }

    s.set_global_callback(Event::Key(Key::Enter), move |s| {
        s.call_on_name("text_area", |view: &mut TextArea| {
            user_input.push_str(view.get_content())
        })
        .unwrap();
    });
}

fn search_screen(s: &mut Cursive) {
    let text_area = TextArea::new();
    let user_input = TextContent::new(text_area.get_content());

    s.add_layer(
        Dialog::new()
            .title("Search")
            .content(text_area)
            .button(format!("{:?}", user_input.get_content().source()), |s| {
                s.quit()
            }),
    );
}

fn help_screen(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .button("Exit", |s| go_back(s))
            .title("Help:")
            .content(TextView::new(format!("{}", help_dialog())).align(Align::center_left()))
            .fixed_size(XY { x: 40, y: 15 }),
    );
}

fn go_back(s: &mut Cursive) {
    if s.screen_mut().layer_sizes().len() == 1 {
        return;
    } else {
        s.pop_layer();
    }
}

fn help_dialog() -> String {
    format!("Press <> to create new File \nPress <> to create new Folder \nPress <s> to Search \nPress <q> to exit program \nPress <h> to open Help")
}

pub(crate) fn return_file_type_as_str(entry: &DirEntry) -> &'static str {
    match entry {
        entry if entry.file_type().unwrap().is_file() => "--- File",
        entry if entry.file_type().unwrap().is_dir() => "--- Directory",
        entry if entry.file_type().unwrap().is_symlink() => "--- Symlink",
        _ => "--- Executable",
    }
}

fn main() {
    let path = Path::new("C:/Users/stoic_fqjp124").to_path_buf();
    let a = fs::read_dir(path.clone());
    let mut entries = Vec::new();
    let mut siv = cursive::default();
    let mut my_pallete = Palette::terminal_default();

    a.unwrap()
        .into_iter()
        .for_each(|entry| entries.push(entry.unwrap()));

    my_pallete.set_color("View", cursive::theme::Color::Rgb(27, 36, 44));
    my_pallete.set_color("Primary", cursive::theme::Color::Rgb(125, 189, 136));
    my_pallete.set_color("TitlePrimary", cursive::theme::Color::Rgb(221, 255, 188));
    my_pallete.set_color("Highlight", Color::Rgb(254, 255, 222));
    my_pallete.set_color("HighlightInactive", Color::Rgb(248, 250, 190));

    siv.set_theme(Theme {
        shadow: true,
        borders: cursive::theme::BorderStyle::Simple,
        palette: my_pallete,
    });

    ExplorerPage {
        path: &path,
        runnable: &mut siv,
    }
    .new();

    let mut spinner = Spinner::new(
        spinners::Balloon,
        "Opening Directories..",
        spinoff::Color::Green,
    );
    thread::sleep(Duration::from_secs(3));
    spinner.success("Opening");

    siv.run();
}
