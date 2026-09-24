let
  pkgs = import <nixpkgs> {
    overlays = [ (import rust-overlay) ];
  };
  rust-overlay = fetchGit {
    url = "https://github.com/oxalica/rust-overlay";
    rev = "fb058ecf6d14837ea152a3d5225ce7f88ee5cde1";
    ref = "master";
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
pkgs.mkShell {
  packages = [
    toolchain
    pkgs.atk
    pkgs.gdk-pixbuf
    pkgs.gtk3
    pkgs.openssl
    pkgs.openssl.dev
    pkgs.package-version-server
    pkgs.pango
    pkgs.perl
    pkgs.pkg-config
    pkgs.webkitgtk_4_1
  ];
  env = {
    OPENSSL_NO_VENDOR = "1";
    RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
  };
}
