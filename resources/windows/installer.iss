[Setup]
AppName=Presence
AppVersion={#MyAppVersion}
DefaultDirName={autopf}\Presence
DefaultGroupName=Presence
UninstallDisplayIcon={app}\presence.exe
Compression=lzma2
SolidCompression=yes
OutputDir=..\..\target\release
OutputBaseFilename=presence_setup
SetupIconFile=..\icon.ico

[Files]
Source: "..\..\target\release\presence.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\resources\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Presence"; Filename: "{app}\presence.exe"; IconFilename: "{app}\icon.ico"
Name: "{autodesktop}\Presence"; Filename: "{app}\presence.exe"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Run]
Filename: "{app}\presence.exe"; Description: "{cm:LaunchProgram,Presence}"; Flags: nowait postinstall skipifsilent
