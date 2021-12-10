with import <nixpkgs> {};
mkShell {
  packages = [
    rustc
    cargo
    rust-analyzer
    rustfmt
  ];
}
