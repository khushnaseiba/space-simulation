{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
  ];

  perSystem =
    {
      self',
      pkgs,
      lib,
      ...
    }:
    {
      rust-project.crates.space-simulation.crane.args = {
        # Build the cdylib for the browser instead of the host.
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        doCheck = false;

        nativeBuildInputs = [ pkgs.wasm-bindgen-cli ];
      };

      # Only the final package (not clippy/doc) generates the web bundle.
      rust-project.crates.space-simulation.crane.extraBuildArgs.installPhase = ''
        mkdir -p $out/pkg
        wasm-bindgen --target web --out-dir $out/pkg \
          target/wasm32-unknown-unknown/release/space_simulation.wasm
        cp ${../../index.html} $out/index.html
      '';

      packages.default = self'.packages.space-simulation;

      apps.default = {
        type = "app";
        program = "${pkgs.writeShellScriptBin "space-simulation-serve" ''
          cd ${self'.packages.default}
          exec ${lib.getExe pkgs.python3} -m http.server 8000 --bind 127.0.0.1
        ''}/bin/space-simulation-serve";
        meta.description = "Serve the space-simulation web build locally";
      };
    };
}
