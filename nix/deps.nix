{ pkgs }:
pkgs.symlinkJoin {
  name = "oneshot-env";
  paths = with pkgs; [
    libsigcxx
    pixman
    physfs
    glib.out
    xfconf.out
    libxcrypt-legacy.out
    stdenv.cc.cc

    # new stuff
    libmodplug.out
    libwebp.out
  ];
}
