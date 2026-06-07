// Copyright (c) 2025, 2026 Konstantin Adamov. Licensed under MIT.

use adw::ToastOverlay;
use adw::prelude::*;
use clap::Parser;
use gtk4::prelude::*;
use gtk4::{Align, IconTheme, PolicyType, TextBuffer, TextView, gio, glib};
use libadwaita as adw;
use relm4::actions::RelmActionGroup;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

mod cli;
mod helpers;
mod i18n;

use cli::CliArgs;
use helpers::actions::{AboutAction, WindowActionGroup, create_about_action};
use helpers::generator::generate;
use helpers::number_editor::NumberEditor;
use helpers::static_data::{APP_ID, APP_NAME};

const SPACING_MEDIUM: i32 = 12;
const SPACING_LARGE: i32 = 18;

#[derive(Serialize, Deserialize)]
struct AppSettings {
    max_words: usize,
    max_sentences: usize,
    paragraphs: usize,
    start_with_lorem: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            max_words: 15,
            max_sentences: 4,
            paragraphs: 5,
            start_with_lorem: true,
        }
    }
}
struct App {
    max_words: usize,
    max_sentences: usize,
    paragraphs: usize,
    start_with_lorem: bool,
    result_text: String,
    word_count: usize,
    char_count_with_spaces: usize,
    char_count_no_spaces: usize,
    sentence_count: usize,
    toast_overlay: Option<ToastOverlay>,
    text_view: Option<TextView>,
    is_collapsed: bool,
    window: Option<adw::ApplicationWindow>,
}

#[derive(Debug)]
enum Messages {
    Generate,
    UpdateMaxWords(usize),
    UpdateMaxSentences(usize),
    UpdateParagraphs(usize),
    ToggleStartWithLorem(bool),
    CopyToClipboard,
    SetCollapsed(bool),
    SaveToFile,
}

impl App {
    fn get_config_path() -> PathBuf {
        let mut path = gtk4::glib::user_config_dir();
        path.push("loremgenerator");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    fn save_config(&self) {
        let settings = AppSettings {
            max_words: self.max_words,
            max_sentences: self.max_sentences,
            paragraphs: self.paragraphs,
            start_with_lorem: self.start_with_lorem,
        };
        if let Ok(content) = serde_json::to_string_pretty(&settings) {
            let path = Self::get_config_path();
            let _ = fs::write(path, content);
        }
    }

    fn load_config() -> AppSettings {
        let path = Self::get_config_path();
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                return settings;
            }
        }
        AppSettings::default()
    }

    fn get_app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Messages;
    type Output = ();

    menu! {
        main_menu: {
            section! {
                &tr!("_About") => AboutAction,
            }
        }
    }

    view! {
        #[root]
        main_window = adw::ApplicationWindow {
            set_visible: true,
            set_title: Some(APP_NAME),
            set_default_size: (900, 600),

            #[name = "toast_overlay"]
            adw::ToastOverlay {

                #[name = "split_view"]
                adw::OverlaySplitView {
                    connect_collapsed_notify[sender] => move |sv| {
                        sender.input(Messages::SetCollapsed(sv.is_collapsed()));
                    },
                    set_max_sidebar_width: 180.0,

                    #[wrap(Some)]
                    set_sidebar = &adw::NavigationPage {
                        set_title: &tr!("Settings"),
                        #[wrap(Some)]
                        set_child = &adw::ToolbarView {
                            add_top_bar = &adw::HeaderBar { 
                                
                            },

                            #[wrap(Some)]
                            set_content = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: SPACING_MEDIUM,
                                set_margin_bottom: SPACING_MEDIUM,
                                set_margin_start: SPACING_MEDIUM,
                                set_margin_end: SPACING_MEDIUM,

                                NumberEditor {
                                    set_label: &tr!("Max Words"),
                                    set_min: 6.0,
                                    set_max: 100.0,
                                    set_value: model.max_words as f64,
                                    connect_value_changed[sender] => move |_, value| {
                                        sender.input(Messages::UpdateMaxWords(value as usize));
                                    },
                                },

                                NumberEditor {
                                    set_label: &tr!("Max Sentences"),
                                    set_min: 1.0,
                                    set_max: 20.0,
                                    set_value: model.max_sentences as f64,
                                    connect_value_changed[sender] => move |_, value| {
                                        sender.input(Messages::UpdateMaxSentences(value as usize));
                                    },
                                },

                                NumberEditor {
                                    set_label: &tr!("Paragraphs"),
                                    set_min: 1.0,
                                    set_max: 50.0,
                                    set_value: model.paragraphs as f64,
                                    connect_value_changed[sender] => move |_, value| {
                                        sender.input(Messages::UpdateParagraphs(value as usize));
                                    },
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_spacing: 5,
                                    set_halign: Align::Start,
                                    set_margin_top: 4,
                                    set_margin_bottom: 4,
                                    gtk::Label {
                                        set_label: &tr!("Start with 'Lorem ipsum'"),
                                    },
                                    gtk::Switch {
                                        set_active: model.start_with_lorem,
                                        set_halign: Align::Start,
                                        connect_state_set[sender] => move |_, state| {
                                            let _ = sender.input(Messages::ToggleStartWithLorem(state));
                                            glib::Propagation::Proceed
                                        },
                                    }
                                },
                            }
                        }
                    },

                    #[wrap(Some)]
                    set_content = &adw::NavigationPage {
                        set_title: APP_NAME,

                        #[wrap(Some)]
                        set_child = &adw::ToolbarView {
                            add_top_bar = &adw::HeaderBar {
                                pack_start = &gtk4::Button {
                                    set_icon_name: "sidebar-show-symbolic",
                                    set_can_focus: false,
                                    #[watch]
                                    set_visible: model.is_collapsed,
                                    connect_clicked[split_view] => move |_| {
                                        split_view.set_show_sidebar(true);
                                    },
                                },
                                pack_end = &gtk::MenuButton {
                                    set_icon_name: "open-menu-symbolic",
                                    set_menu_model: Some(&main_menu),
                                    set_direction: gtk::ArrowType::Down,
                                    set_can_focus: false,
                                }
                            },

                            #[wrap(Some)]
                            set_content = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_vexpand: true,

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_horizontal: SPACING_MEDIUM,
                                    set_vexpand: true,

                                    gtk::ScrolledWindow {
                                        set_hscrollbar_policy: PolicyType::Never,
                                        set_vexpand: true,
                                        set_hexpand: true,

                                        #[name = "result_text_view"]
                                        gtk::TextView {
                                            set_editable: false,
                                            set_wrap_mode: gtk::WrapMode::Word,
                                            set_left_margin: SPACING_MEDIUM,
                                            set_right_margin: SPACING_MEDIUM,
                                            set_top_margin: SPACING_MEDIUM,
                                            set_bottom_margin: SPACING_MEDIUM,
                                            add_css_class: "card",
                                            #[watch]
                                            set_buffer: Some(&TextBuffer::builder()
                                                .text(model.result_text.as_str())
                                                .build()
                                            ),
                                        },
                                    },
                                },

                                gtk::Label {
                                set_halign: Align::Start,
                                set_margin_horizontal: SPACING_MEDIUM,
                                set_margin_top: SPACING_MEDIUM,
                                set_max_width_chars: 1,
                                set_wrap: true,
                                set_wrap_mode: gtk::pango::WrapMode::Word,
                                set_lines: 2,
                                #[watch]
                                set_label: &format!("{}: {}, {}: {}, {}: {}, {}: {}",
                                    tr!("Words"), model.word_count,
                                    tr!("Sentences"), model.sentence_count,
                                    tr!("Characters (with spaces)"), model.char_count_with_spaces,
                                    tr!("Characters (no spaces)"), model.char_count_no_spaces),
                                },

                                gtk::Box {
                                    set_halign: Align::Start,
                                    set_spacing: SPACING_LARGE,
                                    set_margin_horizontal: SPACING_MEDIUM,
                                    set_margin_vertical: SPACING_MEDIUM,
                                    gtk::Button::with_mnemonic(&tr!("_Generate")) {
                                        set_halign: Align::Start,
                                        connect_clicked[sender] => move |_| {
                                            sender.input(Messages::Generate);
                                        },
                                    },
                                    gtk::Button::with_mnemonic(&tr!("_Copy to Clipboard")) {
                                        #[watch]
                                        set_sensitive: !model.result_text.is_empty(),
                                        set_halign: Align::Start,
                                        connect_clicked[sender] => move|_| {
                                            sender.input(Messages::CopyToClipboard);
                                        },
                                    },
                                    gtk::Button::with_mnemonic(&tr!("_Save to File")) {
                                        #[watch]
                                        set_sensitive: !model.result_text.is_empty(),
                                        set_halign: Align::Start,
                                        connect_clicked[sender] => move|_| {
                                            sender.input(Messages::SaveToFile);
                                        },
                                    },
                                },
                            }
                        }
                    },

                },
            },
            add_breakpoint = bp_with_setters(
                    adw::Breakpoint::new(
                        adw::BreakpointCondition::new_length(
                            adw::BreakpointConditionLengthType::MaxWidth,
                            680.0,
                            adw::LengthUnit::Px,
                        )
                    ),
                    &[(&split_view, "collapsed", true)]
                ),
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = Self::load_config();

        let mut model = Self {
            max_words: settings.max_words,
            max_sentences: settings.max_sentences,
            paragraphs: settings.paragraphs,
            result_text: String::new(),
            word_count: 0,
            char_count_with_spaces: 0,
            char_count_no_spaces: 0,
            sentence_count: 0,
            start_with_lorem: settings.start_with_lorem,
            toast_overlay: None,
            window: None,
            text_view: None,
            is_collapsed: false,
        };

        let widgets = view_output!();

        model.toast_overlay = Some(widgets.toast_overlay.clone());
        model.text_view = Some(widgets.result_text_view.clone());
        model.window = Some(root.clone());

        let about_action =
            create_about_action(widgets.main_window.clone(), Self::get_app_version());

        let mut window_actions = RelmActionGroup::<WindowActionGroup>::new();
        window_actions.add_action(about_action);
        window_actions.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Messages::UpdateMaxWords(value) => {
                self.max_words = value;
                self.save_config();
            }
            Messages::UpdateMaxSentences(value) => {
                self.max_sentences = value;
                self.save_config();
            }
            Messages::UpdateParagraphs(value) => {
                self.paragraphs = value;
                self.save_config();
            }
            Messages::ToggleStartWithLorem(state) => {
                self.start_with_lorem = state;
                self.save_config();
            }
            Messages::Generate => {
                self.result_text = generate(
                    self.start_with_lorem,
                    self.paragraphs,
                    self.max_sentences,
                    self.max_words,
                );
                self.word_count = self.result_text.split_whitespace().count();
                self.char_count_with_spaces = self.result_text.chars().count();
                self.char_count_no_spaces = self
                    .result_text
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .count();
                self.sentence_count = self
                    .result_text
                    .chars()
                    .filter(|c| matches!(c, '.' | '!' | '?'))
                    .count();

                self.text_view
                    .clone()
                    .unwrap()
                    .buffer()
                    .set_text(self.result_text.as_str());
            }
            Messages::CopyToClipboard => {
                self.text_view
                    .clone()
                    .unwrap()
                    .clipboard()
                    .set_text(self.result_text.as_str());

                self.toast_overlay
                    .clone()
                    .unwrap()
                    .add_toast(adw::Toast::new(&tr!("Copied to clipboard")));
            }
            Messages::SetCollapsed(state) => {
                self.is_collapsed = state;
            }
            Messages::SaveToFile => {
                let window = self.window.clone();
                let toast_overlay = self.toast_overlay.clone();
                let result_text = self.result_text.clone();

                let dialog = gtk4::FileDialog::builder()
                    .initial_name("output.txt")
                    .build();

                dialog.save(window.as_ref(), gio::Cancellable::NONE, move |result| {
                    if let Ok(file) = result {
                        if let Some(path) = file.path() {
                            let _ = std::fs::write(&path, result_text.as_str());

                            toast_overlay
                                .clone()
                                .unwrap()
                                .add_toast(adw::Toast::new(&tr!("Saved to file")));
                        }
                    }
                });
                
            }
        }
    }
}

fn bp_with_setters(
    bp: adw::Breakpoint,
    additions: &[(&impl IsA<glib::Object>, &str, impl ToValue)],
) -> adw::Breakpoint {
    bp.add_setters(additions);
    bp
}

fn main() {
    let args = CliArgs::parse();

    // Check if any CLI arguments were provided
    let is_cli_mode =
        args.paragraphs.is_some() || args.max_sentences.is_some() || args.max_words.is_some();

    if is_cli_mode {
        // CLI mode - generate and print to stdout
        let defaults = AppSettings::default();

        let start_with_lorem = args.start_with_lorem == 1;
        let paragraphs = args.paragraphs.unwrap_or(defaults.paragraphs);
        let max_sentences = args.max_sentences.unwrap_or(defaults.max_sentences);
        let max_words = args.max_words.unwrap_or(defaults.max_words);

        let result = generate(start_with_lorem, paragraphs, max_sentences, max_words);
        println!("{}", result);
    } else {
        // GUI mode - launch the application
        i18n::init_i18n();

        adw::init().expect("Failed to initialize Libadwaita");

        gtk4::init().expect("Failed to initialize GTK");

        let gtk_app = adw::Application::builder()
            .application_id(APP_ID)
            .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
            .build();

        gtk_app.connect_activate(|_| {
            // Load and register the GResource
            let resources_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/resources.gresource"));
            let resource_data = gtk4::glib::Bytes::from_static(resources_bytes);
            let resource =
                gio::Resource::from_data(&resource_data).expect("Failed to load GResource");
            gio::resources_register(&resource);

            // Add the GResource path to icon theme so AboutDialog can find the icon
            let display = gtk4::gdk::Display::default().expect("Could not get default display.");
            let icon_theme = IconTheme::for_display(&display);
            icon_theme.add_resource_path("/app/rayadams/loremgenerator/assets");
        });

        let app = RelmApp::from_app(gtk_app);
        app.run::<App>(());
    }
}
