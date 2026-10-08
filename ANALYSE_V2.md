# XIMOD Architect 2.0.0 — Analyse du code, de l'interface et des évolutions

> **État au 2026-10-07 (version 2.0.0).** Ce document est l'audit qui a piloté la fin du cycle 2.0 ; il est conservé tel quel comme pièce de référence, avec en tête le bilan de ce qui a été réalisé. Les numéros de ligne cités renvoient à l'instantané audité (18 129 lignes, 30 fichiers, 60 tests) et non au code final (47 394 lignes, 84 fichiers, 299 tests).

## État de réalisation

| Lot (§5) | Contenu | État |
|---|---|---|
| **C0** | B1–B7 corrigés, hook de panic (`crash_guard.rs`), test d'aller-retour XML, dépendances inutilisées retirées | ✅ livré |
| **C1** | plus de `commit_active` par frame ; caches (autocomplétion, polices, Propriétés, éditeur XML) ; export et validation en thread (`ui/jobs.rs`) ; démarrage avec préchargement sur thread et splash | ✅ livré |
| **U1** | U1, U2 (toasts), U4 (libellés humains), U5 (barre d'outils, menus) | ✅ livré |
| **T1–T2** | noyau de traduction (`models/translate.rs`, `xml/patch.rs`, CLI `translate`) + fenêtre de traduction de FOMOD | ✅ livré |
| **U6 + U9** | thème (`ui/theme.rs`), arbre + inspecteur, annuler/rétablir (`ui/history.rs`), panneau Problèmes par document, glisser-déposer | ✅ livré |
| **T3–T4** | paquet Nexus, gabarit de nom, dialogue de mise à jour, mémoire/glossaire, CSV, aperçu traduit, mode « Uniques » | ✅ livré |
| **C2** | découpage de `main_window.rs` (voir CONCEPTION_V2 §8), `Selection`, widgets partagés, tests multi-documents, CI | ✅ livré |
| **F** | fonctions 1 à 11 du §4 | ✅ livré (voir §4) |

Propositions UI du §2.3 : U1 à U6 et U8 à U11 livrées ; U12 partiellement (table de fichiers partagée `ui/widgets/files_table.rs`, sans colonnes redimensionnables ni vue « cartes ») ; U7 (écran d'accueil) non retenu pour 2.0.0. Traduction automatique (§3.10) : non retenue, par choix (aucun appel réseau implicite). Au-delà de l'audit, le cycle a ajouté : groupes de conditions imbriqués et `moduleDependencies` (§4-2 complet), polices par écriture (CJK, arabe…), navigation clavier et menus Déployer/Replier communs à tous les arbres (`ui/foldnav.rs`), éditeur de conditions et aperçu en fenêtres indépendantes, et le réglage pays → langue de la fenêtre Propriétés.

---

Base analysée : `scratchpad/v2/crate` (18 129 lignes de Rust, 30 fichiers, 60 tests, eframe/egui 0.29.1). Les numéros de ligne renvoient à `src/ui/main_window.rs` sauf mention contraire. Tout ce qui est marqué **[confirmé]** a été reproduit par un test exécuté ; le reste a été vérifié par lecture du code.

---

## 0. Synthèse

Le programme est fonctionnel et la V2 a apporté beaucoup de fonctions, mais l'audit fait ressortir trois choses :

1. **Deux bugs de corruption de données silencieuse**, tous deux dans le chemin le plus utilisé (ouvrir/enregistrer) :
   - les attributs XML sont relus sans dé-échappement, donc `&`, `<`, `>`, `"`, `'` sont doublement échappés à chaque cycle **[confirmé]** ;
   - les noms par défaut « Step 1 », « Group 1 », « Plugin 1 » contiennent des caractères Unicode invisibles (U+2068/U+2069) qui finissent dans `ModuleConfig.xml` **[confirmé]**.
2. **Une classe de plantages** : des index de sélection périmés utilisés dans `Vec::remove()` sans contrôle de borne. En release, `panic = "abort"` : l'application se ferme sans message et sans sauvegarde.
3. **L'interface est fonctionnelle mais datée et peu guidante** pour le public visé (moddeurs non développeurs) : cinq niveaux d'onglets identiques, pas d'annuler/rétablir, retours utilisateur par une ligne d'état unique, échecs silencieux (Ctrl+S sans dossier racine ne fait rien), thème egui brut.

Côté évolutions, **l'interface de traduction de FOMOD** que tu proposes est pertinente et bien délimitée. Point important : elle ne doit pas passer par `load_ximod → save_ximod` (chemin avec perte pour un FOMOD tiers) mais par un patcheur XML sans perte qui ne remplace que les chaînes ciblées. La conception complète est en §3.

Ordre recommandé : corriger les 2 bugs de corruption et les index périmés (une demi-journée, lot « C0 »), puis les gains rapides UI (lot « U1 »), puis la traduction FOMOD (lots T1→T3), puis la refonte arbre/inspecteur avec undo/redo.

---

## 1. Code Rust

### 1.1 Bugs à corriger en priorité

**B1 — Double échappement des attributs XML [confirmé].** `src/xml/mod.rs` lit chaque attribut avec `String::from_utf8_lossy(&attr.value)` (lignes 615, 629, 630, 641, 664, 692, 703, 724…), c'est-à-dire la valeur brute encore échappée. À l'écriture, `push_attribute` de quick-xml ré-échappe. Test exécuté : une étape nommée `Textures & Meshes` est relue `Textures &amp; Meshes`, puis réécrite `&amp;amp;`. Chaque ouvrir/enregistrer dégrade les noms de steps, groupes, plugins, flags, chemins source/destination et images. `validate.rs:153` utilise correctement `unescape_value()` — l'oubli est local à `mod.rs`.

```rust
fn attr_str(a: &quick_xml::events::attributes::Attribute) -> String {
    a.unescape_value().map(|c| c.into_owned())
        .unwrap_or_else(|_| String::from_utf8_lossy(&a.value).into_owned())
}
```
Ajouter un test d'aller-retour `parse(write(x)) == x` avec `& < > " '` (il faut `PartialEq` sur `Ximod`).

**B2 — Marques d'isolation Fluent dans les noms [confirmé].** `src/i18n/mod.rs:310` crée le `FluentBundle` sans `set_use_isolating(false)`. Fluent entoure alors chaque placeable de U+2068/U+2069. `default-step-name = Step { $num }` produit `"Step \u{2068}1\u{2069}"`, qui devient le nom réel du step/groupe/plugin (lignes 2736, 3089, 3189) et est écrit dans le XML. egui ne les affiche pas ; les autres outils (MO2, Vortex, éditeurs) les voient. `clean_attr` (`xml/mod.rs:218`) ne les retire pas (catégorie Cf, pas `is_control()`). Correctif : une ligne `bundle.set_use_isolating(false);` + étendre `clean_attr` à `'\u{2066}'..='\u{2069}'` pour nettoyer les fichiers déjà pollués.

**B3 — Index périmés → panic → abort.** Changer de plugin (3174), de groupe (3008), de step (2719) ou de pattern conditionnel (3998) ne remet pas à zéro `current_flag_index`, `current_file_index`, `current_dependency_index`, `current_cond_file_index`. Puis `condition_flags.remove(flag_idx)` (3492), `files.remove(file_idx)` (3835), `dependencies.remove(dep_idx)` (4145), `files.remove(file_idx)` (4243) sont appelés sans test de borne. Scénario : sélectionner le 3ᵉ flag du plugin A, cliquer sur le plugin B qui n'en a aucun, cliquer « Remove Flag » → panic. Même sans panic, l'index périmé supprime le mauvais élément. Même problème après « Appliquer » dans l'éditeur XML (`xml_editor.rs:86-95` remplace `steps` sans `reset_navigation()`). Correctif : une fonction `select_plugin(&mut self, pi)` qui réinitialise les sous-index, et `if idx < v.len()` devant chaque `remove`. En filet de sécurité, un `std::panic::set_hook` dans `main()` qui écrit une copie de secours du projet (le hook s'exécute même avec `panic = "abort"`).

**B4 — CDATA ignoré.** Aucun bras `Event::CData` dans `parse_info_xml` ni `parse_module_config_xml` : une description en `<![CDATA[…]]>` (courant dans les FOMOD tiers) est perdue au chargement.

**B5 — Écriture XML non atomique, erreurs avalées.** `save_info_xml` / `save_module_config_xml` (`xml/mod.rs:41-44, 109-112`) : `BufWriter` jamais flushé explicitement (une erreur disque plein au flush final est ignorée et « Enregistré » s'affiche), et `File::create` tronque le fichier avant d'écrire (un crash pendant l'écriture laisse un `ModuleConfig.xml` vide). Correctif : écrire dans `ModuleConfig.xml.tmp`, `sync_all()`, puis `rename`.

**B6 — `fomod_root_from_drop` (5362-5365) et `open_file` (1119).** Les deux branches renvoient `parent.parent()`, le test `pname == "fomod"` n'a aucun effet. Comme `load_ximod` renvoie `Ok(Ximod::default())` quand aucun XML n'existe (`xml/mod.rs:480-497`), déposer n'importe quel fichier ouvre un projet vide « avec succès » et l'ajoute aux récents.

**B7 — Shift+Tab mort dans Réglages (4334/4346).** `consume_key(NONE, Tab)` est testé avant `consume_key(SHIFT, Tab)` ; egui ignore le Shift en trop, donc Shift+Tab est consommé comme Tab. L'ordre correct est déjà utilisé dans `translation.rs`, `flag_picker.rs`, `properties.rs`.

**B8 — Scripts pré/post-save.** `config.rs:342-345 / 432-436` : l'échappement `\n` ↔ saut de ligne n'est pas réversible, un chemin `C:\new_mod` est corrompu au rechargement. `run_script` (`config.rs:520-563`) bloque le thread UI, ignore le code de retour, et utilise un nom de fichier temporaire fixe. Les deux appels font `let _ = run_script(...)` (1251, 1274).

**B9 — `translation.rs:870`** : si le `main.ftl` source est illisible, `unwrap_or_default()` puis écriture → le `main.ftl` cible est écrasé par le seul en-tête. **`translation.rs:632, 696`** : `serde_json::to_string_pretty` sans `preserve_order` réordonne alphabétiquement `Languages.json` (1,3 Mo) à la première sauvegarde, contrairement au commentaire qui promet l'inverse (`indexmap` est déjà là : activer la feature).

**B10 — Divers.** `xml/mod.rs:848` : une entité inconnue (`&nbsp;`) vide silencieusement la description. Éléments `<file …></file>` (forme non vide) ignorés, seuls les `Event::Empty` sont traités (714-745) : `reader.config_mut().expand_empty_elements = true` règle le cas. `moduleDependencies`, `dependencies` imbriquées, `alwaysInstall`, `installIfUsable`, `order` ne sont pas modélisés et sont supprimés à l'enregistrement d'un FOMOD tiers — au minimum avertir au chargement. `build.rs:9` teste l'OS hôte et non la cible (`CARGO_CFG_TARGET_OS`). `update.rs:92-113` : `is_newer` ignore le suffixe, donc un build `2.0.0-dev` ne verra jamais la sortie de `2.0.0`. Chemins d'images `fomod\images\x.png` non normalisés dans 3 endroits (2595, 3364, `preview.rs:729`) alors qu'ils le sont ailleurs : sous Linux/macOS l'image n'apparaît pas. Filtres de fichiers proposent `bmp` mais la feature `image/bmp` n'est pas compilée.

### 1.2 Travail inutile à chaque frame

| Où | Quoi | Correctif |
|---|---|---|
| 5394-5395 `commit_active()` | clone profond de tout le `Ximod` à chaque frame, en permanence | ne committer qu'au changement d'onglet/fermeture/sortie (déjà fait par `switch_doc`, `close_all_fomods`, `save_all_modified`) ; `render_fomod_tabs` lit l'état vivant pour l'onglet actif |
| 2898, 3456, 3650, 4080 | `get_all_flags()`, `get_all_flag_values()`, `get_all_dependency_names()` : 3-4 parcours complets du projet avec clones et dédoublonnage O(n²) | cache invalidé par un compteur de révision, ou calcul seulement quand le champ a le focus |
| `components/mod.rs:154-156` | `path.is_file()` (un `stat`) + `format!("file://…")` par image affichée par frame | mémoriser `(PathBuf, bool, String)` |
| 1011-1091 `sync_fonts` | si Propriétés/Traduction/Drapeaux ouvert : parcourt les **6 777** langues de `Languages.json` avec `wanted.contains()` → ~460 000 comparaisons par frame | précalculer la liste des polices distinctes une fois ; recalculer seulement si `(locale, pays, trans_font, show_*)` change |
| `properties.rs:361-424` | clone des 6 777 langues + `to_lowercase()` + rendu de toutes les lignes sans virtualisation | cache des indices filtrés + `ScrollArea::show_rows` (déjà fait dans `flag_picker.rs:297`) |
| `flag_picker.rs:195-230` | 244 `is_file()` par frame | liste calculée à l'ouverture |
| `xml_editor.rs:115-184` | coloration + `layout_job` de **chaque ligne** par frame, sans culling ; en édition : parse complet + validation schéma par frame (208-246) | cache par `(révision, largeur, thème)` ; recalcul sur `response.changed()` ; `byte_to_line_col` quadratique → table des débuts de ligne |
| `translation.rs:1217-1263` | 327 lignes posées à chaque frame avec 2 clones par ligne | `egui_extras::TableBuilder` virtualisé (jamais utilisé dans le crate alors qu'il est déjà dans les dépendances) |
| `preview.rs:357-374, 545` | `compute()` appelé 2× par frame ; `compute_install` (clone + tri) par frame sur la page récapitulative | un seul appel ; cache à l'entrée dans `finished` |
| `condition_editor.rs:52` | `analyze()` reconstruit le `BTreeMap` complet par frame | cache par révision |
| 1861, 1450 | clone du rapport de validation / du diff entier par frame | emprunt de champ |
| 2072-2113 | 27 `i18n.t()` + clone de `recent_files` par frame, menu fermé ou non | mineur, mais symptomatique (voir 1.4) |

Ce qui est bien : aucune relecture/re-parse XML par frame, pas de re-sérialisation dans l'aperçu, bundles Fluent non reconstruits, polices rechargées seulement si l'ensemble change.

### 1.3 E/S bloquantes sur le thread UI

- **Export zip/7z** (1315-1321 → `export.rs`, `archive.rs`) : synchrone dans le handler de menu. Fenêtre gelée sans progression ni annulation, plusieurs minutes pour un mod de textures en LZMA2. De plus `export.rs:58` charge chaque fichier entier en RAM (`fs::read`) : un BA2 de 3 Go = pic de 3 Go. Utiliser `io::copy` + `FileOptions::large_file(true)` (sinon échec > 4 Gio). Le modèle thread + `mpsc` + `request_repaint` existe déjà dans `update.rs`.
- **Validation complète** (1705-1772) : un `WalkDir` du dossier racine, puis **un `WalkDir` par source de type dossier** dans `detect_conflicts`, puis `image_dimensions` par image. Mutualiser un seul parcours et passer en thread.
- **Optimisation d'image** (`media.rs:82-105`) : décodage + Lanczos3 + ré-encodage JPEG avec perte, écrasement en place non atomique, sur le thread UI.
- **Démarrage Windows** (`main.rs:89-125`) : 2 processus PowerShell synchrones avant toute fenêtre, dont un pour un DPI jamais lu (`screen_info.dpi` n'a aucun lecteur). `GetSystemMetrics` suffit (feature `Win32_UI_WindowsAndMessaging` déjà activée). Puis le splash bloque 2,5 s avant même de commencer l'initialisation — lancer l'init (JSON 1,8 Mo, i18n, polices) en parallèle du splash.
- `fonts.rs:89-122` : à chaque changement de l'ensemble voulu, **toutes** les polices sont relues du disque (jusqu'à 68 fichiers). Cache des octets.
- `update.rs` : le thread ne réveille pas l'UI → le bandeau de mise à jour n'apparaît qu'au prochain mouvement de souris. Passer `ctx.clone()` et appeler `request_repaint()`.

### 1.4 Structure de `main_window.rs`

5 546 lignes, `XimodApp` a 116 champs `pub`. Fonctions > 200 lignes : `render_settings_dialog` (676, closure imbriquée sur 7 niveaux), `render_menu_bar` (334), `render_conditional_tab` (306), `render_groups_panel` (285), `render_info_tab` (226), `render_plugin_dependencies` (222).

Duplications à factoriser :
- **table de fichiers ×3** (3746, 3858, 4158) → `fn files_table(ui, files: &mut Vec<InstallFile>, sel: &mut Option<usize>, …)` ;
- **éditeur de dépendances ×3** (2877, 3627, 4059) avec trois triplets `temp_dep_*`, `temp_pdep_*`, `temp_vdep_*` → `struct DepEditorState` ;
- **invite Oui/Non/Annuler ×2**, **pagination d'onglets ×2**, **`optimize_header_image` / `optimize_plugin_image`** quasi identiques ;
- **gabarit de fenêtre libre ×9** (`free_viewport_builder` + `show_viewport_immediate` + `record_win_geom` + Échap + `free_window_closed`) ;
- **12 index `current_*`** dupliqués entre `XimodApp` et `DocState`, recopiés à la main dans 4 fonctions → une `struct Selection` `Copy` ;
- **recherche de dossiers d'assets ×7** (`i18n`, `data` ×2, `fonts`, `games`, `main`, `icon`) → `assets::locate(rel)` avec `OnceLock`.

Chaînes d'index : 64 × `self.ximod.steps[step_idx]`, ~30 × la chaîne complète jusqu'à `plugins[plugin_idx]`. Accesseurs sur le **modèle** pour ne pas emprunter tout `self` :
```rust
impl Ximod {
    pub fn plugin_mut(&mut self, s: usize, g: usize, p: usize) -> Option<&mut Plugin> {
        self.steps.get_mut(s)?.plugin_groups.get_mut(g)?.plugins.get_mut(p)
    }
}
```
Cela supprime aussi les gardes manuelles et le motif « cloner le champ, éditer la copie, réécrire » (2779, 3019, 3300, 3781…) — `ui.text_edit_singleline(&mut self.ximod.steps[si].name)` compile directement, le code le fait déjà pour `self.ximod.name` en 2465.

~300 lignes `let label_x = self.i18n.t("…")` en tête des fonctions de rendu, uniquement pour contourner l'emprunt de `self` : passer `i18n: &I18n` en paramètre (emprunt de champ disjoint), ou faire renvoyer `Cow<str>` à `t()`.

Typage par chaînes : `Dependency.dep_type: String` vaut `"flag"`/`"file"` et est comparé par `==` dans 6 fichiers alors que l'enum `DependencyType` existe et n'est jamais utilisée. Idem `pattern_type`. `DependencyPattern::default()` donne `pattern_type = ""` mais `new()` donne `"Optional"`.

Découpage proposé : `ui/app.rs` (struct, `update`), `ui/docs.rs`, `ui/menu.rs`, `ui/tabs/{info,steps,required,conditional}.rs`, `ui/dialogs/{settings,about,script,confirm}.rs`, `ui/validation.rs`, `ui/widgets/{files_table,dependency_editor,free_window}.rs`.

### 1.5 Build et dépendances

- Dépendances déclarées **sans aucun usage** : `uuid`, `thiserror`, `fluent-bundle` (les types viennent de `fluent`), `objc`, `cocoa` (le splash macOS est un `sleep`). `rand` ne sert qu'à `$RANDOM$` (remplaçable), `quick-xml` n'a pas besoin de `serialize`, `zip` avec `deflate` embarque `zopfli` (`deflate-flate2` suffit), `image/ico` n'est utile que sous Windows.
- `[profile.dev]` à `opt-level = 0` partout : `image`, `resvg`, `lzma-rust` sont très lents en debug. Ajouter `[profile.dev.package."*"] opt-level = 2`. En release, `lto = "thin"` accélère nettement les builds pour un gain quasi identique.
- `rust-version = "1.98.1"` : l'édition 2024 n'exige que 1.85. Une MSRV aussi haute bloque les toolchains de distribution sans raison technique.
- Aucune table `[lints]`. Aucun `cargo test`/`clippy`/`fmt --check` en CI (seul workflow : build release sur tag). `Cargo.toml.bak` traîne à la racine.
- `eframe` sans `accesskit` : aucun support des lecteurs d'écran, à décider consciemment.

### 1.6 Tests

60 tests, 0 test d'intégration. Non couverts : l'aller-retour XML complet (3 tests seulement — c'est ce qui aurait révélé B1), `ui/translation.rs` (1 301 lignes, 0 test, fonctions pures `find_tokens`/`analyze`/`reconstruct` facilement testables), `xml_highlight.rs` (garantie octet-pour-octet non testée), `main_window.rs` (0 test : cycle multi-documents, invariants d'index), i18n (aucun test de parité des clés entre les 33 `.ftl`), `export.rs`, `config.rs` (aller-retour scripts). Les tests écrivent dans `temp_dir()/ximod_test*` à nom fixe : deux exécutions concurrentes se marchent dessus (`tempfile` en dev-dependency).

### 1.7 Clippy

133 avertissements, dont 104 `collapsible_if` (stylistiques, l'édition 2024 autorise `if let … && let …`). `cargo clippy --fix` en applique 119 automatiquement. Les 14 restants : `needless_range_loop` (lié à B3), `should_implement_trait` sur 7 `from_str` inhérents (dont certains sensibles à la casse et d'autres non : `type="selectany"` devient silencieusement `SelectAny`, `name="required"` devient `Optional`), `ptr_arg`, `derivable_impls`, `manual_div_ceil`.

---

## 2. Interface utilisateur

### 2.1 État actuel en bref

Fenêtre 1280×800 : barre de menus (File / Options / Help), bandeau de mise à jour, bande d'onglets de documents, quatre onglets (Mod Info / Install Steps / Required / Conditional), barre d'état avec un `status_message` unique. Pas de `SidePanel`, pas de barre d'outils (le helper `toolbar()` existe dans `components` mais n'est jamais appelé). Neuf fenêtres OS indépendantes (Settings, About, Script, Translation, XML, Preview, Validation, Properties, Flag picker) et trois `egui::Window` internes (Templates, Compare, Condition editor) : deux mécanismes de modalité qui coexistent. Thème `Visuals::dark()/light()` brut, seule personnalisation `item_spacing = (8, 6)`, 48 `Color32::` codés en dur, 5 PNG « glossy » 32×32 + glyphes texte `➕ ➖ « » ⚑ ●`. 18 infobulles au total, aucune n'explique un concept FOMOD. Pas d'undo/redo. Raccourcis : Ctrl+N/O/Shift+O/S/,/Q et F1 (qui ouvre About, pas le manuel).

### 2.2 Ce qui gêne vraiment l'utilisateur

**Pertes de données et plantages.** B3 ci-dessus. « Yes » à l'invite de sortie enregistre tous les documents… sauf ceux sans dossier racine (992), puis ferme quand même (5257). Suppression d'un groupe avec tous ses plugins, d'un plugin, d'un ensemble conditionnel : sans confirmation ni annulation (seule l'étape est confirmée). Échap dans l'éditeur XML en mode édition jette les modifications sans demander (`xml_editor.rs:420`). Fermer l'éditeur de traduction perd les saisies.

**Échecs silencieux.** Ctrl+S sans dossier racine ne fait rien (2028) et Save est grisé sans explication — un débutant fait New, remplit, enregistre : rien. Pas d'« Enregistrer sous ». Fichier ou image choisi hors racine : ignoré sans message (2562, 3338, 3800, 3907) alors que la clé `msg-file-outside-root` existe et n'est jamais utilisée. Les raccourcis globaux sont désactivés dès qu'une fenêtre outil non modale est ouverte (2016) — Ctrl+S ne marche plus avec le rapport de validation ouvert. Les réglages Window Width/Height sont sans effet (écrasés chaque frame en 5527). « Browse... » du dossier racine change le chemin mais ne charge pas le FOMOD existant (2451).

**Navigation confuse.** Jusqu'à cinq niveaux d'« onglets » visuellement identiques (`selectable_label`) : documents, sections, étapes, patterns de plugin, patterns conditionnels. Aucune vue d'ensemble de l'installeur : pour voir une option il faut cliquer étape → groupe → plugin, avec des listes de 80-120 px de haut même en 1280×800 et une pagination par 8 qui masque les étapes au-delà. Titres de section trompeurs : « Group Name: » coiffe la liste des groupes, « Plugin Name: » la liste des plugins, « Flag Name: » la section drapeaux, « Source » la section fichiers. Ajouter une étape la sélectionne, ajouter un groupe ou un plugin non (le panneau droit reste sur « Select a group first »). Sans étape, l'onglet n'affiche qu'un `➕` sans libellé ni infobulle. Le menu Options est un fourre-tout où Preview et Validate (actions centrales) côtoient Translation et Properties (outils de contributeur).

**Vocabulaire.** `SelectExactlyOne`, `CouldBeUsable`, `And`/`Or` affichés bruts et non traduits (3034, 3394, 2866) alors que les clés `preview-sel-*` traduites existent et ne servent que dans l'aperçu. « Plugin » et « option » désignent la même chose (pour un moddeur Bethesda, « plugin » = .esp). Chaînes en dur : « Nom » en français dans `properties.rs:301`, « Step {} », « Pattern of », « Conditional set » dans `condition_editor.rs`. 11 clés FTL mortes.

**Aspect.** Thème egui par défaut sans identité. Iconographie hétérogène (PNG années 2000 + glyphes Unicode, la croix rouge sert à la fois à fermer un onglet, vider un filtre et supprimer un récent). Tables de fichiers en `Grid` sans colonnes redimensionnables où seule la cellule « Type » sélectionne la ligne. `ConfirmDialog` « Save anyway » affiche toute la liste dans un seul label, fenêtre non redimensionnable sans défilement. Point `YELLOW` illisible en thème clair. Après « Optimiser l'image », egui continue d'afficher l'ancienne image (aucun `forget_image` dans le code ; les caches d'`egui_extras` n'ont aucune éviction, donc chaque image vue reste en RAM en 3 copies jusqu'à la fermeture).

### 2.3 Propositions, par ordre de rapport impact/effort

Effort : S ≤ 1 jour, M = 2-5 jours, L > 1 semaine. Crates vérifiées compatibles egui 0.29 : `egui_dnd 0.10`, `egui-notify 0.17`, `egui-toast 0.15`, `egui-phosphor 0.7`, `egui_material_icons 0.1`, `egui_dock 0.14`, `egui_tiles 0.10`, `egui-modal 0.5`, `egui_commonmark 0.18`, `egui-file-dialog 0.7`, `catppuccin-egui 5.x`. Incompatibles : `egui_ltreeview` (≥ 0.31), `egui::Modal` natif (≥ 0.30).

| # | Proposition | Effort | Contenu |
|---|---|---|---|
| U1 | **Flux d'enregistrement sûr** | S | Ctrl+S sans racine → `pick_folder` puis enregistre ; « Enregistrer sous… » (Ctrl+Shift+S) ; titre de fenêtre `● NomDuMod — XIMOD Architect` via `ViewportCommand::Title` ; à la sortie, traiter les documents sans racine ; ne plus désactiver les raccourcis pour les fenêtres non modales |
| U2 | **Toasts et erreurs visibles** | S | `fn notify(level, msg)` → `egui-notify` (succès 3 s, erreur persistante) ; barre d'état pour l'info durable (racine, nb étapes/options, compteur d'erreurs cliquable) ; brancher les échecs silencieux sur `msg-file-outside-root` / `msg-no-root-selected` |
| U3 | **Annuler / rétablir** | M | pile de snapshots `Vec<Ximod>` par document (bornée à 100 ; `Ximod` est déjà `Clone`), `fn touch()` qui fusionne les frappes d'un même champ, Ctrl+Z/Y, menu Édition ; les suppressions peuvent alors se passer de confirmation avec un toast « Groupe supprimé — Annuler » |
| U4 | **Libellés humains, infobulles, i18n des énumérations** | S | `label(i18n, SelectionType)` → « Un seul choix obligatoire », etc. ; réutiliser `preview-sel-*` comme infobulles des ComboBox ; icône (i) sur chaque concept FOMOD ; `hint_text` dans les champs vides ; corriger les titres de section ; un seul terme « Option » ; sortir les chaînes en dur vers le FTL |
| U5 | **Barre d'outils + menus réorganisés + modales unifiées** | S-M | Barre : Nouveau, Ouvrir, Enregistrer │ Annuler, Rétablir │ Valider, Aperçu, Exporter. Menus : Fichier / Édition / Projet (Valider, Aperçu, Conditions, Comparer, Modèles, XML) / Outils (Scripts, Traduction UI, Traduction FOMOD, Base pays-langues) / Aide (Manuel sur F1, Mises à jour, À propos). Un composant modal unique avec voile, Entrée/Échap, `ScrollArea`. Supprimer `close_menu_if_pointer_left` |
| U6 | **Thème et iconographie** | M | module `theme.rs` : palette sémantique (accent, success, warning, danger, muted) pour les deux thèmes ; `rounding` 6/8, `button_padding (10,6)`, `interact_size.y 28` ; remplacer les 48 couleurs en dur ; police d'icônes `egui-phosphor` fusionnée dans `fonts::build_defs` à la place des PNG et glyphes |
| U7 | **Écran d'accueil** | S-M | quand `active_is_pristine()` : trois cartes (Nouveau vide / Nouveau depuis un dossier / Ouvrir), liste des récents (grisés si le dossier n'existe plus), zone de dépôt, liens vers les manuels PDF ; choix de langue inline au premier lancement au lieu de la fenêtre Settings forcée ; exposer les `WizardOptions` (aujourd'hui figées par `default()`) |
| U8 | **Panneau de problèmes cliquable** | M | `struct Issue { severity, message, target: Option<Selection> }` (les types sources `RefLoc`, `ValidationError` portent déjà la localisation) ; `TopBottomPanel::bottom` repliable avec icône de sévérité, filtres, clic → sélection de l'élément ; revalidation différée après modification (contrôles disque seulement à la demande) ; badges dans l'arbre ; « Save anyway » réduit à « 3 erreurs, 2 avertissements — Voir / Enregistrer quand même » |
| U9 | **Arbre projet à gauche + inspecteur à droite** | L | `SidePanel::left` redimensionnable avec l'arbre Mod → Étapes → Groupes → Options / Fichiers requis / Ensembles conditionnels ; icône, badge (type de sélection, nb fichiers, alerte) et menu contextuel par nœud (Ajouter, Dupliquer, Renommer, Supprimer, Enregistrer comme modèle) ; `CentralPanel` = inspecteur du nœud sélectionné. Remplace les 12 index par `enum Selection { Info, Step(usize), Group(usize,usize), Plugin(usize,usize,usize), Required, CondSet(usize) }`, ce qui supprime B3 à la racine. Socle de U8, U10, U12 |
| U10 | **Réordonnancement par glisser-déposer** | M | poignée sur chaque ligne (`egui_dnd` pour les listes plates, `dnd_drag_source`/`dnd_drop_zone` natifs pour déplacer une option entre groupes) ; Alt+↑/↓ ; « Dupliquer » ; étendre aux fichiers (l'ordre et la priorité comptent) |
| U11 | **Aperçu intégré fidèle MO2/Vortex** | M | la logique de `preview.rs` est déjà bonne ; il manque : bandeau avec image d'en-tête/nom/auteur/version, liste des étapes à gauche, badges de type traduits, préréglage visuel MO2 ou Vortex, bascule Éditer/Aperçu (F5), récapitulatif final en `TableBuilder` avec conflits marqués |
| U12 | **Tables de fichiers et miniatures** | M | `egui_extras::TableBuilder` avec colonnes redimensionnables, sélection de ligne entière, source et priorité éditables, multi-sélection, Suppr, zone de dépôt visible (`hovered_files`) ; vue « cartes » d'un groupe avec miniature/nom/badge ; `ctx.forget_image(uri)` après optimisation ; vignettes 2× la taille affichée plutôt que pleine résolution |

Gains rapides complémentaires : dialogues de fichiers avec `set_directory(root)` et `set_title` ; retirer Window Width/Height des réglages, prévisualiser le thème en direct, remplacer la navigation de focus manuelle (4291-4472) par le focus natif ; splash non bloquant et position de fenêtre mémorisée ; XML en lecture seule sélectionnable/copiable + confirmation avant de fermer en édition ; éditeur de traduction UI : filtre « non traduits », recherche, compteur, avertissement à la fermeture ; Templates : permettre d'enregistrer groupe et plugin (le modèle le prévoit, `templates.rs:37-43`), suppression, aperçu.

Ordre suggéré : U1, U2, U4, U5 (tous S, sans refonte) → U6 (thème, avant U9 pour ne pas restyler deux fois) → U9 avec le nouveau modèle de sélection, en y greffant U3, U8, U10 → U7, U11, U12.

---

## 3. Nouvelle fonction : traduction de FOMOD existants

### 3.1 Ce qui existe déjà

`src/ui/translation.rs` (1 301 lignes) est l'éditeur de traduction **de l'interface de l'application** (fichiers Fluent `main.ftl`, endonymes, `Languages.json`) — il ne touche jamais `info.xml` ni `ModuleConfig.xml`. Réutilisable : le gabarit de fenêtre libre, `handle_trans_keys` (navigation clavier, à passer en `pub(crate)`), l'idée analyse/reconstruction des jetons protégés, la police d'aperçu `fonts::PREVIEW_FAMILY`. Il faudra renommer l'entrée de menu « Traduire l'interface… » pour lever l'ambiguïté.

### 3.2 Décision d'architecture : un patcheur XML sans perte

Le passage `load_ximod → save_ximod` **ne doit pas servir** à écrire un FOMOD traduit : il perd CDATA, `moduleDependencies`, dépendances imbriquées, `alwaysInstall`/`installIfUsable`, les attributs inconnus, force `order="Explicit"` (si l'original trie par nom, traduire les noms change l'ordre affiché), impose UTF-8 BOM et sa propre indentation, et pour l'instant double-échappe (B1). Un auteur de mod dont on traduit le FOMOD ne doit pas retrouver son XML réécrit.

Nouveau `src/xml/patch.rs` : un seul parcours quick-xml Reader → Writer où tous les événements sont recopiés tels quels et seules les valeurs ciblées sont remplacées. Le même parcours sert à l'extraction et à l'application, ce qui garantit la symétrie des clés. Propriété à tester : appliquer une table vide redonne l'original à l'octet près. Le modèle `Ximod` ne sert qu'à l'affichage (contexte, aperçu traduit).

### 3.3 Ce qui est traduisible

| Champ | XML | Statut |
|---|---|---|
| nom du mod | `<Name>` (info) et `<moduleName>` (config) | traduisible, **deux clés** (ils peuvent différer dans un FOMOD tiers ; bouton « synchroniser ») |
| description du mod | `<Description>` | traduisible, multiligne |
| auteur, site | `<Author>`, `<Website>` | verrouillés par défaut, déverrouillables (ajout du traducteur, page de la traduction) |
| version, catégorie, jeu | | jamais |
| nom d'étape, de groupe, d'option | `installStep/@name`, `group/@name`, `plugin/@name` | traduisible |
| description d'option | `plugin/description` | traduisible, multiligne |
| images | `moduleImage/@path`, `image/@path` | chemins ; « image localisée » en option |
| flags (nom et valeur), dépendances, chemins source/destination, types, énumérations | | **jamais** — les valeurs de flags sont comparées par égalité stricte (`preview.rs`, `dep_satisfied`) : les traduire casse l'installateur |

### 3.4 Modèle de données (`src/models/translate.rs`, serde, sans i18n)

```rust
pub enum TField { InfoName, InfoAuthor, InfoWebsite, InfoDescription,
                  ModuleName, StepName, GroupName, PluginName, PluginDescription }
pub enum TStatus { Untranslated, Translated, Auto /*mémoire ou MT, à relire*/,
                   Fuzzy /*source modifiée*/, Obsolete }

pub struct TUnit {
    pub key: String,      // "config/step[2]/group[0]/plugin[1]/name", "info/Description"
    pub field: TField,
    pub source: String,   // dé-échappé
    pub target: String,   // vide = non traduit → la source est écrite à l'export
    pub status: TStatus,
    pub locked: bool,
    pub context: String,  // fil d'Ariane : "Textures › Résolution › 4K"
    pub note: String,
}

pub struct TranslationDoc {
    pub format: u32, pub tool: String,
    pub source_lang: String, pub target_lang: String, pub translator: String,
    pub mod_name: String, pub mod_version: String,
    pub source_fingerprint: SourceFingerprint, // crc32 + taille + encodage des 2 XML (archive.rs a déjà un crc32)
    pub created: String, pub updated: String,
    pub units: Vec<TUnit>, pub obsolete: Vec<TUnit>,
    pub image_overrides: IndexMap<String, String>,
    pub export: ExportPrefs,
}

pub enum TIssue {
    EmptyTarget, WhitespaceOnly, EdgeWhitespaceMismatch,
    TokenMismatch { missing: Vec<String>, extra: Vec<String> }, // \n littéral, URL, nombres, *.esp, <balises>
    NewlineInName, ControlChars, LengthRatio { ratio: f32 }, TooLongName { chars: usize },
    IdenticalToSource, InconsistentDuplicate { other_key: String },
    GlossaryViolation { term: String }, CdataTerminator,
}
```

Mémoire de traduction et glossaire : `<config_dir>/translation_memory/<src>-<tgt>.json`, sur le modèle de `templates_dir()`.

### 3.5 Mise à jour quand le mod original change

`merge_update(old, new_units) -> (TranslationDoc, UpdateReport)` : même clé + même source → repris ; source identique ailleurs (élément déplacé) → repris ; même clé + source différente → `Fuzzy` avec l'ancienne cible en suggestion ; aucune correspondance → `Untranslated` puis pré-remplissage par la mémoire (`Auto`) ; unités non consommées → `obsolete`. Rapport `{ new, changed, moved, removed, unchanged }`. `compare.rs` ne suffit pas seul (il ne compare ni les noms de groupes ni les descriptions d'options) mais sa stratégie d'appariement par nom sert de secours quand les indices ont bougé, et son rendu `+ / − / ~` est réutilisable.

### 3.6 Interface (`src/ui/fomod_translation.rs`)

État dans une `struct FomodTranslationState` (deux champs seulement dans `XimodApp`, comme `preview: PreviewState`), fenêtre libre `"ximod_fomod_translation"` 1150×720, sortie de l'état par `std::mem::take` comme pour `condition_editor`.

- **Bandeau** : chemin source, « Ouvrir un dossier… », « Depuis le projet actif » (exige un projet enregistré), plus tard « Ouvrir une archive… » ; langues source/cible (ComboBox alimentées par `i18n.languages()`) ; traducteur ; Enregistrer / Exporter ▾ / Valider / « Mettre à jour depuis une nouvelle version… » ; badge d'encodage détecté ; avertissement si `order ≠ Explicit`.
- **Barre d'outils** : filtres Toutes / Non traduites / À relire / Avec problèmes / Verrouillées ; type (noms / descriptions / métadonnées) ; recherche ; `ProgressBar` « 123 / 250 » ; actions : copier source → cible, appliquer la mémoire, propager aux identiques, traduction automatique de la sélection (désactivée par défaut).
- **Centre** : `egui_extras::TableBuilder` `.striped().resizable().sense(click)`, colonnes n° / statut / contexte / source / cible / problèmes, `heterogeneous_rows` (virtualisé), `TextEdit::singleline` en cellule pour les noms, extrait élidé pour les descriptions, `scroll_to_row` au clavier.
- **Panneau de détail bas** (redimensionnable) : source en lecture seule avec option « afficher espaces et `\n` » ; `TextEdit::multiline` cible en `PREVIEW_FAMILY` ; compteurs, problèmes de la ligne, suggestions de la mémoire (clic pour insérer), note, vignette de l'option via `ImageDisplay`.
- **« Aperçu traduit »** : ouvre le simulateur existant sur `apply_to_model(&ximod, &doc)` (il faut que `open_preview` accepte un `Ximod` en paramètre).
- Clavier : Tab/Maj+Tab, Ctrl+Entrée = valider et passer à la prochaine non traduite, Ctrl+S, Ctrl+F.

### 3.7 Sidecar et exports

Sidecar : `<racine>/fomod/translations/<ModName>.<lang>.ximod-translation` (JSON, écriture atomique). `verify.rs` ignore déjà `fomod/` ; il faut étendre `export::is_junk` à `*.ximod-translation`, `translations/`, `fomod_*`.

Modes d'export :
1. **Sur place** : écrase `fomod/info.xml` et `ModuleConfig.xml` après copie de sauvegarde horodatée.
2. **Dossier frère `fomod_<lang>/`** : zone de préparation.
3. **Paquet de traduction `ModName_FR.zip|7z`** prêt pour Nexus — **patch seul** par défaut (les deux XML + images localisées + LISEZMOI généré ; ne redistribue rien de l'auteur), ou **complet** avec avertissement sur les permissions. Nouvelle `archive::export_with_overrides(root, out, format, overrides, only)` qui s'appuie sur `collect_distribution_files` et **n'appelle pas** `save_ximod`.

Gabarit de nom `{name}_{LANG}` avec `{version}`, `{lang3}`, `{langname}`. Options : suffixer `<Name>`/`<moduleName>` (« [FR] »), ajouter le traducteur à `<Author>`, remplacer `<Website>`. Échanges : CSV (clé, contexte, source, cible) ; PO/XLIFF plus tard.

### 3.8 Cas limites pris en compte

UTF-16 LE/BE avec BOM (réécrire dans l'encodage d'origine), UTF-16 sans BOM et Windows-1252 (repli avec avertissement) ; CDATA préservé, entités toujours dé-échappées (ne jamais montrer `&amp;`), `&#10;` dans un attribut signalé (`NewlineInName`) ; vrais sauts de ligne conservés, séquences littérales `\n` traitées comme jetons à nombre égal ; doublons : une unité par clé, « propager » explicite et réversible ; `order` non explicite : option « figer l'ordre d'origine » cochée par défaut ; casse de `fomod/`, `ModuleConfig.xml` insensible (Linux) ; fichier manquant : n'extraire que ce qui existe ; polices : étendre `sync_fonts` à la langue cible (l'arabe fait partie des 33 locales — RTL d'egui à valider) ; projet ouvert modifié : demander d'enregistrer.

### 3.9 CLI et lot

```
ximod-architect translate extract <root> --lang fra [--source-lang eng] [-o FILE] [--csv FILE]
ximod-architect translate update  <root> <FILE.ximod-translation>
ximod-architect translate apply   <root> <FILE> [--mode inplace|sibling|package] [--full] [-o OUT] [--format zip|7z] [--strict]
ximod-architect translate status  <FILE>
ximod-architect translate import-csv <FILE> <CSV>
```
Code 3 avec `--strict` s'il reste du non traduit/fuzzy/bloquant. `BatchEntry` étendu avec `#[serde(default)] translations: Vec<BatchTranslation>` (rétro-compatible).

### 3.10 Traduction automatique (optionnelle, désactivée par défaut)

`trait MtProvider { fn translate(&self, texts: &[String], src: Option<&str>, tgt: &str) -> Result<Vec<String>, String>; }`, implémentations DeepL et LibreTranslate avec `ureq` (déjà présent, features `tls`+`json`), thread + `mpsc` comme `update::spawn_check`. Config : `MtProvider`, `MtEndpoint`, `MtApiKey` (clé en clair dans l'INI → accepter aussi une variable d'environnement). Consentement au premier usage, jetons masqués avant envoi, résultats en statut `Auto`, jamais d'appel implicite.

### 3.11 Effort et découpage

| Sous-partie | Effort |
|---|---|
| `models/translate.rs` | S |
| `xml/patch.rs` (encodage, extraction, application sans perte, tests de référence) | **L** — cœur du risque |
| `merge_update` + rapport | M |
| validation `TIssue` + jetons | S/M |
| fenêtre (bandeau, table, filtres, recherche, détail, clavier) | M/L |
| sidecar + export sur place / dossier frère | S |
| paquet zip/7z + LISEZMOI + gabarit + `is_junk` | M |
| dialogue de mise à jour | S/M |
| mémoire + glossaire | M |
| CLI + batch | M |
| traduction automatique | M |
| images localisées | M |
| CSV | S |
| aperçu traduit | S |
| i18n (~110 clés × 33 langues) + manuel EN/FR | M |

Lots : **T1** noyau sans interface (modèle, patcheur, validation, merge, tests, CLI minimal — testable en CI) → **T2** interface + sidecar + export simple + menu + i18n → **T3** paquet Nexus, gabarit, dialogue de mise à jour, batch → **T4** mémoire, glossaire, propagation, CSV, aperçu traduit → **T5** optionnel : MT, images localisées, import d'archive, PO/XLIFF.

**Prérequis** : B1 et B4 (dé-échappement, CDATA) doivent être corrigés avant, puisqu'ils touchent l'ouverture de tout FOMOD existant.

---

## 4. Autres fonctions envisageables

Ordonnées par intérêt pour un moddeur Bethesda, en tenant compte de ce qui existe déjà dans le code. Les onze fonctions sont livrées dans la 2.0.0 (✅) ; le manuel les décrit aux § 8.14 à 8.25.

1. ✅ **Ouvrir un FOMOD depuis une archive .zip/.7z** (M) — les FOMOD arrivent de Nexus en archive ; c'est le point d'entrée naturel de la traduction. `zip` et `sevenz-rust` sont déjà des dépendances (`decompress_file` est même utilisé dans un test). Extraction partielle de `fomod/` pour traduire, complète vers un dossier de travail pour éditer. `.rar` : message explicite.
2. ✅ **Fidélité d'import des FOMOD tiers** (S pour l'alerte, M/L pour le support) — au chargement, détecter avec `validate::parse_tree` les constructions non modélisées et afficher « ce FOMOD contient des éléments que XIMOD supprimerait à l'enregistrement » ; puis modéliser `moduleDependencies`, dépendances imbriquées, `alwaysInstall`/`installIfUsable`, `order`.
3. ✅ **Sauvegardes tournantes, écriture atomique, récupération** (S/M) — B5 + sauvegardes horodatées (`chrono` présent), « Restaurer une version… » avec différentiel via `compare.rs`, sauvegarde automatique des documents modifiés, et le hook de panic de B3.
4. ✅ **Masters → conditions automatiques** (S/M) — `read_plugin_header` renvoie déjà `masters`, `author`, `light` mais n'est appelé que par la CLI `inspect`, jamais par l'interface. À brancher sur l'ajout de fichiers (`render_plugin_files`, `dragdrop`) : générer un `DependencyPattern` avec `Dependency::new_file(master, "Active")`, pré-remplir l'auteur, avertir si un master n'est ni dans le paquet ni dans la table des masters du jeu.
5. ✅ **Contrôle ESL / plugins légers** (S rapport, M complet) — `PluginHeader.light` existe. Rapide : signaler les incohérences extension/drapeau dans la validation. Complet : compter les nouveaux FormID pour vérifier l'éligibilité ESL (plages par jeu à vérifier).
6. ✅ **Simulateur enrichi** (M) — `preview.rs` liste déjà les fichiers installés mais sans développer les dossiers, sans tailles, sans écrasements par priorité ; `conflicts.rs` a déjà les règles de chemin final. Déplacer `compute`/`compute_install` vers `models/simulate.rs`, ajouter l'arbre final avec tailles, des scénarios nommés rejouables (`simulate <root> --scenario s.json`), la détection d'étapes/options inatteignables.
7. ✅ **Poids d'installation par option** (S) — « 4K : 2,1 Go » dans l'éditeur et l'aperçu, total par scénario ; `walkdir` présent, `conflicts.rs` parcourt déjà les sources.
8. ✅ **Générateur de description Nexus (BBCode/Markdown)** (S) — parcours de `Ximod` : liste des options, prérequis tirés des dépendances de fichiers et des masters, changelog depuis `compare::diff_projects` (son en-tête mentionne déjà cet usage), `games.nexus_slug_for` existant. Version traduite depuis un `TranslationDoc`.
9. ✅ **Éditeur de conditions réellement éditable + renommage de flag** (S renommage, M graphe, L constructeur) — `condition_editor.rs` est en lecture seule et sans navigation. Clic sur « défini par » / « utilisé par » → sélection ; renommer un flag partout ; signaler les valeurs testées mais jamais définies ; constructeur visuel « SI [flag|fichier] [=] [valeur] ET/OU … ».
10. ✅ **Lecture BSA/BA2** (M) — détecter que deux options livrent des archives contenant le même asset ; valider par aller-retour l'écriture BA2 expérimentale.
11. ✅ **Table des chaînes du projet courant** (S une fois T2 livré) — réutiliser la table de traduction pour relire/corriger en masse tous les noms et descriptions (rechercher/remplacer, doublons).

---

## 5. Feuille de route proposée

| Lot | Contenu | Effort | État |
|---|---|---|---|
| **C0 — Correctifs critiques** | B1, B2, B3 (+ hook de panic), B4, B5, B6, B7 ; test d'aller-retour XML ; `clippy --fix` ; retrait des 5 dépendances inutilisées ; `[profile.dev.package."*"]` | 1-2 j | ✅ |
| **C1 — Réactivité** | suppression du `commit_active` par frame ; caches autocomplétion/`sync_fonts`/Propriétés/éditeur XML ; export et validation en thread avec progression ; `forget_image` ; démarrage sans PowerShell | 3-4 j | ✅ |
| **U1 — Gains rapides UI** | U1, U2, U4, U5 | 3-4 j | ✅ |
| **T1-T2 — Traduction FOMOD** | noyau + interface | 2-3 sem. | ✅ |
| **U6 + U9 — Refonte visuelle et arbre/inspecteur** | avec U3 (undo), U8 (problèmes), U10 (drag) | 3-4 sem. | ✅ |
| **T3-T4 — Traduction : distribution et productivité** | | 1-2 sem. | ✅ |
| **C2 — Refactor `main_window.rs`** | découpage en modules, `Selection`, widgets partagés, tests multi-documents, CI test+clippy | 1 sem., peut s'étaler | ✅ |
| **F — Fonctions 1 à 8 de §4** | selon priorités | variable | ✅ |

Je recommande de commencer par **C0** : c'est court, et tant que B1/B2 ne sont pas corrigés, chaque FOMOD enregistré par l'application est légèrement abîmé.

*Bilan : les huit lots ont été exécutés dans cet ordre et constituent la version 2.0.0 (voir « État de réalisation » en tête).*
