# Rendre XIMOD Architect auto-installable sous Windows (Inno Setup)

Ce guide explique, pas à pas, comment produire un **installateur `setup.exe`** pour
XIMOD Architect : l'utilisateur double-clique, un assistant l'installe, crée les
raccourcis et l'inscrit dans « Applications et fonctionnalités » (avec désinstallation).

Le script d'installation est fourni : **`packaging/windows/ximod-architect.iss`**.
Tu n'as en principe qu'à le compiler.

---

## Ce que tu obtiendras

- Un fichier `installer\XIMOD_Architect_1.0.0_Setup.exe` (~ la taille de l'app + assets).
- Installation **par utilisateur** (pas de droits administrateur) dans
  `%LOCALAPPDATA%\Programs\XIMOD Architect`, un dossier accessible en écriture — c'est
  important car XIMOD écrit son `Config.ini` à côté de l'exécutable.
- Raccourci menu Démarrer (et bureau en option), plus un désinstalleur propre.

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
4. Ouvre l'app, change un réglage, ferme : un `Config.ini` doit apparaître dans le
   dossier d'installation (preuve que l'écriture fonctionne).
5. Teste la **désinstallation** via « Applications et fonctionnalités » : l'app, les
   raccourcis, `assets` et `Config.ini` doivent disparaître.

---

## Notes importantes

### Config.ini et emplacement d'installation
Le script installe **par utilisateur** (`PrivilegesRequired=lowest`), dans un dossier
accessible en écriture, précisément pour que `Config.ini` (écrit à côté de l'exe)
fonctionne. Si tu voulais plutôt une installation **pour tous les utilisateurs** dans
`C:\Program Files`, il faudrait modifier XIMOD pour écrire sa configuration dans
`%APPDATA%` (la dépendance `dirs` est déjà présente) — sinon la sauvegarde des
réglages échouerait faute de droits. Je peux te faire cette petite modification si
tu le souhaites.

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

## (Optionnel) Générer l'installateur automatiquement en CI

Inno Setup est préinstallé sur les runners `windows-latest` de GitHub Actions. Tu peux
compiler le `setup.exe` à chaque tag en ajoutant, dans le job **windows** de
`.github/workflows/release.yml`, une étape après le build :

```yaml
      - name: Construire l'installateur (Inno Setup)
        shell: pwsh
        run: |
          & "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe" `
            "/DMyAppVersion=$($env:GITHUB_REF_NAME -replace '^v','')" `
            "packaging\windows\ximod-architect.iss"
      - uses: actions/upload-artifact@v4
        with:
          name: windows-installer
          path: installer/*.exe
```

(Le `.iss` accepte alors la version passée par `/DMyAppVersion=…` ; si tu veux cette
option, je peux adapter le script pour qu'il utilise cette valeur quand elle est
fournie.) Dis-moi si tu veux que je l'intègre proprement au workflow.
