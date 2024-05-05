{ pkgs }:

{
  portfolio = pkgs.callPackage ./portfolio { };
  webserver = pkgs.callPackage ./webserver { };
}

