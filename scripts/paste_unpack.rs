//! ```cargo
//! [dependencies]
//! arboard = "3"
//! ```

use arboard::Clipboard;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

/// Промпт, который скрипт выводит в терминал при запуске.
const PROMPT: &str = r##"Отвечай СТРОГО в таком формате, без пояснений вне блоков:

FILE путь/к/файлу
====
<полное содержимое файла>
====
END FILE

Правила:
- Путь указывай относительно корня проекта (например, src/main.rs).
- Содержимое — полное, без сокращений и без "...".
- Никаких markdown-обёрток вокруг блоков.
- Если файл не менялся — не включай его.
- Если файл нужно удалить — напиши: DELETE путь/к/файлу
"##;

fn main() {
    // 1. Сразу печатаем промпт
    println!("──────────────────────────────────────────────");
    println!("ПРОМПТ (скопируй и вставь в DeepSeek):");
    println!("──────────────────────────────────────────────");
    println!("{}", PROMPT);
    println!("──────────────────────────────────────────────");
    println!();

    let mut clipboard = match Clipboard::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Нет доступа к буферу обмена: {}", e);
            return;
        }
    };

    // 2. Кладём промпт в буфер сразу
    if let Err(e) = clipboard.set_text(PROMPT.to_string()) {
        eprintln!("⚠️ Не удалось положить промпт в буфер: {}", e);
    } else {
        println!("✅ Промпт уже в буфере обмена — просто вставь в чат.");
    }

    println!();
    println!("Слушаю буфер обмена... Нажмите Ctrl+C для выхода.");
    println!("Скопируй ответ DeepSeek с блоками FILE ... END FILE");
    println!();

    // Инициализируем last_text промптом, чтобы скрипт не срабатывал на сам себя.
    let mut last_text = PROMPT.to_string();

    loop {
        if let Ok(text) = clipboard.get_text() {
            if text != last_text
                && (text.contains("FILE ") || text.contains("DELETE "))
                && text.contains("END FILE")
            {
                last_text = text.clone();
                match unpack_files(&text) {
                    Ok((written, deleted, skipped)) => {
                        println!();
                        println!("══════════════ РЕЗУЛЬТАТ ══════════════");
                        if !written.is_empty() {
                            println!("✅ Записано ({}):", written.len());
                            for f in &written {
                                println!("   • {}", f);
                            }
                        }
                        if !deleted.is_empty() {
                            println!("🗑 Удалено ({}):", deleted.len());
                            for f in &deleted {
                                println!("   • {}", f);
                            }
                        }
                        if !skipped.is_empty() {
                            println!("⚠️ Пропущено ({}):", skipped.len());
                            for f in &skipped {
                                println!("   • {}", f);
                            }
                        }
                        println!("═══════════════════════════════════════");
                        println!();
                    }
                    Err(e) => {
                        println!();
                        println!("❌ Ошибка распаковки: {}", e);
                        println!();
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(300));
    }
}

fn unpack_files(text: &str) -> Result<(Vec<String>, Vec<String>, Vec<String>), String> {
    let mut written = Vec::new();
    let mut deleted = Vec::new();
    let mut skipped = Vec::new();

    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // --- Удаление файла ---
        if let Some(path) = line.strip_prefix("DELETE ") {
            let path = path.trim().to_string();
            let p = Path::new(&path);
            if p.exists() {
                match fs::remove_file(p) {
                    Ok(_) => deleted.push(path),
                    Err(e) => skipped.push(format!("{} (ошибка удаления: {})", path, e)),
                }
            } else {
                skipped.push(format!("{} (не существует)", path));
            }
            i += 1;
            continue;
        }

        // --- Запись файла ---
        if let Some(path) = line.strip_prefix("FILE ") {
            let path = path.trim().to_string();
            i += 1;

            if i < lines.len() && lines[i].trim() == "====" {
                i += 1;
            }

            let mut content = String::new();
            while i < lines.len() && lines[i].trim() != "====" {
                content.push_str(lines[i]);
                content.push('\n');
                i += 1;
            }

            if i < lines.len() && lines[i].trim() == "====" {
                i += 1;
            }

            if i < lines.len() && lines[i].trim() == "END FILE" {
                i += 1;
            }

            // Базовая проверка пути
            if path.is_empty() || path.contains("..") {
                skipped.push(format!("{} (подозрительный путь)", path));
                continue;
            }

            let file_path = Path::new(&path);
            if let Some(parent) = file_path.parent() {
                if !parent.as_os_str().is_empty() {
                    if let Err(e) = fs::create_dir_all(parent) {
                        skipped.push(format!(
                            "{} (не могу создать папку {}: {})",
                            path,
                            parent.display(),
                            e
                        ));
                        continue;
                    }
                }
            }

            match fs::write(file_path, content) {
                Ok(_) => written.push(path),
                Err(e) => skipped.push(format!("{} (ошибка записи: {})", path, e)),
            }
            continue;
        }

        i += 1;
    }

    if written.is_empty() && deleted.is_empty() {
        return Err("Не найдено ни одного блока FILE или DELETE".to_string());
    }

    Ok((written, deleted, skipped))
}
