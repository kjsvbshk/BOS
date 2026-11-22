; Instalador de Windows (Inno Setup 6). No se compila a mano: lo llama
; packaging/build-package.ps1 con la versión y las carpetas correctas.

#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif
#ifndef SourceDir
  #define SourceDir "..\dist\Compraventa3Diamantes"
#endif
#ifndef OutputDir
  #define OutputDir "..\dist"
#endif

#define AppName "Compraventa 3 Diamantes"
#define AppExe "Compraventa3Diamantes.exe"

[Setup]
; Identificador fijo: no cambiarlo, o Windows tratará cada versión como otro programa.
AppId={{B13971DE-4755-4302-B0F6-A62C60810575}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion} (demo)
AppPublisher={#AppName}
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
OutputDir={#OutputDir}
OutputBaseFilename=Compraventa3Diamantes-{#AppVersion}-instalador
SetupIconFile=..\crates\pos_desktop\assets\icon\app.ico
UninstallDisplayIcon={app}\{#AppExe}
UninstallDisplayName={#AppName} (demo)
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
; Windows 10 u 11 de 64 bits (lo exige el compilador de Rust actual).
MinVersion=10.0
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
; Permite instalar sólo para el usuario actual si no hay permisos de administrador.
PrivilegesRequiredOverridesAllowed=dialog

[Languages]
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExe}"
Name: "{group}\{cm:UninstallProgram,{#AppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent
