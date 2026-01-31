use adw::ToastOverlay;
use gtk4::prelude::*;
use gtk4::{Align, IconTheme, PolicyType, TextBuffer, TextView, glib};
use libadwaita as adw;
use relm4::actions::RelmActionGroup;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use clap::Parser;

mod cli;
mod helpers;

use cli::CliArgs;
use helpers::actions::{AboutAction, WindowActionGroup, create_about_action};
use helpers::generator::generate;
use helpers::number_editor::{CounterOutput, NumberEditor};
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
    max_words_widget: Controller<NumberEditor>,
    max_sentences_widget: Controller<NumberEditor>,
    max_paragraphs_widget: Controller<NumberEditor>,
    toast_overlay: Option<ToastOverlay>,
    text_view: Option<TextView>,
}

#[derive(Debug)]
enum Messages {
    Generate,
    UpdateMaxWords(usize),
    UpdateMaxSentences(usize),
    UpdateParagraphs(usize),
    ToggleStartWithLorem(bool),
    CopyToClipboard,
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
                "_About" => AboutAction,
            }
        }
    }

    view! {
        #[root]
        main_window = adw::ApplicationWindow {
            set_visible: true,
            set_title: Some(APP_NAME),
            set_default_size: (800, 600),

            #[name = "toast_overlay"]
            adw::ToastOverlay {

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            set_menu_model: Some(&main_menu),
                            set_direction: gtk::ArrowType::Down,
                            set_can_focus: false,
                        }
                    },

                    gtk::Box{
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: SPACING_MEDIUM,
                        set_margin_horizontal: SPACING_MEDIUM,
                        set_margin_top: SPACING_MEDIUM,

                        model.max_words_widget.widget(),
                        model.max_sentences_widget.widget(),
                        model.max_paragraphs_widget.widget(),

                        gtk::Box{
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 5,
                            set_halign: Align::Center,
                            gtk::Label{
                                set_label: "Start with 'Lorem ipsum'",
                            },
                            gtk::Switch{
                                set_active: model.start_with_lorem,
                                set_halign: Align::Center,
                                connect_state_set[sender] => move |_, state| {
                                    let _ = sender.input(Messages::ToggleStartWithLorem(state));
                                    glib::Propagation::Proceed
                                },
                            }
                        },


                    },
                    gtk::Box{
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_horizontal: SPACING_MEDIUM,
                        set_margin_top: SPACING_MEDIUM,

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
                                set_vexpand: true,
                                add_css_class: "card",
                                #[watch]
                                set_buffer: Some(&TextBuffer::builder()
                                    .text(model.result_text.as_str())
                                    .build()
                                ),
                            },
                        },
                    },

                    gtk::Box{
                        set_halign: Align::Start,
                        set_spacing: SPACING_LARGE,
                        set_margin_horizontal: SPACING_MEDIUM,
                        set_margin_vertical: SPACING_MEDIUM,
                        gtk::Button::with_mnemonic("_Generate") {
                            set_halign: Align::Start,
                            connect_clicked[sender] => move |_| {
                                sender.input(Messages::Generate);
                            },
                        },
                        gtk::Button::with_mnemonic("_Copy to Clipboard") {
                            #[watch]
                            set_sensitive: !model.result_text.is_empty(),
                            set_halign: Align::Start,
                            connect_clicked[sender] => move|_| {
                                sender.input(Messages::CopyToClipboard);
                            },
                        },
                    },
                }
            }
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
            start_with_lorem: settings.start_with_lorem,
            toast_overlay: None,
            text_view: None,
            max_words_widget: NumberEditor::builder()
                .launch(("Max Words".to_string(), 1, 100, settings.max_words))
                .forward(sender.input_sender(), |output| match output {
                    CounterOutput::ValueChanged(value) => Messages::UpdateMaxWords(value),
                }),
            max_sentences_widget: NumberEditor::builder()
                .launch(("Max Sentences".to_string(), 1, 20, settings.max_sentences))
                .forward(sender.input_sender(), |output| match output {
                    CounterOutput::ValueChanged(value) => Messages::UpdateMaxSentences(value),
                }),
            max_paragraphs_widget: NumberEditor::builder()
                .launch(("Paragraphs".to_string(), 1, 50, settings.paragraphs))
                .forward(sender.input_sender(), |output| match output {
                    CounterOutput::ValueChanged(value) => Messages::UpdateParagraphs(value),
                }),
        };

        let widgets = view_output!();

        model.toast_overlay = Some(widgets.toast_overlay.clone());
        model.text_view = Some(widgets.result_text_view.clone());

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
                    .add_toast(adw::Toast::new("Copied to clipboard"));
            }
        }
    }
}

fn main() {
    let args = CliArgs::parse();

    // Check if any CLI arguments were provided
    let is_cli_mode = args.paragraphs.is_some()
        || args.max_sentences.is_some()
        || args.max_words.is_some();

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
        adw::init().expect("Failed to initialize Libadwaita");

        gtk4::init().expect("Failed to initialize GTK");

        let gtk_app = adw::Application::builder()
            .application_id(APP_ID)
            .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
            .build();

        gtk_app.connect_activate(|_| {
        let display = gtk4::gdk::Display::default().expect("Could not get default display.");
        let icon_theme = IconTheme::for_display(&display);
        if let Ok(snap_path) = std::env::var("SNAP") {
            let assets_path = std::path::Path::new(&snap_path).join("assets");
            icon_theme.add_search_path(assets_path);
        } else {
            // Fallback for local development
            icon_theme.add_search_path("assets");

            // Check paths relative to the executable
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    // 1. Assets next to executable (e.g. portable tarball)
                    let local_assets = exe_dir.join("assets");
                    if local_assets.exists() {
                        icon_theme.add_search_path(local_assets);
                    }

                    // 2. Standard Linux install: ../share/loremgenerator/assets
                    // (assuming binary is in /usr/bin or /usr/local/bin)
                    if let Some(prefix) = exe_dir.parent() {
                        let system_assets =
                            prefix.join("share").join("loremgenerator").join("assets");
                        if system_assets.exists() {
                            icon_theme.add_search_path(system_assets);
                        }
                    }
                }
            }
        }
    });

        let app = RelmApp::from_app(gtk_app);
        app.run::<App>(());
    }
}
