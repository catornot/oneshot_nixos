{
  version ? "0.1.0",
  lib,
  rustPlatform,
  pkgs,
}:
rustPlatform.buildRustPackage {
  name = "xconf-stub";
  inherit version;

  buildInputs = [
  ];

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  src = ../xconf-stub;

  meta = {
    license = lib.licenses.mit;
    maintainers = [ "cat_or_not" ];
  };

  cargoLock = {
    lockFile = ../xconf-stub/Cargo.lock;
  };

  postInstall = ''
    mv $out/lib/libxfconf_stub.so $out/lib/libxfconf-0.so.2
  '';

}
