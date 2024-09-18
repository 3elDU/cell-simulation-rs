{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs";
		rust-overlay.url = "github:oxalica/rust-overlay";
		flak-utils.url = "github:numtide/flake-utils";
	};

	outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				overlays = [ (import rust-overlay) ];
				pkgs = import nixpkgs {
					inherit system overlays;
				};
			in {
				devShells.default = pkgs.mkShell {
					buildInputs = [
						pkgs.mesa
						pkgs.pkg-config
						pkgs.rust-bin.stable.latest.default
					];
				};
			}
		);
}
