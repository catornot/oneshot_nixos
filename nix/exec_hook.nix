{
  version ? "0.1.0",
  lib,
  rustPlatform,
  pkgs,
}:
rustPlatform.buildRustPackage {
  name = "exec_hook";
  inherit version;

  buildInputs = [
  ];

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  src = ../exec_hook;

  meta = {
    license = lib.licenses.mit;
    maintainers = [ "cat_or_not" ];
  };

  cargoLock = {
    lockFile = ../exec_hook/Cargo.lock;
  };
}
