final: _prev: {
  webserverConfigs = with final;
  {
    portfolio = {
      hostAt = "/"; # the webserver will serve the below files at this point
      linkFrom = "${pkgs.custom.portfolio}/lib";
    };
    other = { # where the files are saved to / offset from static
      hostAt = "/other"; # the webserver will serve the below files at this point
      linkFrom = "${pkgs.custom.portfolio}/lib";
    };
  };
}

