// Copyright (c) 2025, 2026 Konstantin Adamov. Licensed under MIT.

use gettextrs::{bind_textdomain_codeset, bindtextdomain, setlocale, textdomain, LocaleCategory};
use std::path::PathBuf;

const GETTEXT_PACKAGE: &str = "loremgenerator";

pub fn init_i18n() {
    setlocale(LocaleCategory::LcAll, "");
    
    // Try to find locale directory in various locations
    let locale_dir = find_locale_dir();
    
    if let Some(dir) = locale_dir {
        bindtextdomain(GETTEXT_PACKAGE, dir).expect("Failed to bind text domain");
    } else {
        // Fallback to system default
        bindtextdomain(GETTEXT_PACKAGE, "/usr/share/locale").ok();
    }
    
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Failed to set codeset");
    textdomain(GETTEXT_PACKAGE).expect("Failed to set text domain");
}

fn find_locale_dir() -> Option<PathBuf> {
    // 1. Check SNAP environment
    if let Ok(snap_path) = std::env::var("SNAP") {
        let snap_locale = PathBuf::from(&snap_path).join("assets/locale");
        if snap_locale.exists() {
            return Some(snap_locale);
        }
    }
    
    // 2. Check relative to executable (for development and portable installs)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // Next to executable in assets folder
            let assets_locale = exe_dir.join("assets/locale");
            if assets_locale.exists() {
                return Some(assets_locale);
            }
            
            // Standard Linux install: ../share/locale
            if let Some(prefix) = exe_dir.parent() {
                let system_locale = prefix.join("share/locale");
                if system_locale.exists() {
                    return Some(system_locale);
                }
            }
        }
    }
    
    // 3. Check local development directory
    let dev_locale = PathBuf::from("assets/locale");
    if dev_locale.exists() {
        return Some(dev_locale);
    }
    
    None
}

// Macro for marking translatable strings
#[macro_export]
macro_rules! tr {
    ($text:expr) => {
        gettextrs::gettext($text)
    };
}
