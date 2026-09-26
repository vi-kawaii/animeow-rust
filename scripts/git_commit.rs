//! ```cargo
//! [dependencies]
//! ```
//!
//! git_commit.rs — открывает редактор для сообщения коммита,
//! затем коммитит и пушит. Редактор берётся из core.editor
//! (настраивается один раз: git config --global core.editor "zed --wait").

use std::process::Command;

fn run(cmd: &mut Command) -> Result<(), String> {
    let status = cmd.status().map_err(|e| format!("не удалось запустить: {e}"))?;
    if !status.success() {
        return Err(format!("команда завершилась с кодом {:?}", status.code()));
    }
    Ok(())
}

fn main() {
    // 1. git add -A — стейджим всё. Если хочется выборочно — уберём.
    if let Err(e) = run(Command::new("git").arg("add").arg("-A")) {
        eprintln!("❌ git add: {e}");
        std::process::exit(1);
    }

    // 2. git commit без -m: git сам откроет core.editor и дождётся закрытия.
    //    Если пользователь закроет файл без сообщения — git вернёт ненулевой код,
    //    и мы просто выйдем, не пытаясь пушить.
    if let Err(e) = run(Command::new("git").arg("commit")) {
        eprintln!("⚠️ git commit: {e}");
        eprintln!("   (вероятно, пустое сообщение или нечего коммитить — пуш отменён)");
        std::process::exit(1);
    }

    // 3. git push — только если коммит прошёл.
    if let Err(e) = run(Command::new("git").arg("push")) {
        eprintln!("❌ git push: {e}");
        eprintln!("   (коммит создан локально, пуш не удался — проверь remote/upstream)");
        std::process::exit(1);
    }

    println!("✅ Закоммичено и запушено.");
}
