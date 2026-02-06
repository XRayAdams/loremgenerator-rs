// Copyright (c) 2025, 2026 Konstantin Adamov. Licensed under MIT.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Rerun this script if assets change
    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=po");

    let out_dir = env::var("OUT_DIR").unwrap();
    // Navigate up to the target directory (target/debug or target/release)
    // Structure is usually: target/<profile>/build/<package>-<hash>/out
    // So we go up 3 levels to get to target/<profile>
    let dest_path = Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap();
        
    let src = Path::new("assets");
    let dest = dest_path.join("assets");

    if src.exists() {
        copy_dir_all(src, &dest).expect("Failed to copy assets");
    }
    
    // Compile translations
    compile_translations(dest_path);
}

fn compile_translations(dest_path: &Path) {
    let po_dir = Path::new("po");
    if !po_dir.exists() {
        return;
    }
    
    // Find all .po files
    if let Ok(entries) = fs::read_dir(po_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("po") {
                    if let Some(lang) = path.file_stem().and_then(|s| s.to_str()) {
                        // Skip the template file
                        if lang.ends_with(".pot") || lang == "loremgenerator" {
                            continue;
                        }
                        
                        compile_po_file(&path, lang, dest_path);
                    }
                }
            }
        }
    }
}

fn compile_po_file(po_file: &Path, lang: &str, dest_path: &Path) {
    let locale_dir = dest_path.join("assets/locale").join(lang).join("LC_MESSAGES");
    fs::create_dir_all(&locale_dir).expect("Failed to create locale directory");
    
    let mo_file = locale_dir.join("loremgenerator.mo");
    
    println!("Compiling {} translation: {:?} -> {:?}", lang, po_file, mo_file);
    
    // Try to compile using msgfmt
    let status = Command::new("msgfmt")
        .arg("-o")
        .arg(&mo_file)
        .arg(po_file)
        .status();
    
    match status {
        Ok(status) if status.success() => {
            println!("Successfully compiled {} translation", lang);
        }
        Ok(status) => {
            eprintln!("Warning: msgfmt failed with status: {}", status);
            eprintln!("Translation for {} will not be available", lang);
        }
        Err(e) => {
            eprintln!("Warning: Could not run msgfmt: {}", e);
            eprintln!("Please install gettext tools (msgfmt) to enable translations");
            eprintln!("On Ubuntu/Debian: sudo apt install gettext");
            eprintln!("On Fedora: sudo dnf install gettext");
        }
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
