let
  pkgs = import <nixpkgs> {
    overlays = [ (import rust-overlay) ];
  };
  rust-overlay = fetchGit {
    url = "https://github.com/oxalica/rust-overlay";
    rev = "7fcafdf6f0761462d68012aba6a627be3f008d68";
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
