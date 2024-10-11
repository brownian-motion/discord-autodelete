let
  pkgs = import <nixpkgs> {};
in

pkgs.mkShell {

  packages = with pkgs; [
    cargo
  ];

  CARGO_TERM_COLOR = "always";
}