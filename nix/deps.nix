{
  pkgs,
  lib,
  xconf-stub,
  exec_hook,
}:
pkgs.symlinkJoin {
  name = "oneshot-env";
  paths = with pkgs; [
    libsigcxx.out
    pixman.out
    physfs
    glib.out
    dbus-glib.out
    libxcrypt-legacy.out
    stdenv.cc.cc

    # new stuff
    libmodplug.out
    libwebp.out
    libjpeg8.out
    libogg.out
    libz.out
    # uh
    freetype.out
    speex.out

    # wayland
    wayland.out
    
    # solstice
    # python stuff is left from the game libs
    libsForQt5.qt5.qtbase.out

    # working wallpaper
    (pkgs.writers.writeBashBin "xconf-query" ''
      echo "[OneShot] xfconf-query called: $@" >> ~/.oneshot_hook.log

      # your payload
      ${lib.getExe' pkgs.libnotify "notify-send"} "Wallpaper change intercepted (XFCE)"      

      # call real binary
    '')

    xconf-stub
    exec_hook
  ];
}
