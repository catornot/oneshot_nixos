{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust-overlay,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        toolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      in
      {
        formatter = pkgs.nixfmt-tree;

        packages = {
          deps = pkgs.callPackage ./nix/deps.nix { inherit (self.packages.${system}) xconf-stub exec_hook; };
          install = pkgs.callPackage ./nix/install.nix { };
          xconf-stub = pkgs.callPackage ./nix/xconf-stub.nix { };
          exec_hook = pkgs.callPackage ./nix/exec_hook.nix { };
          default = self.packages.${system}.install;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            toolchain
          ];
        };
      }
    );
}
