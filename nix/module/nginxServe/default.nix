{ config, lib, pkgs, ... }:
let
  cfg = config.webserver.nginx;
in {
  options.webserver.nginx = {
    enable = lib.mkEnableOption "Enable Nginx web server";

    baseDir = lib.mkOption {
      type = lib.types.str;
      default = "localhost";
      example = "niko.ink";
      description = "The base directory path for virtual hosts";
    };

    virtualHosts = lib.mkOption {
      type = lib.types.attrsOf (lib.types.submodule {
        options = {
          root = lib.mkOption {
            type = lib.types.path;
            example = "/var/www/example";
            description = "The root directory for the virtual host";
          };

          serverName = lib.mkOption {
            type = lib.types.str;
            example = "example.com";
            description = "The server name for the virtual host";
          };

          enableACME = lib.mkOption {
            type = lib.types.bool;
            default = true;
            description = "Whether to enable ACME (Let's Encrypt) for the virtual host";
          };

          forceSSL = lib.mkOption {
            type = lib.types.bool;
            default = true;
            description = "Whether to force SSL redirection for the virtual host";
          };

          locations = lib.mkOption {
            type = lib.types.attrsOf (lib.types.submodule {
              options = {
                root = lib.mkOption {
                  type = lib.types.path;
                  example = "/var/www/example";
                  description = "The root directory for the location";
                };
                # Add more location-specific options as needed
              };
            });
            default = {};
            description = "Additional location configurations for the virtual host";
          };

          # Add more virtual host options as needed
        };
      });
      default = {};
      example = lib.literalExpression ''
        {
          "''${config.webserver.nginx.baseDir}/example1" = {
            root = "''${pkgs.custom.portfolio}/lib";
          };
          "''${config.webserver.nginx.baseDir}/example2" = {
            root = "/path/to/example2/files";
          };
        }
      '';
      description = "Virtual host configurations";
    };
  };

  config = lib.mkIf cfg.enable {
    services.nginx = {
      enable = true;
      virtualHosts = lib.mapAttrs (name: vhostConfig: {
        serverName = if vhostConfig.serverName != null then vhostConfig.serverName else name;
        enableACME = vhostConfig.enableACME;
        forceSSL = vhostConfig.forceSSL;
        locations = lib.mapAttrs (locName: locConfig: {
          inherit (locConfig) root;
          # Add more location-specific options as needed
        }) vhostConfig.locations;
        # Add more virtual host options as needed
      }) cfg.virtualHosts;
    };
  };
}

