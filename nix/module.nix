{ config, lib, pkgs, ... }:
let
  cfg = config.programs."mimi-ime";
  isHM = config ? home.packages;
in {
  options.programs."mimi-ime" = {
    enable = lib.mkEnableOption "Mimi IME";
  };

  config = lib.mkIf cfg.enable (if isHM then {
    home.packages = [ pkgs.mimi-ime ];
    systemd.user.services."mimi-ime" = {
      Unit = {
        Description = "Mimi IME";
        After = [ "graphical-session.target" ];
      };
      Install.WantedBy = [ "graphical-session.target" ];
      Service = {
        ExecStart = "${pkgs.mimi-ime}/bin/mimi-ime";
        Restart = "on-failure";
      };
    };
  } else {
    environment.systemPackages = [ pkgs.mimi-ime ];
    systemd.user.services."mimi-ime" = {
      Unit = {
        Description = "Mimi IME";
        After = [ "graphical-session.target" ];
      };
      Install.WantedBy = [ "graphical-session.target" ];
      Service = {
        ExecStart = "${pkgs.mimi-ime}/bin/mimi-ime";
        Restart = "on-failure";
      };
    };
    warnings = [
      "programs.mimi-ime: add 'input-method { enable; }' to your compositor config to activate the IME."
    ];
  });
}
