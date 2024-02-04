{ pkgs, shellHook ? "" , name ? "default" }:
# Should imported from flake.nix with rust overlay pkgs

let 
	rust = pkgs.rust-bin.stable.latest.default.override {
		targets = [ ];
		extensions = [
			"rust-src"
		];
	};
in 
pkgs.mkShell {
	inherit name shellHook;

	buildInputs = with pkgs; [
		openssl
		pkg-config
		rust
	];
	
	packages = with pkgs; [
		nil
	];
	
	RUST_BACKTRACE = 1;
	RUST_SRC_PATH = "${rust}/lib/rustlib/src";
}
