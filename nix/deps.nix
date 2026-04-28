{ pkgs }:
pkgs.symlinkJoin {
  name = "oneshot-env";
  paths = with pkgs; [
    libsigcxx
    pixman
    # zlib
    # SDL2
    # SDL2_image
    # SDL2_ttf
    SDL_sound
    # openal
    # libvorbis
    physfs
    # gtk3
    glib.out
    xfconf.out
    # dbus-glib.out
    libxcrypt-legacy.out
    stdenv.cc.cc
  ];
}
