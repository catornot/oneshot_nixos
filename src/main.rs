use std::{
    fs,
    os::unix::fs::symlink,
    process::{Command, Stdio},
};

use steamlocate::SteamDir;

const ONESHOT_ID: u32 = 420530;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oneshot_path = SteamDir::locate()
        .ok()
        .and_then(|steam_dir| {
            let (app, library) = steam_dir.find_app(ONESHOT_ID).ok()??;

            Some(library.resolve_app_dir(&app))
        })
        .ok_or("no oneshot found")?;
    let oneshot_lib_backup = oneshot_path.join("backup_lib");

    println!("path is {}", oneshot_path.display());

    fs::create_dir_all(&oneshot_lib_backup)?;
    for path in fs::read_dir(&oneshot_path)? {
        let lib = match path.map(|path| path.path()) {
            Ok(path)
                if path.is_file()
                    && path
                        .file_name()
                        .map(|name| {
                            name.to_string_lossy().find(".so").is_some()
                                && name.to_string_lossy().find("steam_api").is_none()
                                && name
                                    .to_string_lossy()
                                    .find("libboost_program_options")
                                    .is_none()
                                && name.to_string_lossy().find("ruby").is_none()
                                // TODO: get libxfconf-0.so.2 in deps.nix
                                && name.to_string_lossy().find("libxfconf").is_none()
                        })
                        .unwrap_or_default() =>
            {
                path
            }
            Ok(_) => {
                continue;
            }
            Err(err) => {
                eprintln!("error: {err}");
                continue;
            }
        };

        let dest = oneshot_lib_backup.join(lib.file_name().unwrap());

        if dest.exists() {
            fs::remove_file(&dest)?;
        }
        fs::copy(&lib, &dest)?;
        fs::remove_file(&lib)?;
    }

    // remove old symlinks
    for path in fs::read_dir(&oneshot_path)? {
        let lib = match path.map(|path| path.path()) {
            Ok(path)
                if path.is_symlink()
                    && path
                        .file_name()
                        .map(|name| name.to_string_lossy().find(".so").is_some())
                        .unwrap_or_default() =>
            {
                path
            }
            Ok(_) => {
                continue;
            }
            Err(err) => {
                eprintln!("error: {err}");
                continue;
            }
        };

        fs::remove_file(&lib)?;
    }

    const EXPERIEMENTAL_FLAGS: [&str; 4] = [
        "--extra-experimental-features",
        "nix-command",
        "--extra-experimental-features",
        "flakes",
    ];

    let out_link = oneshot_path.join("libs");

    let output = Command::new("nix")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        // .current_dir(  )
        .arg("build")
        .arg(".#deps")
        .args(EXPERIEMENTAL_FLAGS)
        .arg("--out-link")
        .arg(&out_link)
        .output()?;

    println!("{}", String::from_utf8(output.stderr).unwrap_or_default());
    println!("{}", String::from_utf8(output.stdout).unwrap_or_default());

    for path in fs::read_dir(out_link.join("lib"))? {
        let lib = match path.map(|path| path.path()) {
            Ok(path)
                if (path.is_file() || path.is_symlink())
                    && path
                        .file_name()
                        .map(|name| {
                            name.to_string_lossy().find(".so").is_some()
                                && name.to_string_lossy().find("libgio").is_none()
                        })
                        .unwrap_or_default() =>
            {
                path
            }
            Ok(_) => {
                continue;
            }
            Err(err) => {
                eprintln!("error: {err}");
                continue;
            }
        };

        let dest = oneshot_path.join(lib.file_name().unwrap());

        if dest.exists() {
            fs::remove_file(&dest)?;
        }
        symlink(&lib, &dest)?;
    }

    Ok(())
}
