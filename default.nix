{ pkgs, ... }:

let
	cargoToml = with builtins; fromTOML (readFile ./Cargo.toml);
	pname = cargoToml.package.name;
	version = cargoToml.package.version;
in
{
	app = pkgs.rustPlatform.buildRustPackage {
		inherit pname version;
		src = ./.;
		
		cargoLock = {
			lockFile = ./Cargo.lock;
		};

		nativeBuildInputs = [ pkgs.pkg-config ];
		PKGS_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
	};
}
