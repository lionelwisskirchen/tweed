{
  description = "Rust Development Shell";
  nixConfig.bash-prompt = "[nix(develop):\\w]$ ";

  inputs = {
    nixpkgs.url = "git+https://gitlab+deploy-token-2663:gldt-iUzzjA9yUqfEzNcUM-5g@gitlab.lrz.de/aemy/tweed/mirrors/nixpkgs-mirror.git?ref=nixos-unstable";
    rust-overlay = {
      url = "git+https://gitlab+deploy-token-2661:gldt-fnxNgazf26aSZoB_DZJN@gitlab.lrz.de/aemy/tweed/mirrors/rust-overlay-mirror.git?ref=master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "git+https://gitlab+flake-utils-mirror-token:gldt--S6zra6rqoh9JVv9y1EG@gitlab.lrz.de/aemy/tweed/mirrors/flake-utils-mirror.git";
    devshell.url = "git+https://gitlab+devshell-mirror-token:gldt-uGXtGGQS6uk991XQSh1T@gitlab.lrz.de/aemy/tweed/mirrors/devshell-mirror.git?ref=main";
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
              {
                name = "benchmark-on-device";
                command = ''                   
                   cd $PRJ_ROOT/benchmarks/stm32/l4
                   nix develop --command bash -c 'openocd -f openocd.cfg > openocd.log 2>&1 &'
                   nix develop --command bash -c '
                         python ../run-benches.py --outname benchmark_output ../../benchmark_sources results'
                '';
                help = "Benchmark on STM32L4";
                category = "benchmarks";
              }
            ];
          }

        );
      }
    );
}
