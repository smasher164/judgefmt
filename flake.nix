{
  description = "generate ASCII rules of inference";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let supportedSystems = [
      "aarch64-linux"
      "i686-linux"
      "x86_64-linux"
      "aarch64-darwin"
    ]; in
    flake-utils.lib.eachSystem supportedSystems (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [];
        };
      in
      {
        packages.judgefmt = pkgs.rustPlatform.buildRustPackage {
          name = "judgefmt";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.rustc
            pkgs.clippy
            pkgs.rustfmt
          ];
        };
      });
}
