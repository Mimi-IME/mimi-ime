{ config, lib, pkgs, options, ... }:
let
  cfg = config.programs."mimi-ime";
  isHM = options ? home.packages;
in {
  options.programs."mimi-ime" = {
    enable = lib.mkEnableOption "Mimi IME";
    package = lib.mkOption {
      type = lib.types.package;
      description = "The mimi-ime package to use.";
    };
  };

  config = lib.mkIf cfg.enable (if isHM then {
    home.packages = [ cfg.package ];
    systemd.user.services."mimi-ime" = {
      Unit = {
        Description = "Mimi IME";
        After = [ "graphical-session.target" ];
      };
      Install.WantedBy = [ "graphical-session.target" ];
      Service = {
        ExecStart = "${cfg.package}/bin/mimi-ime";
        Restart = "on-failure";
      };
    };
  } else {
    environment.systemPackages = [ cfg.package ];
    systemd.user.services."mimi-ime" = {
      description = "Mimi IME";
      wantedBy = [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/mimi-ime";
        Restart = "on-failure";
      };
    };
    warnings = [
      "programs.mimi-ime: add 'input-method { enable; }' to your compositor config to activate the IME."
    ];
  });
}
