with import <nixpkgs> {};
mkShell {
  packages = [
    rustc
    cargo
    rust-analyzer
    rustfmt
  ];
  shellHook = ''
    alias run="cargo run"
    alias build="cargo build"
  '';
}
