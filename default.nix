{
  rustPlatform,
  lib,
  pkg-config,
  gtk3,
  gtk-layer-shell,
}: let
  inherit (rustPlatform) buildRustPackage;

  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
  buildRustPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;

    nativeBuildInputs = [pkg-config];
    buildInputs = [
    gtk3
    gtk-layer-shell
    ];

    meta = with lib; {
      description = "A layer shell program that renders information you would with a bar as your background instead.";
      homepage = "https://github.com/diniamo/bgar";
      license = licenses.mit;
      maintainers = with maintainers; [diniamo];
      platforms = platforms.linux;
      mainProgram = cargoToml.package.name;
    };
  }
