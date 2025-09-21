{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      rust-overlay,
      nixpkgs,
    }:
    {
      devShells.x86_64-linux.default =
        let
          pkgsCross = nixpkgs.legacyPackages.x86_64-linux.pkgsCross.mingwW64;
          rust-bin = rust-overlay.lib.mkRustBin { } pkgsCross.buildPackages;
        in
        pkgsCross.callPackage (
          {
            mkShell,
            pkg-config,
            stdenv,
          }:
          mkShell {
            nativeBuildInputs = [
              (rust-bin.nightly.latest.minimal.override {
                extensions = [ "rustfmt" "rust-src" "rust-analyzer" ];
              })
            ];

            depsBuildBuild = [];
            buildInputs = [];

            env = {
              CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${pkgsCross.windows.pthreads}/lib";
            };
          }
        ) { };
    };
}
