{
  pkgs,
  shellHook ? "",
  name ? "default",
}:
# Should imported from flake.nix with rust overlay pkgs
let
  rust = pkgs.rust-bin.selectLatestNightlyWith (
    toolchain:
    toolchain.default.override {
      extensions = [
        "rust-src"
        "rust-std"
        "rust-analyzer"
        "llvm-tools-preview"
      ];
    }
  );
in
pkgs.mkShell {
  inherit name shellHook;

  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${rust}/lib/rustlib/src";

  buildInputs = with pkgs; [
    openssl
    pkg-config
    rust
  ];
  packages = with pkgs; [
    nixd
    nixfmt-rfc-style
    just
  ];
}
