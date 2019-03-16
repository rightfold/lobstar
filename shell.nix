{pkgs ? import ./nix/pkgs.nix {}}:
pkgs.stdenv.mkDerivation {
    name = "deet";
    buildInputs = [
        pkgs.cargo
    ];
    phases = ["installPhase"];
    installPhase = ''
        {
            echo 'This derivation is not for building, just for use with '
            echo 'nix-shell.'
        } 2>&1
        false
    '';
}
