{
  mkShell,
  statix,
  deadnix,
  rust-analyzer,
  clippy,
  rustfmt,
  bgar,
}:
mkShell {
  inputsFrom = [bgar];
  packages = [
    # Nix
    statix
    deadnix

    # Develop
    rust-analyzer
    clippy
    rustfmt
  ];
}
