# Rendre XIMOD Architect auto-installable sous Windows (Inno Setup)

Ce guide explique, pas à pas, comment produire un **installateur `setup.exe`** pour
XIMOD Architect : l'utilisateur double-clique, un assistant l'installe, crée les
raccourcis et l'inscrit dans « Applications et fonctionnalités » (avec désinstallation).

Le script d'installation est fourni : **`packaging/windows/ximod-architect.iss`**.
Tu n'as en principe qu'à le compiler.

---

## Ce que tu obtiendras

- Un fichier `installer\XIMOD_Architect_1.0.0_Setup.exe` (~ la taille de l'app + assets).
- Installation **pour tous les utilisateurs** (droits administrateur demandés) dans
  `C:\Program Files\XIMOD Architect`. XIMOD **n'écrit rien** dans ce dossier après
  l'installation : ses réglages (`Config.ini`) sont rangés dans
  `%APPDATA%\XIMOD Architect`, propre à chaque utilisateur.
- Raccourci menu Démarrer (et bureau en option), plus un désinstalleur propre.

> **Mode portable :** si un `Config.ini` est présent **à côté de l'exécutable**, XIMOD
> l'utilise en priorité (aucune écriture dans `%APPDATA%`). C'est le comportement des
> archives `.zip` portables produites par la CI. En installation Program Files, ce
> fichier n'existe pas, donc la configuration part automatiquement dans `%APPDATA%`.

---

## Étape 1 — Compiler l'exécutable Windows (release)

Sur une machine **Windows**, dans le dossier du projet :

```bat
cargo build --release
```

Cela produit `target\release\ximod-architect.exe`. (Rust s'installe depuis
https://rustup.rs ; l'édition 2024 requiert une toolchain récente.)

> L'icône de l'application est déjà intégrée à l'exe au moment de la compilation
> (via `build.rs`), à condition que `assets\icons\ximod-architect.ico` existe.

## Étape 2 — Vérifier les fichiers requis

Assure-toi d'avoir, à la racine du projet :

- `target\release\ximod-architect.exe` (étape 1) ;
- le dossier `assets\` complet (data, locales, fonts, images, icons) ;
- l'icône `assets\icons\ximod-architect.ico` (sert aussi d'icône au `setup.exe`).

## Étape 3 — Installer Inno Setup

Télécharge et installe **Inno Setup 6** (gratuit) depuis le site officiel :
https://jrsoftware.org/isdl.php

L'installation fournit deux outils utiles :
- **Inno Setup Compiler** (interface graphique) ;
- **ISCC.exe** (compilateur en ligne de commande), typiquement dans
  `C:\Program Files (x86)\Inno Setup 6\ISCC.exe`.

## Étape 4 — (Optionnel) Ajuster le script `.iss`

Ouvre `packaging\windows\ximod-architect.iss`. En haut, tu peux adapter :

```
#define MyAppVersion "1.0.0"
#define MyAppPublisher "XIMOD"
#define MyAppURL "https://www.nexusmods.com/..."   ; l'URL de ta page Nexus
```

Ne change **pas** la ligne `AppId={{059D7A9E-...}}` : cet identifiant doit rester
identique d'une version à l'autre pour que les mises à jour remplacent proprement
l'ancienne installation.

## Étape 5 — Compiler l'installateur

**Méthode simple (interface graphique) :**
1. Ouvre `packaging\windows\ximod-architect.iss` avec Inno Setup Compiler.
2. Menu **Build → Compile** (ou touche **F9**).
3. À la fin, le fichier apparaît dans `installer\XIMOD_Architect_1.0.0_Setup.exe`.

**Méthode ligne de commande** (depuis la racine du projet) :

```bat
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" packaging\windows\ximod-architect.iss
```

## Étape 6 — Tester

1. Double-clique sur `installer\XIMOD_Architect_1.0.0_Setup.exe`.
2. Suis l'assistant (langue, dossier, raccourci bureau optionnel).
3. Vérifie que XIMOD se lance, que la fenêtre s'affiche correctement, que les langues
   et les polices fonctionnent (le dossier `assets` a bien été copié à côté de l'exe).
4. Ouvre l'app, change un réglage, ferme : un `Config.ini` doit apparaître dans
   `%APPDATA%\XIMOD Architect` (tape `%APPDATA%` dans l'explorateur) — preuve que la
   sauvegarde des réglages fonctionne sans droits admin.
5. Teste la **désinstallation** via « Applications et fonctionnalités » : l'app, les
   raccourcis et `assets` doivent disparaître. Le dossier `%APPDATA%\XIMOD Architect`
   est laissé en place volontairement (réglages conservés en cas de réinstallation).

---

## Notes importantes

### Config.ini et emplacement d'installation
Le script installe **pour tous les utilisateurs** (`PrivilegesRequired=admin`) dans
`C:\Program Files\XIMOD Architect`. Pour que cela fonctionne sans droits d'écriture
dans Program Files, XIMOD range sa configuration ailleurs :

- **Installé (Program Files)** : `Config.ini` est écrit dans
  `%APPDATA%\XIMOD Architect` — un dossier propre à chaque utilisateur, toujours
  accessible en écriture. C'est le comportement par défaut.
- **Portable** : si un `Config.ini` existe **à côté de l'exécutable**, il est utilisé
  en priorité et rien n'est écrit dans `%APPDATA%`. Idéal pour une clé USB ou un
  gestionnaire de mods : décompresse le `.zip`, l'app reste autonome.

Cette logique est dans `src/config.rs` (fonction `config_dir()`). La dépendance
`dirs` fournit `%APPDATA%` de façon portable (et `~/.config`, `~/Library/Application
Support` sur Linux/macOS).

### SmartScreen / avertissement « éditeur inconnu »
Un installateur **non signé** déclenche l'avertissement Windows SmartScreen
(« Windows a protégé votre PC »). L'utilisateur peut cliquer « Informations
complémentaires → Exécuter quand même », mais pour une distribution propre :
- procure-toi un **certificat de signature de code** (OV ou EV) auprès d'une autorité ;
- signe l'exe **et** le setup avec `signtool` :

```bat
signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 ^
  /a target\release\ximod-architect.exe
signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 ^
  /a installer\XIMOD_Architect_1.0.0_Setup.exe
```

(Un certificat **EV** lève l'avertissement SmartScreen immédiatement ; un certificat
OV nécessite d'accumuler de la réputation.) C'est optionnel : sans signature,
l'installateur fonctionne, il y a juste l'avertissement au premier lancement.

### Installation silencieuse
L'installateur généré accepte les commutateurs standard Inno Setup, pratiques pour
un déploiement automatisé :

```bat
XIMOD_Architect_1.0.0_Setup.exe /VERYSILENT /NORESTART
```

### Nouvelle version
Pour publier une mise à jour : recompile l'exe, change `#define MyAppVersion` dans le
`.iss`, recompile l'installateur. Grâce à l'`AppId` inchangé, l'installateur met à jour
l'installation existante.

### Distribution sur Nexus
Tu peux téléverser le `setup.exe` comme fichier principal du mod. Beaucoup d'auteurs
proposent en parallèle une **archive portable** (le `.zip` produit par la CI, sans
installation) pour les utilisateurs de gestionnaires de mods — les deux formats
peuvent coexister sur la même page.

---

## Génération automatique de l'installateur en CI (déjà intégrée)

Le workflow `.github/workflows/release.yml` construit désormais l'installateur
**automatiquement à chaque tag**. Le job **windows** :
1. installe Inno Setup (via Chocolatey : `choco install innosetup`) ;
2. compile le `.iss` en reprenant la version du tag
   (`ISCC /DMyAppVersion=<version> packaging\windows\ximod-architect.iss`) ;
3. publie le `setup.exe` en artefact **`windows-installer`**, également joint à la
   Release GitHub aux côtés des archives Linux/macOS.

Tu n'as donc rien de plus à faire que **pousser un tag `vX.Y.Z`** (voir
`COMPILING-linux.md`, Voie 2). Le `.iss` reprend automatiquement la version fournie ;
sans `/DMyAppVersion`, il retombe sur la valeur par défaut (`1.0.0`) — utile en local.
