{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.libiconv
    pkgs.nixpkgs-fmt
  ];

  shellHook = ''
    # Command to be executed every time the shell starts
    echo "Rust is ready!"
  '';
}
