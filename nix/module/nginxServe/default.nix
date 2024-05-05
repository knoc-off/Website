{ config, lib, pkgs, ... }:
let
  # Define the base directory path
  baseDir = "niko.ink"; # Change this to your domain or IP address

  # Define the webserver configurations
  webserverConfigs = {
    "${baseDir}/example1" = {
      root = "${pkgs.custom.portfolio}/lib";
    };
    "${baseDir}/example2" = {
      root = "/path/to/example2/files";
    };
  };

in {
  options.webserver.nginx.enable = lib.mkEnableOption "Enable Nginx web server";

  config = lib.mkIf config.webserver.nginx.enable {
    services.nginx = {
      enable = true;
      virtualHosts = lib.mapAttrs (name: value: {
        serverName = name;
        enableACME = true;
        forceSSL = true;
        locations."/" = {
          root = value.root;
        };
      }) webserverConfigs;
    };
  };
}

