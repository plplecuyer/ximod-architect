# Compiler XIMOD Architect sous Linux

Ce guide décrit **deux façons** de produire un binaire Linux de XIMOD Architect,
à partir des deux fichiers d'outillage du projet :

| Fichier | Rôle |
| --- | --- |
| `packaging/linux/build-deps.sh` | Installe les dépendances de compilation sur une machine **Linux** (ou WSL). |
| `.github/workflows/release.yml` | Fait compiler **GitHub** à votre place (Linux + Windows + macOS). Ce workflow appelle lui‑même `build-deps.sh`. |

- **Voie 1** — compiler soi‑même sur une machine Linux : n'utilise que `build-deps.sh`.
- **Voie 2** — laisser GitHub compiler : utilise les **deux** fichiers ensemble (la compilation a lieu sur les serveurs de GitHub, pas sur votre PC).

Le binaire produit est un exécutable **Linux** (`x86_64-unknown-linux-gnu`) ; il ne
s'exécute pas sous Windows. Pour un `.exe` Windows ou un `.app` macOS, voir la Voie 2.

---

## Voie 1 — Compiler localement sous Linux

> Le fichier `release.yml` n'intervient **pas** ici.

### Étape 1 — Ouvrir un terminal et se placer dans le projet

Ouvrez un terminal Linux (ou, sous Windows, un shell **WSL2 Ubuntu**). Placez‑vous
dans le dossier qui contient `Cargo.toml` :

```bash
cd ~/ximod-architect      # adaptez au chemin réel de votre projet
ls Cargo.toml             # doit exister
```

> **WSL** : copiez d'abord le projet dans le système de fichiers Linux (`~/…`),
> **pas** sous `/mnt/c/…` (plus lent et source de problèmes de permissions / fins de ligne).

### Étape 2 — Rendre le script exécutable

```bash
ls packaging/linux/build-deps.sh
chmod +x packaging/linux/build-deps.sh
```

### Étape 3 — Installer les dépendances système

Le script détecte la distribution (Debian/Ubuntu, Fedora, Arch) et installe les
bonnes bibliothèques (il demandera le mot de passe `sudo`) :

```bash
./packaging/linux/build-deps.sh        # interactif (demande confirmation)
# ou, sans confirmation :
./packaging/linux/build-deps.sh -y
```

<details>
<summary>Paquets installés selon la distribution</summary>

- **Debian/Ubuntu** (`apt`) : `build-essential pkg-config curl git libgtk-3-dev
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev
  libxkbcommon-dev libssl-dev libx11-dev`
- **Fedora/RHEL** (`dnf`) : `gcc gcc-c++ make pkgconf-pkg-config curl git
  gtk3-devel libxcb-devel speech-dispatcher-devel libxkbcommon-devel
  openssl-devel libX11-devel`
- **Arch/Manjaro** (`pacman`) : `base-devel pkgconf curl git gtk3 libxcb
  speech-dispatcher libxkbcommon openssl libx11`

</details>

### Étape 4 — Installer Rust (si nécessaire)

Le script n'installe pas Rust volontairement. Vérifiez :

```bash
cargo --version
```

S'il n'est pas trouvé :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Étape 5 — Compiler

```bash
cargo build --release
```

La première compilation télécharge les dépendances et prend quelques minutes ;
les suivantes sont bien plus rapides.

### Étape 6 — Récupérer et lancer le binaire

Le résultat se trouve ici :

```bash
./target/release/ximod-architect
```

Lancez‑le directement (sous Windows 11, **WSLg** ouvre l'interface tout seul ;
sous Windows 10, il faut un serveur X comme VcXsrv). Pour une installation propre
avec les données :

```bash
./packaging/linux/install.sh          # installe binaire + assets + icône + entrée de menu
```

---

## Voie 2 — Laisser GitHub compiler (utilise les deux fichiers)

GitHub exécute `release.yml`, qui appelle `build-deps.sh` sur son runner Linux, et
compile en plus Windows et macOS.

### Étape 1 — Mettre le projet dans un dépôt GitHub

```bash
git init
git add .
git commit -m "XIMOD Architect"
git branch -M main
git remote add origin https://github.com/<votre-compte>/ximod-architect.git
git push -u origin main
```

Les deux fichiers doivent être aux bons emplacements dans le dépôt :
`packaging/linux/build-deps.sh` et `.github/workflows/release.yml`.

### Étape 2 — Vérifier l'emplacement du crate

Si votre `Cargo.toml` n'est **pas** à la racine du dépôt, ouvrez `release.yml` et
changez la ligne `MANIFEST_DIR: "."` en indiquant le sous‑dossier
(ex. `MANIFEST_DIR: "ximod-architect"`). Sinon, ne touchez à rien.

### Étape 3 — Premier essai sans publier (recommandé)

Sur GitHub : onglet **Actions** → workflow **Release** → bouton **Run workflow** →
branche `main` → **Run**. Cela compile les trois OS **sans** créer de version publique.

### Étape 4 — Suivre la compilation

Dans **Actions**, ouvrez le run en cours : les trois jobs (`linux`, `windows`,
`macos`) s'exécutent. En bas de la page du run, la section **Artifacts** contient
les archives à télécharger.

### Étape 5 — Produire une vraie version (quand vous êtes prêt)

Créez et poussez un tag `vX.Y.Z` :

```bash
git tag v1.0.0
git push origin v1.0.0
```

Le workflow se relance automatiquement, compile les trois OS **et** crée une
**Release GitHub** (onglet *Releases*) avec les fichiers en pièces jointes :

- `ximod-architect-1.0.0-linux-x86_64.tar.gz`
- `ximod-architect-1.0.0-windows-x86_64.zip`
- `ximod-architect-1.0.0-macos-universal.dmg` et `…-macos-universal.app.zip`

> **Signature macOS** : sans secrets, le `.app`/DMG est signé *ad‑hoc* (avertissement
> Gatekeeper chez l'utilisateur). Pour une vraie signature + notarisation, ajoutez les
> secrets de dépôt `MACOS_CERTIFICATE_BASE64`, `MACOS_CERTIFICATE_PASSWORD`,
> `MACOS_SIGN_IDENTITY`, et pour la notarisation `AC_APPLE_ID`, `AC_TEAM_ID`,
> `AC_PASSWORD` : le workflow les détecte automatiquement.

---

## Dépannage rapide

- **`linker 'cc' not found` / erreurs de compilation C** → dépendances système
  manquantes : relancez `./packaging/linux/build-deps.sh`.
- **`error: package requires rustc 1.xx`** → Rust trop ancien : `rustup update stable`.
- **La fenêtre ne s'ouvre pas sous WSL** → Windows 11 : WSLg est intégré ; Windows 10 :
  installez un serveur X (VcXsrv) et exportez `DISPLAY`. La *compilation*, elle,
  fonctionne sans écran.
- **Le binaire ne démarre pas sur une autre machine Linux (`GLIBC_x.y not found`)** →
  il a été compilé avec une glibc plus récente que la machine cible. Compilez sur une
  distribution plus ancienne (Ubuntu 22.04, voire 20.04) pour abaisser la version minimale.
- **Caractères manquants (carrés) dans l'interface** → le dossier `assets/fonts`
  (polices Noto) n'a pas été installé à côté du binaire ; utilisez `install.sh` ou
  copiez `assets/` auprès de l'exécutable.

---

## En résumé

- Pour **compiler sous Linux tout de suite sur votre machine** : **Voie 1**
  (`build-deps.sh` puis `cargo build --release`).
- Pour **compiler les trois OS automatiquement** : **Voie 2** (push d'un tag ; la
  compilation a lieu sur GitHub).

> Rappel : la publication d'une Release **publique** suppose l'accord de Wenderer
> (le portage dérive de son code C++). Les essais via *Run workflow* (Voie 2, étape 3)
> ne publient rien : ils sont sans risque.
