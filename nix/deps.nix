{ pkgs }:
pkgs.symlinkJoin {
  name = "oneshot-env";
  paths = with pkgs; [
    # mesa
    # libGL
    # libglvnd

    # glib
    # glib.out
    # gtk3
    # gtk2

    # dbus
    # zlib
    # curl

    # alsa-lib
    # pulseaudio

    # fontconfig
    # freetype

    # libxcrypt-legacy
    # libgcrypt

    # libsigcxx.out
    # pixman
    # zlib
    # SDL2
    # SDL2_image
    # SDL2_ttf
    # SDL_sound
    # openal
    # libvorbis
    # physfs
    # gtk3
    # xfconf.out
    # dbus-glib.out

    # wayland

    # libselinux.out
    # util-linux.lib
    # json-glib.out

    # libx11
    # libxext
    # libxcursor
    # libxrandr
    # libxi

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
