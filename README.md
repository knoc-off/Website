# Description
This is my website, using Yew, trunk, and Nix.

## Packages
- portfolio
    - outputs compiled files from trunk.
- webserver (W.I.P)
    - this should be the router behind my website in the future. running actix

## DevShells
The development environment is defined in `./nix/devshell`.

### Entering a DevShell

To enter a development shell:

```sh
nix develop .#portfolio
```

## License

This project is licensed under the [MIT License](LICENSE).
