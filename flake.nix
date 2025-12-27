{
  description = "Rust Development Shell";
  nixConfig.bash-prompt = "[nix(develop):\\w]$ ";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";

  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      devshell,

    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          devshell.overlays.default
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = (
          pkgs.devshell.mkShell {
            packages = with pkgs; [
              (pkgs.rust-bin.fromRustupToolchainFile ./tweed/rust-toolchain.toml)
              pkgs.wabt
              pkgs.pre-commit
              pkgs.hotspot
              pkgs.perf
            ];
            env = [
              {
                name = "LLC_ALL";
                value = "C";
              }
            ];
            commands = [
              {
                name = "style-checks";
                command = ''
                  $PRJ_ROOT/scripts/style-checks.bash
                '';
                help = "check code styles";
                category = "code-checks";
              }
              {
                name = "format-checks";
                command = ''
                  $PRJ_ROOT/scripts/format-checks.bash
                '';
                help = "check formatting.";
                category = "code-checks";
              }
              {
                name = "coverage-checks";
                command = ''
                  $PRJ_ROOT/scripts/coverage-checks.bash
                '';
                help = "check coverage.";
                category = "code-checks";
              }
              {
                name = "test-checks";
                command = ''
                  $PRJ_ROOT/scripts/test-checks.bash
                '';
                help = "run tests.";
                category = "code-checks";
              }
            ];
          }

        );
      }
    );
}
