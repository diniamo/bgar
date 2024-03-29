{
  mkShell,
  statix,
  deadnix,
  rust-analyzer,
  clippy,
  rustfmt,
  rustc,
  cargo,
  pkg-config,
  gtk3,
  gtk-layer-shell
}:
mkShell {
  packages = [
    # Nix
    statix
    deadnix

    # Develop
    rust-analyzer
    clippy
    rustfmt

    # Build
    rustc
    cargo

    pkg-config
    gtk3
    gtk-layer-shell
  ];
}
