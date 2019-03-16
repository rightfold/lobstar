let
    tarball = fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/1c10f5e395a58c4fc76092c77cdf02903fa16e34.tar.gz";
        sha256 = "1f7rqvvzj71msb9krckq8mjps7r5wdbga20zybz079clbrdhv9nf";
    };
    config = {
    };
in
    {}:
        import tarball {config = config;}
