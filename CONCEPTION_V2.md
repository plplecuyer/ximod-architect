# XIMOD Architect — Conception de la Version 2

Document de conception et plan d'implémentation de la V2. Rédigé au départ pour
accompagner le squelette de code de la branche `2.0.0-dev`, il est tenu à jour
jusqu'à la publication : les sections 1 à 7 décrivent les douze items de la
feuille de route initiale, la section 8 l'organisation du code, et la section 9
les lots ajoutés ensuite à partir de l'audit `ANALYSE_V2.md`.

- Version du paquet : **`2.0.0`** (dans `Cargo.toml`), publiée le 2026-10-07.
- Toolchain : Rust 1.98.1 (MSRV), édition 2024, eframe/egui 0.29.1.
- État : `cargo fmt --check`, `cargo clippy --all-targets -D warnings` et
  `cargo test` verts — **299 tests**, 47 394 lignes de Rust, 84 fichiers.
- **Lot A livré** : conflits de destination (2) + export 7z (3).
- **Lot B livré** : modèles réutilisables (6), assistant de dossiers (4),
  glisser-déposer (8) — implémentés, testés et câblés dans l'UI.
- **Lot C livré** : comparaison de FOMOD (10), lecture d'en-têtes .esp/.esm/.esl (5),
  traitement d'images (7).
- **Lot D livré** : éditeur visuel de conditions (9), batch CLI (11),
  empaquetage BA2 expérimental (12).
- **Lots de l'audit livrés** (§9) : C0, C1, U1, T1–T2, U6+U9, T3–T4, C2 et F
  (fonctions 1 à 11 de `ANALYSE_V2.md` §4).
- i18n : toutes les clés traduites dans les **33 langues** (parité vérifiée par
  le test `test_all_locales_define_every_key`).
- Manuel utilisateur **EN + FR** à jour (chapitre 8 « Version 2 — nouveautés »,
  § 8.1 à 8.25), livré en PDF dans `Manuals/`.
- CLI : `validate` · `build` · `package` · `batch` · `inspect` · `archive` ·
  `translate` · `simulate` · `ba2`.
- Les 12 items de la feuille de route V2 et les 8 lots de l'audit sont implémentés.

---

## 1. Objectifs et principes

La V1 est complète côté fonctionnalités d'édition. La V2 vise la **fiabilité des
installateurs produits** et la **productivité de l'auteur**, en prolongeant le
simulateur de prévisualisation, la validation par schéma et l'export déjà
présents.

Trois principes guident l'architecture, dans la continuité de la V1 :

1. **Modèle sans i18n.** La logique (détection, comparaison, lecture d'en-têtes…)
   vit dans `src/models/` et renvoie des types *neutres* (énumérations/anglais
   technique). La couche UI traduit ces types en messages localisés, exactement
   comme le fait déjà `ValidationError` et le tout récent `models::verify`.
2. **Zéro régression sur les chemins V1.** Les nouveautés s'ajoutent à côté de
   l'existant. Exemple : l'export 7z passe par un nouveau point d'entrée
   `archive::export_archive`, et la branche ZIP y délègue au code V1 inchangé
   (`export::build_distribution_archive`).
3. **Parité CLI.** Toute capacité « traitable en lot » doit être accessible sans
   interface, pour l'intégration continue (commande `batch`).

---

## 2. Cartographie du squelette

Fichiers créés (ou déjà présents) et leur rôle :

| Fichier | Item feuille de route | Rôle |
|---|---|---|
| `src/models/verify.rs` | 1 — Vérification des fichiers référencés | **Déjà implémenté** et câblé dans l'UI (V1.0.5) |
| `src/models/conflicts.rs` | 2 — Conflits de destination | ✅ **Implémenté** — détection des collisions (fichiers + dossiers) |
| `src/archive.rs` | 3 — Export 7z · 12 — BA2 | ✅ **7z implémenté** (LZMA2) · BA2 : stub |
| `src/wizard.rs` | 4 — Assistant structure de dossiers | Proposition de squelette depuis l'arborescence |
| `src/models/plugin_header.rs` | 5 — Lecture en-têtes .esp/.esm/.esl | Masters requis, nom interne, flag ESL |
| `src/models/templates.rs` | 6 — Modèles réutilisables | Enregistrer/réinjecter step/groupe/option |
| `src/media.rs` | 7 — Traitement d'images | Redimensionnement/conversion + validation |
| `src/ui/dragdrop.rs` | 8 — Glisser-déposer & aperçu | Affectation de fichiers par glisser-déposer |
| `src/ui/condition_editor.rs` | 9 — Éditeur visuel de conditions | Fenêtre graphe des drapeaux/dépendances |
| `src/models/compare.rs` | 10 — Comparaison de FOMOD | Diff structurel entre deux projets |
| `src/cli_batch.rs` | 11 — Extensions CLI | Commande `batch` depuis un manifeste JSON |

Enregistrements effectués : `models/mod.rs` (compare, conflicts, plugin_header,
templates), `ui/mod.rs` (condition_editor, dragdrop), `main.rs` (archive,
cli_batch, media, wizard) et la commande `batch` branchée dans `run_cli`.

---

## 3. Détail par fonctionnalité

### Item 1 — Vérification des fichiers référencés ✅ (déjà livré)

`models::verify::verify_files(ximod, root) -> Vec<FileIssue>` vérifie l'existence
de chaque source/image sous la racine, repère les chemins absolus, les
échappements hors racine (`..`) et les fichiers orphelins. Déjà appelé par
`main_window.rs` et traduit par `translate_file_issue`. Rien à refaire ; sert de
patron pour l'item 2.

### Item 2 — Détection des conflits de destination ✅ (livré, Lot A)

- **Module :** `models/conflicts.rs` — `Conflict`, `ConflictSource`,
  `ConflictMode`, `detect_conflicts(ximod, mode)`.
- **Algorithme :** collecter `(destination normalisée, source, localisation)` sur
  `required_files`, tous les plugins et `conditional_files` ; regrouper par
  destination ; conserver les groupes ≥ 2 ; marquer `certain` selon le contexte
  de sélection (toujours installé, ou options coexistant sous
  `SelectAll`/`SelectAny`/`SelectAtLeastOne`).
- **Lien simulateur :** le simulateur pourra afficher l'arborescence finale et
  surligner les collisions à partir de ce rapport.
- **Dépendances :** aucune nouvelle.

### Item 3 — Export 7z ✅ (livré, Lot A)

- **Module :** `archive.rs` — `ArchiveFormat {Zip, SevenZip}`,
  `CompressionLevel`, `export_archive(...)`.
- **Algorithme :** préparer l'arborescence FOMOD comme la branche ZIP, puis
  compresser en LZMA2 selon le niveau choisi. La branche ZIP reste déléguée au
  code V1.
- **Dépendance ajoutée :** `sevenz-rust = "0.6"` (écriture LZMA2, Rust pur).
  Niveau de compression : paramètre présent (`CompressionLevel`), mapping fin
  vers un préréglage LZMA2 laissé en TODO (défaut LZMA2 pour l'instant).
- **UI :** un choix de format (ZIP/7z) + niveau dans la boîte d'export.

### Item 4 — Assistant depuis la structure de dossiers ✅ (livré, Lot B)

- **Module :** `wizard.rs` — `WizardOptions`, `propose_from_folder(root, opts)`.
- **Algorithme :** énumérer les sous-dossiers de premier niveau (via `walkdir`,
  déjà présent) ; « un sous-dossier = une option » ; regrouper selon les options ;
  détecter une image d'en-tête évidente à la racine. Produit un `Ximod` que
  l'auteur ajuste.
- **Dépendances :** aucune nouvelle.

### Item 5 — Lecture des en-têtes de plugins (.esp/.esm/.esl) ✅ (livré, Lot C)

- **Module :** `models/plugin_header.rs` — `PluginKind`, `PluginHeader`,
  `read_plugin_header(path)`.
- **Algorithme :** lire l'enregistrement `TES4` (little-endian) : taille, flags
  (flag ESL `0x200`), puis sous-enregistrements `MAST` (masters), `CNAM`
  (auteur), `SNAM` (description). Ne jamais lire au-delà du `TES4`.
- **Usage :** proposer automatiquement des dépendances de fichier et vérifier la
  cohérence avec le jeu cible.
- **Dépendances à envisager :** parseur maison (format simple et stable) ou crate
  `esplugin`. Le squelette penche pour un parseur maison (pas de dépendance).

### Item 6 — Modèles réutilisables ✅ (livré, Lot B)

- **Module :** `models/templates.rs` — `Template`, `TemplateBody
  {Step|Group|Plugin}`, `load_templates`, `save_template`, `apply_template`.
- **Format :** JSON, en réutilisant les dérivations `serde` existantes du modèle.
  Dossier de modèles sous le répertoire de configuration de l'app.
- **Dépendances :** aucune nouvelle (`serde_json` déjà présent).

### Item 7 — Traitement des images ✅ (livré, Lot C)

- **Module :** `media.rs` — `ImageConstraints`, `ImageIssue`, `validate_image`,
  `process_image`.
- **Algorithme :** décoder (crate `image`, déjà présent), redimensionner en
  conservant le ratio sous les bornes, ré-encoder dans un format autorisé,
  valider format/dimensions.
- **Dépendances :** aucune nouvelle.

### Item 8 — Glisser-déposer & aperçu amélioré ✅ (livré, Lot B — glisser-déposer)

- **Module :** `ui/dragdrop.rs` — `dropped_files(ctx)`,
  `assign_dropped_to_plugin(plugin, paths)`.
- **Mécanique egui :** `ctx.input(|i| i.raw.dropped_files)` pour le dépôt OS ;
  `dnd_drag_source`/`dnd_drop_zone` pour le glisser interne. Rendre les sources
  relatives à la racine avant de créer les `InstallFile`.
- **Aperçu :** agrandir et fiabiliser l'aperçu d'image (en-tête + options).
- **Dépendances :** aucune nouvelle.

### Item 9 — Éditeur visuel de conditions ✅ (livré, Lot D)

- **Module :** `ui/condition_editor.rs` — fenêtre indépendante `ConditionEditor`
  (viewport libre, comme l'aperçu), avec `show(ctx, ximod, i18n, autocomplete, builder)`.
- **Réalisation finale :** plutôt qu'un graphe, une liste par drapeau (« défini
  par » / « utilisé par », valeurs définies et testées, orphelins signalés),
  chaque ligne étant un lien vers le nœud ; renommage d'un drapeau partout,
  suppression de ses usages, et un **constructeur de conditions** (« SI … ET/OU … »)
  qui édite la condition du nœud sélectionné, groupes imbriqués compris. Menus
  Déployer/Replier et navigation clavier via `ui/foldnav.rs`.
- **Dépendances :** aucune (rendu egui).

### Item 10 — Comparaison de deux versions de FOMOD ✅ (livré, Lot C)

- **Module :** `models/compare.rs` — `ProjectDiff`, `DiffItem`,
  `diff_projects(from, to)`.
- **Algorithme :** apparier les étapes par nom (repli sur l'index), puis groupes
  et options par nom, puis fichiers par `(source, destination)` ; émettre les
  ensembles ajoutés/retirés et les changements de métadonnées scalaires.
- **Usage :** aide à la rédaction du changelog ; détection des pertes
  accidentelles.
- **Dépendances :** aucune nouvelle.

### Item 11 — Extensions CLI (build en lot) ✅ (livré, Lot D)

- **Module :** `cli_batch.rs` — `BatchManifest`, `BatchEntry`,
  `run_batch(manifest)` ; commande `batch <manifest.json>` branchée dans
  `run_cli`.
- **Manifeste JSON :** liste d'entrées `{root, format?, out?}`. Pour chaque
  entrée : valider → construire → empaqueter (via `archive`) ; code de sortie non
  nul si une entrée échoue (fait échouer le job CI).
- **Dépendances :** aucune nouvelle.

### Item 12 — Empaquetage BA2 ✅ (livré, Lot D — expérimental, GNRL v1 non compressé)

- **Module :** `archive.rs` — `Ba2Game {SkyrimSE, Fallout4, Starfield}`,
  `Ba2Options`, `package_ba2(src, out, opts)`.
- **Algorithme :** écrire le conteneur BA2 (en-tête BTDX, enregistrements de
  fichiers, table des noms) selon le format du jeu cible. Fonctionnalité lourde,
  planifiée en fin de cycle.
- **Dépendances à envisager :** crate `ba2`, ou implémentation maison du format.

---

## 4. Nouvelles dépendances envisagées

| Besoin | Candidat | Alternative |
|---|---|---|
| Export 7z (item 3) | `sevenz-rust` (LZMA2) | binaire `7z` système |
| Lecture en-têtes (item 5) | parseur maison | crate `esplugin` |
| BA2 (item 12) | crate `ba2` | format maison |

Les items 2, 4, 6, 7, 8, 9, 10, 11 n'ajoutent **aucune** dépendance : ils
réutilisent `serde`/`serde_json`, `image`, `walkdir`, `anyhow` et `eframe/egui`
déjà présents.

---

## 5. Stratégie de tests

- **Modèle (pur) :** tests unitaires par module (déjà amorcés : conflits,
  comparaison, en-têtes, modèles, archive). Les cas à implémenter sont présents
  mais `#[ignore]` tant que l'algorithme n'est pas écrit, pour garder la suite
  verte sans masquer le travail restant.
- **Golden files :** pour la lecture d'en-têtes et BA2, ajouter de petits
  fichiers d'exemple et comparer les sorties.
- **CLI :** tester `batch` sur un manifeste jouet pointant des FOMOD de test.
- **Non-régression V1 :** la suite existante (32 tests V1) reste verte ; la V2
  ne modifie pas les chemins V1.

---

## 6. Lots d'implémentation proposés

1. **Lot A — Fiabilité (fort impact, effort raisonnable) :** item 2 (conflits de
   destination) + item 3 (export 7z). ✅ **Livré.** Avec l'item 1 déjà présent, le
   « trio prioritaire » de la feuille de route est complet.
2. **Lot B — Productivité :** item 6 (modèles), item 4 (assistant de dossiers),
   item 8 (glisser-déposer). ✅ **Livré.**
3. **Lot C — Analyse :** item 10 (comparaison), item 5 (en-têtes de plugins),
   item 7 (images). ✅ **Livré.**
4. **Lot D — Avancé :** item 9 (éditeur visuel), item 11 (CLI batch complet),
   item 12 (BA2). ✅ **Livré.**

Chaque lot a été livré indépendamment sous forme de pré-versions `2.0.0-dev`,
avant la `2.0.0` finale.

---

## 7. Après la 2.0.0

Les lots A à D (§3) puis les huit lots de l'audit (§9) sont livrés et la
version 2.0.0 est publiée. Restent ouverts, pour un cycle ultérieur :

- l'écriture BA2 (`ba2`, expérimentale : GNRL v1 non compressé, validée par
  relecture mais pas encore en jeu) ;
- l'écran d'accueil (U7 de l'audit) et la vue « cartes » des tables de fichiers
  (U12), non retenus ;
- la traduction automatique (ANALYSE §3.10), écartée par choix : XIMOD ne fait
  aucun appel réseau implicite ;
- les chantiers continus : relecture communautaire des 31 traductions non
  référentes, nouveaux jeux dans `Games.json`, retours des utilisateurs Nexus.

## 8. Organisation du module `ui` (après le lot C2)

`src/ui/main_window.rs` ne garde que la structure `XimodApp`, ses `Default`/`new`, les onglets (`Tab`, `SettingsTab`), la sélection (`select_*`, `clamp_selection`), l'historique (`mark_modified`, `undo`, `redo`), les polices, les notifications et l'implémentation `eframe::App`. Le reste est réparti ainsi :

| Fichier | Rôle |
|---|---|
| `ui/selection.rs` | `Selection { step, group, plugin, cond_pattern }`, capturée telle quelle dans chaque `DocState`. |
| `ui/docs.rs` | Cycle multi-document : `DocState`, `CloseScope`, ouverture/fermeture/bascule d'onglets, chargement, enregistrement, export, dépôt de dossiers, barre d'onglets. Tests du cycle complet. |
| `ui/menu.rs` | Barre de menus, barre d'outils, raccourcis clavier, ouverture du manuel, de l'éditeur de traduction, des réglages et des modèles. |
| `ui/validation.rs` | Validation complète et traduction des diagnostics en messages i18n. |
| `ui/tabs/{info,steps,required,conditional}.rs` | Contenu des quatre onglets de l'inspecteur. |
| `ui/dialogs/{settings,about,script,confirm,compare,templates,update}.rs` | Boîtes de dialogue modales et bannière de mise à jour. |
| `ui/widgets/files_table.rs` | Table de fichiers partagée (`FilesTarget` : option, fichiers requis, ensemble conditionnel). |
| `ui/widgets/dependency_editor.rs` | Éditeur de dépendances partagé (`DepTarget` : visibilité d'étape, motif d'option, ensemble conditionnel). |
| `ui/widgets/free_window.rs` | Fenêtres libres (`free_viewport_builder`, `record_win_geom`, `free_window_closed`). |
| `ui/tree.rs` | Arbre du projet (sélection, glisser-déposer, menus contextuels, curseur clavier `NavKey`, pliage `Fold`). |
| `ui/problems.rs` | Panneau Problèmes par document, `Target`, `select_target`, `target_of_loc`. |
| `ui/history.rs` | Annuler / rétablir : pile de snapshots `Ximod` bornée, fusion des frappes. |
| `ui/toasts.rs`, `ui/theme.rs`, `ui/labels.rs` | Notifications, palette sémantique et thèmes (système, pinned), libellés humains des énumérations. |
| `ui/jobs.rs` | Travaux en thread (export, validation disque) et leurs canaux. |
| `ui/preview.rs` | Simulateur : page d'informations, étapes, résumé, arbre final (`models/simulate.rs`), scénarios. |
| `ui/condition_editor.rs`, `ui/flag_picker.rs` | Éditeur de conditions (fenêtre libre) et sélecteur de drapeau. |
| `ui/foldnav.rs` | Comportement commun des arbres : menus Déployer/Replier, curseur clavier (↑ ↓ ← → Entrée, Début/Fin), file de pliage multi-images. |
| `ui/fomod_translation.rs`, `ui/translation.rs` | Traduction de FOMOD tiers (table, mémoire, glossaire, Uniques, paquet) et éditeur des traductions de l'interface. |
| `ui/project_strings.rs`, `ui/properties.rs`, `ui/xml_editor.rs` | Table des chaînes du projet, fenêtre Pays / langues, éditeur XML. |
| `ui/dialogs/{archive_view,backups,nexus_desc,plugins}.rs` | Contenu d'archive, sauvegardes, description Nexus, rapport des plugins. |
| `ui/components/` | Widgets partagés (`table_nav_keys`, affichage d'images, listes virtuelles…). |

Côté modèle : `models/{conflicts,simulate,flags,strings,translate,condition_text,nexus_desc,bethesda_archive,plugin_checks,plugin_header,compare,templates,verify}.rs` ; côté XML : `xml/{validate,patch,fidelity}.rs` ; à la racine : `archive.rs`, `archive_open.rs`, `backups.rs`, `cli_batch.rs`, `cli_translate.rs`, `crash_guard.rs`, `fonts.rs`, `media.rs`, `wizard.rs`.

Le formatage est fixé par `rustfmt.toml` (`max_width = 120`) et contrôlé, avec clippy (`-D warnings`) et la suite de tests, par `.github/workflows/ci.yml` à chaque push et pull request.

## 9. Lots issus de l'audit (`ANALYSE_V2.md`)

Après les lots A à D, l'audit du code et de l'interface a défini huit lots
supplémentaires, exécutés dans l'ordre de sa feuille de route (§5) :

| Lot | Contenu livré | Manuel |
|---|---|---|
| **C0** | Correctifs de corruption (dé-échappement XML, marques Fluent), index périmés, CDATA, écriture atomique, racine FOMOD, Shift+Tab ; `crash_guard.rs` ; test d'aller-retour XML | 8.11 |
| **C1** | Suppression du travail par frame, caches, export et validation en thread (`ui/jobs.rs`), préchargement au démarrage | 8.10 |
| **U1** | Enregistrement sûr, toasts, libellés humains et infobulles, barre d'outils et menus | 8.12 |
| **T1–T2** | Traduction de FOMOD existants : modèle, patcheur XML sans perte, validation, fusion, CLI `translate`, fenêtre | 8.13 |
| **U6 + U9** | Thème, arbre + inspecteur, annuler/rétablir, panneau Problèmes, glisser-déposer | 2.2, 8.12 |
| **T3–T4** | Paquet Nexus, gabarit, dialogue de mise à jour, mémoire et glossaire, CSV, aperçu traduit, « Uniques » | 8.13 |
| **C2** | Découpage de `main_window.rs` (§8), CI fmt + clippy + tests | — |
| **F** | Archives (8.14), fidélité (8.15), sauvegardes (8.16), masters (8.17), ESL et rapport (8.18), simulateur et scénarios (8.19), tailles (8.20), description Nexus (8.21), éditeur de conditions (8.22), chaînes du projet (8.23), BSA/BA2 (8.24) | 8.14–8.24 |

S'y ajoutent, en fin de cycle : les groupes de conditions imbriqués,
`moduleDependencies` et les conditions de version (8.25) ; les polices par
écriture (CJK, arabe, grec… via `fonts.rs`) ; la navigation clavier et les
menus Déployer/Replier communs à tous les arbres (`ui/foldnav.rs`) ; l'éditeur
de conditions et l'aperçu en fenêtres indépendantes ; et, dans la fenêtre
Pays / langues, les noms de pays dans la langue choisie dans Paramètres.
