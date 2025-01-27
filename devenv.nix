{ pkgs, lib, ... }:

{
  packages = with pkgs; [
    openssl
    rustup
  ];

  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "stable";
    # channel = "nightly";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-std" "rust-src" ];
    targets = [ "x86_64-unknown-linux-musl" ];
  };

  # pre-commit.hooks = {
  #  rustfmt.enable = true;
  #  clippy.enable = true;
  # };
  
  enterShell = ''
    exec fish
  '';
}
