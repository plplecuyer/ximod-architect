; =============================================================================
; XIMOD Architect - script d'installation Inno Setup (Windows)
; -----------------------------------------------------------------------------
; Produit un installateur "setup.exe" qui :
;   - installe l'application et son dossier assets\ ;
;   - cree les raccourcis (menu Demarrer + bureau optionnel) ;
;   - inscrit XIMOD dans "Applications et fonctionnalites" (desinstallation) ;
;   - s'installe POUR TOUS LES UTILISATEURS dans Program Files (droits admin).
;     Les reglages de chaque utilisateur sont ranges dans %APPDATA%\XIMOD Architect,
;     donc l'application n'ecrit rien dans Program Files apres l'installation.
;
; Prerequis avant compilation :
;   1. Avoir compile l'executable Windows en release :  cargo build --release
;      -> target\release\ximod-architect.exe
;   2. Avoir l'icone  assets\icons\ximod-architect.ico
;   3. Avoir Inno Setup 6 installe (https://jrsoftware.org/isdl.php)
;
; Compilation :
;   - Ouvrir ce fichier dans "Inno Setup Compiler" puis Build > Compile (F9), ou
;   - En ligne de commande :  ISCC.exe packaging\windows\ximod-architect.iss
; Le setup est genere dans  installer\XIMOD_Architect_<version>_Setup.exe
; =============================================================================

#define MyAppName "XIMOD Architect"
; La version peut etre passee par la ligne de commande : ISCC /DMyAppVersion=1.2.3
; (utilise par la CI pour reprendre le tag). Valeur par defaut sinon.
#ifndef MyAppVersion
  #define MyAppVersion "1.0.0"
#endif
#define MyAppPublisher "XIMOD"
#define MyAppURL "https://www.nexusmods.com/"
#define MyAppExeName "ximod-architect.exe"

[Setup]
; AppId identifie l'application de facon unique (NE PAS le changer entre versions).
AppId={{059D7A9E-6EB8-42D5-A75E-8D1DCC7D5CFB}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}

; Installation POUR TOUS LES UTILISATEURS dans Program Files (droits admin requis).
; {autopf} devient C:\Program Files. L'application n'ecrit rien ici apres coup :
; sa configuration (Config.ini) va dans %APPDATA%\XIMOD Architect (voir src/config.rs).
PrivilegesRequired=admin
DefaultDirName={autopf}\{#MyAppName}
DisableProgramGroupPage=yes
DefaultGroupName={#MyAppName}
UninstallDisplayIcon={app}\{#MyAppExeName}
UninstallDisplayName={#MyAppName}

; 64 bits uniquement (le binaire Rust release est x86-64).
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

; Sortie
OutputDir=..\..\installer
OutputBaseFilename=XIMOD_Architect_{#MyAppVersion}_Setup
SetupIconFile=..\..\assets\icons\ximod-architect.ico
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "french";  MessagesFile: "compiler:Languages\French.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; L'executable
Source: "..\..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
; Toutes les donnees (data, locales, fonts, images, icons) a cote de l'exe
Source: "..\..\assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs
; Documents optionnels (decommentez si vous voulez les inclure)
; Source: "..\..\README.md";           DestDir: "{app}"; Flags: ignoreversion
; Source: "..\..\XIMOD_Architect_Manual_GBR.pdf"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}";                        Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}";  Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}";                  Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#MyAppName}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; A la desinstallation, retire aussi les donnees copiees.
; La configuration utilisateur (%APPDATA%\XIMOD Architect\Config.ini) est laissee
; en place volontairement (reglages conserves si l'utilisateur reinstalle).
Type: filesandordirs; Name: "{app}\assets"
