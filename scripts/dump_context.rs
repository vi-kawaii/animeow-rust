//! ```cargo
//! [dependencies]
//! walkdir = "2"
//! arboard = "3"
//! ```

use arboard::Clipboard;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // Расширения, которые включаем (можно дополнить)
    let include_ext = [
        "rs", "toml", "md", "json", "yaml", "yml",
        "js", "ts", "tsx", "jsx", "html", "css", "sql", "sh",
    ];

    // Папки, которые пропускаем
    let skip_dirs = ["target", ".git", "node_modules", "dist", "build", ".idea"];

    let mut output = String::new();

    // 1. Дерево проекта
    output.push_str("=== PROJECT TREE ===\n");
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !skip_dirs.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
    {
        let depth = entry.depth();
        let name = entry.file_name().to_string_lossy();
        let prefix = "  ".repeat(depth);
        output.push_str(&format!("{}{}\n", prefix, name));
    }

    // 2. Содержимое файлов
    output.push_str("\n=== FILE CONTENTS ===\n");
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !skip_dirs.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        if !include_ext.contains(&ext) {
            continue;
        }

        let display_path = path.strip_prefix("./").unwrap_or(path);
        output.push_str(&format!("\n── {} ──\n", display_path.display()));

        match fs::read_to_string(path) {
            Ok(content) => output.push_str(&content),
            Err(_) => output.push_str("<не удалось прочитать файл>\n"),
        }
        output.push('\n');
    }

    // 3. Копируем в буфер
    match Clipboard::new() {
        Ok(mut clipboard) => match clipboard.set_text(output.clone()) {
            Ok(_) => {
                let size_kb = output.len() / 1024;
                println!("✅ Контекст скопирован в буфер ({} KB)", size_kb);
                println!("   Файлов включено, дерево и содержимое готово к вставке.");
            }
            Err(e) => eprintln!("❌ Не могу записать в буфер: {}", e),
        },
        Err(e) => eprintln!("❌ Нет доступа к буферу: {}", e),
    }
}
