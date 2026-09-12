defaultPackage:
{ config, lib, pkgs, options, ... }:

let
  cfg = config.programs."mimi-ime";
  isHM = options ? home.packages;

  desktopItem = pkgs.makeDesktopItem {
    name = "mimi-ime";
    desktopName = "Mimi IME";
    comment = "Vietnamese Input Method";
    exec = "systemctl --user start mimi-ime";
    icon = "mimi-ime";
    terminal = false;
    categories = [ "Utility" "System" ];
  };
in {
  options.programs."mimi-ime" = {
    enable = lib.mkEnableOption "Mimi IME";
    package = lib.mkOption {
      type = lib.types.package;
      description = "The mimi-ime package to use.";
      default = defaultPackage;
    };
  };

  config = lib.mkIf cfg.enable (if isHM then {
    home.packages = [ cfg.package desktopItem ];

    systemd.user.services."mimi-ime" = {
      Unit = {
        Description = "Mimi IME";
        After = [ "graphical-session.target" ];
        StartLimitBurst = 5;
        StartLimitIntervalSec = 30;
      };
      Install.WantedBy = [ "graphical-session.target" ];
      Service = {
        ExecStart = "${cfg.package}/bin/mimi-ime";
        Restart = "on-failure";
        RestartSec = 1;
      };
    };
  } else {
    environment.systemPackages = [ cfg.package desktopItem ];

    systemd.user.services."mimi-ime" = {
      description = "Mimi IME";
      wantedBy = [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      unitConfig = {
        StartLimitBurst = 5;
        StartLimitIntervalSec = 30;
      };
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/mimi-ime";
        Restart = "on-failure";
        RestartSec = 1;
      };
    };

    warnings = [
      "programs.mimi-ime: add 'input-method { enable; }' to your compositor config to activate the IME."
    ];
  });
}
