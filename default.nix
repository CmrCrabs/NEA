# from rust-gpu

let
  pkgs = import <nixpkgs> {};
in with pkgs; stdenv.mkDerivation rec {
  name = "rust-gpu";

  hardeningDisable = [ "fortify" ];

  SSL_CERT_FILE = "${cacert}/etc/ssl/certs/ca-bundle.crt";

  nativeBuildInputs = [ rustup ];

  LD_LIBRARY_PATH = with xorg; lib.makeLibraryPath [
    vulkan-loader

    wayland libxkbcommon

    libX11 libXcursor libXi libXrandr
  ];
}
