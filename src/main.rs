use std::{
    fs::{self, Permissions},
    io::Write,
    os::unix::fs::{PermissionsExt, symlink},
    process::{Command, Stdio},
};

use steamlocate::SteamDir;

const ONESHOT_ID: u32 = 420530;

const WRAPPER: &str = r#"
#!/bin/sh

export LD_PRELOAD="libexec_hook.so"
export XDG_CURRENT_DESKTOP="XFCE"
export DESKTOP_SESSION="xfce"
# export PATH="$PWD/xfconf-query:$PATH"
./.oneshot-wrapped "$@"
"#;

const XFCONF_QUERY: &str = r#"
#!/bin/sh

# stub
"#;

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
                                && name.to_string_lossy().find("vorbis").is_none()
                                && name.to_string_lossy().find("SDL").is_none()
                                && name.to_string_lossy().find("libsound").is_none()
                                && name.to_string_lossy().find("openal").is_none()
                                && name.to_string_lossy().find("tiff").is_none()
                                && name.to_string_lossy().find("FLAC").is_none()
                                && name.to_string_lossy().find("python").is_none()
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

    let game_path = oneshot_path.join("oneshot");
    let game_wrapped = oneshot_path.join(".oneshot-wrapped");

    let wrapper_contents = fs::read_to_string(&game_path).unwrap_or_default();
    if game_path.exists() && !wrapper_contents.contains("#!/bin/sh") {
        fs::copy(&game_path, &game_wrapped)?;
        fs::remove_file(&game_path)?;

        fs::set_permissions(game_wrapped, Permissions::from_mode(0o775))?;

        let mut wrapper = fs::File::create(game_path)?;
        wrapper.write_all(WRAPPER.as_bytes())?;

        wrapper.set_permissions(Permissions::from_mode(0o774))?;
    } else if wrapper_contents != WRAPPER {
        let mut wrapper = fs::File::create(game_path)?;
        wrapper.write_all(WRAPPER.as_bytes())?;

        wrapper.set_permissions(Permissions::from_mode(0o774))?;
    }

    let mut stub = fs::File::create(oneshot_path.join("xfconf-query"))?;
    stub.write_all(XFCONF_QUERY.as_bytes())?;

    stub.set_permissions(Permissions::from_mode(0o774))?;

    fs::File::create(oneshot_path.join("wallpaper-cmd"))?
        .write_all("nix run nixpkgs#awww -- img".as_bytes())?;

    Ok(())
}
