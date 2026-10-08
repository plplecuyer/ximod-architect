# XIMOD Architect - translation metadata
# @language = fra
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Français
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Version { $version }

# Status messages
status-ready = Prêt
msg-save-success = FOMOD enregistré avec succès
msg-save-error = Erreur lors de l'enregistrement du FOMOD
msg-export-success = Archive de distribution créée ({ $count } fichiers) : { $path }
msg-export-error = Erreur lors de la création de l'archive de distribution: { $error }
msg-load-success = FOMOD chargé avec succès
msg-load-error = Erreur lors du chargement du FOMOD
msg-merge-success = FOMOD fusionné avec succès
msg-merge-error = Erreur lors de la fusion du FOMOD
msg-no-root-selected = Veuillez d'abord sélectionner un répertoire racine
msg-no-fomod-folder = Aucun dossier 'fomod' trouvé. Créer un ?
msg-file-outside-root = Le fichier est en dehors du répertoire racine

# Menu - File
menu-file = Fichier
menu-new = Nouveau
menu-open = Ouvrir dossier…
menu-open-file = Ouvrir fichier…
menu-save = Enregistrer
menu-recent = Récents
menu-exit = Quitter
menu-merge = Fusionner FOMOD…
menu-export = Exporter l'archive de distribution…
# Menu - Options
menu-options = Options
menu-settings = Paramètres…
menu-pre-save-script = Script pré-sauvegarde…
menu-post-save-script = Script post-sauvegarde…
menu-translation = Traduire l'interface…
# Menu - Help
menu-help = Aide
menu-check-updates = Rechercher les mises à jour…
menu-about = À propos

# Update check
update-checking = Recherche de mises à jour…
update-up-to-date = XIMOD Architect est à jour.
update-check-failed = Impossible de vérifier les mises à jour. Réessayez plus tard.
update-available-status = La version { $version } est disponible.
update-banner-text = XIMOD Architect { $version } est disponible.
update-download = Télécharger :
update-skip = Ignorer cette version
update-later = Plus tard

# Tabs
tab-info = Info Mod
tab-steps = Étapes d'Installation
tab-required = Installations requises
tab-conditional = Installations conditionnelles

# Info Tab
label-workspace = Espace de travail
label-root-dir = Répertoire racine :
label-mod-name = Nom du Mod :
label-author = Auteur :
label-version = Version :
label-game-name = Nom du jeu :
label-category = Catégorie :
label-url = Site web :
label-header-image = Image d'En-tête :
label-description = Description :
placeholder-select-dir = (Sélectionnez un répertoire)
placeholder-select-game = (Sélectionnez un jeu)

# Steps Tab
label-step-name = Nom de l'Étape :
label-group-name = Nom du groupe :
label-group-type = Type de groupe :
label-plugin-name = Nom de l'option :
label-plugin-desc = Description :
label-plugin-type = Type par défaut :
label-plugin-image = Image :
label-visibility = Conditions de visibilité
label-operator = Opérateur :

# Buttons
btn-browse = Parcourir...
btn-clear = Effacer
btn-add = Ajouter
btn-remove = Supprimer
btn-add-step = Nouvelle étape
btn-delete-step = Supprimer étape
btn-add-group = Ajouter groupe
btn-remove-group = Supprimer groupe
btn-add-plugin = Ajouter une option
btn-remove-plugin = Supprimer l'option
btn-add-file = Ajouter fichier
btn-add-folder = Ajouter dossier
btn-remove-file = Supprimer
btn-add-flag = Ajouter flag
btn-remove-flag = Supprimer flag
btn-add-condition = Ajouter condition
btn-remove-condition = Supprimer condition
btn-add-dependency = Ajouter dépendance
btn-remove-dependency = Supprimer dépendance
btn-add-pattern = Nouveau pattern
btn-remove-pattern = Supprimer pattern
btn-save = Enregistrer
btn-cancel = Annuler
btn-ok = OK
btn-yes = Oui
btn-no = Non

# Condition/Dependency Labels
label-flag-name = Nom du flag :
label-flag-value = Valeur :
label-condition-type = Type :
label-condition-name = Nom :
label-condition-value = Valeur :
label-dep-type = Type de dépendance :
label-dep-name = Nom/Fichier :
label-dep-value = Valeur/État :

# Files
label-source = Source
label-destination = Destination
label-priority = Priorité
label-file-type = Type

# Destination groupée (affecter une destination à tout un groupe ou une page)
label-group-dest = Destination pour tout le groupe
label-page-dest = Destination d'installation (toute la page)
btn-apply-group-dest = Appliquer à toutes les options du groupe
btn-apply-page-dest = Appliquer à toutes les options de la page
group-dest-hint = Définir une seule destination d'installation pour tous les fichiers de toutes les options de ce groupe.
page-dest-hint = Définir une seule destination d'installation pour tous les fichiers de toutes les options de cette page (tous les groupes).
bulk-dest-nofiles = Aucun fichier à mettre à jour — ajoutez d'abord des fichiers aux options.
status-dest-applied = Destination appliquée à { $num } fichier(s).
preview-hidden-steps = { $num } étape(s) masquée(s) par les choix actuels.
label-files = Fichiers
label-dependencies = Dépendances

# Settings Dialog
settings-title = Paramètres
settings-tab-general = Général
settings-tab-recent-files = Fichiers récents
settings-language = Langue :
settings-theme = Thème :
settings-font-size = Taille de police :
settings-replace-newlines = Traiter les retours à la ligne dans les descriptions
settings-check-updates = Vérifier les mises à jour au démarrage
settings-max-recent = Fichiers récents max :
settings-window-width = Largeur fenêtre :
settings-window-height = Hauteur fenêtre :
settings-no-recent-files = Aucun fichier récent.

# Status messages for settings
status-settings-saved = Paramètres sauvegardés avec succès

# About Dialog
about-title = À propos de XIMOD Architect
about-description = Un outil multiplateforme de création d'installeurs FOMOD pour les mods de jeux Bethesda.
about-license = Sous licence MIT
about-copyright = © 2025-2026 Équipe XIMOD
about-credit = Portage Rust de l’outil original de Wenderer :

# Script Dialog
script-title = Éditer script
script-info = Les scripts sont exécutés avant ou après la sauvegarde. Vous pouvez utiliser les macros suivantes :
script-macros = Macros disponibles :
macro-modname = $MODNAME$ - Nom du mod
macro-modauthor = $MODAUTHOR$ - Nom de l'auteur
macro-modversion = $MODVERSION$ - Version du mod
macro-modroot = $MODROOT$ - Chemin du répertoire racine
macro-date = $DATE$ - Date actuelle (AAAA-MM-JJ)
macro-time = $TIME$ - Heure actuelle (HH:MM:SS)
macro-random = $RANDOM$ - Nombre aléatoire

# Plugin Dependencies
label-plugin-dependencies = Dépendances de l'option
label-default-type = Type par défaut :
label-pattern-type = Type de pattern :
label-pattern-operator = Opérateur de pattern :

# Conditional Files
label-pattern = Pattern

# Validation Messages
validation-no-name = Le nom du mod est requis
validation-no-steps = Au moins une étape ou un fichier requis est nécessaire
validation-empty-step = L'étape { $num } n'a pas de nom
validation-empty-group = L'étape { $step }, groupe { $group } n'a pas de nom
validation-no-plugins = L'étape { $step }, groupe « { $name } » n'a pas d'options

# File States
state-active = Actif
state-inactive = Inactif
state-missing = Manquant

# Confirmation
confirm-title = Confirmation
confirm-delete = Êtes-vous sûr de vouloir supprimer cet élément ?
confirm-discard = Vous avez des modifications non enregistrées. Les abandonner et continuer ?
confirm-unsaved = Vous avez des modifications non enregistrées. Voulez-vous enregistrer avant de fermer ?
confirm-save-issues = Le projet comporte les problèmes suivants :
confirm-save-anyway = Enregistrer quand même ?

# Errors
error-invalid-xml = Fichier XML invalide
error-parse-failed = Échec de l'analyse du FOMOD
error-write-failed = Échec de l'écriture du fichier
error-create-dir = Échec de la création du répertoire

# Default names (generated when creating new items)
default-step-name = Étape { $num }
default-group-name = Groupe { $num }
default-plugin-name = Option { $num }
pattern-label = Motif { $num }

# Selection prompts
msg-select-group-first = Sélectionnez d'abord un groupe.
msg-select-plugin-edit = Sélectionnez une option à éditer.
label-empty = (vide)
image-no-image = Aucune image

# File dialog filters
filter-images = Images
filter-xml = XML

# Dependency types
dep-type-flag = Flag
dep-type-file = Fichier

# Status bar
status-modified = Modifié

# Status messages (errors)
msg-settings-save-error = Erreur lors de la sauvegarde des paramètres
msg-script-save-error = Erreur lors de la sauvegarde du script

# Translation editor
trans-title = Éditeur de traduction
trans-source-lang = Langue affichée :
trans-target-lang = Langue à traduire :
trans-col-key = Clé
trans-col-source = Libellé
trans-col-target = Traduction
trans-saved = Traduction enregistrée
trans-save-error = Erreur lors de l'enregistrement de la traduction

# XML editor
xml-editor-title = Éditeur XML
xml-editor-edit = Modifier
xml-editor-apply = Valider
xml-editor-revert = Annuler
xml-editor-readonly = Lecture seule
xml-editor-editing = Édition — les onglets sont verrouillés
xml-editor-error = Erreur :
xml-editor-applied = Modifications XML appliquées
xml-editor-wellformed = XML bien formé
xml-editor-error-at = Ligne { $line }, colonne { $col } : { $msg }

# Country / flag picker
settings-country-name = Nom du pays :
settings-pick-country = Cliquez pour choisir votre pays
flags-title = Choix du pays
flags-filter = Filtre :
flags-none = Aucun drapeau trouvé

# Translation editor: country & font
trans-endonym = Nom endonyme du pays :
trans-font = Police :
trans-no-font = (aucune)
trans-browse = Parcourir…
trans-google-fonts = Google Fonts
trans-pick-country = Cliquez pour choisir le pays
trans-font-outside = La police doit d'abord être installée dans assets/fonts.
trans-font-dir-missing = Le dossier assets/fonts est introuvable.

# Translation submission
trans-lang-endonym = Nom endonyme de la langue :
trans-author = Auteur :
trans-submit = Envoyer…
trans-submit-hint = Crée une archive zip et ouvre un courriel prérempli
trans-data-updated = Données de référence mises à jour (Languages.json / Countries.json)
trans-package-ready = Archive prête :
trans-package-error = Création de l'archive impossible :

# ISO 639-3 requirement
trans-lang-not-iso = La traduction n'est possible que pour une langue disposant d'un code ISO 639-3.

# FOMOD installer preview
menu-preview = Prévisualiser l'installateur…
preview-title = Prévisualisation de l'installateur FOMOD
preview-refresh = Rafraîchir
preview-assumptions = Hypothèses fichiers
preview-details = Détails
preview-back = Précédent
preview-next = Suivant
preview-install = Installer
preview-close = Fermer
preview-restart = Recommencer
preview-summary-title = Fichiers qui seront installés
preview-empty = Aucun fichier ne serait installé.
preview-none-option = (aucun)
preview-invalid = Complétez les choix requis pour continuer.
preview-no-steps = Aucune étape visible ; voir le récapitulatif d'installation.
preview-select-hint = Sélectionnez une option pour voir sa description.
preview-col-source = Source
preview-col-dest = Destination
preview-col-priority = Priorité
preview-sel-exactlyone = Choisissez exactement une option.
preview-sel-atmostone = Choisissez au plus une option.
preview-sel-any = Choisissez un nombre quelconque d'options.
preview-sel-all = Toutes les options sont installées.
preview-sel-atleastone = Choisissez au moins une option.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Valider le FOMOD
validate-report-title = Validation du FOMOD
validate-ok = Aucun problème détecté. Le FOMOD est conforme au schéma.
xml-editor-schema-ok = Conforme au schéma ModConfig 5.0.
xml-editor-schema-issues = Problèmes de schéma :
schema-line-col = Ligne { $line }, col. { $col } : { $msg }
schema-wrong-root = Racine « { $found } » inattendue (attendu « { $expected } »).
schema-unknown = Élément inattendu « { $element } » dans « { $parent } ».
schema-missing = « { $parent } » doit contenir « { $child } ».
schema-needs-one = « { $parent } » doit contenir au moins un « { $child } ».
schema-too-many = « { $child } » ne peut apparaître qu'une fois dans « { $parent } ».
schema-missing-attr = L'attribut « { $attr } » est requis sur « { $element } ».
schema-bad-enum = Valeur « { $value } » invalide pour { $element }/@{ $attr } (attendu : { $allowed }).
schema-choose-one = « { $parent } » doit contenir exactement un parmi : { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Déplacer avant
reorder-after = Déplacer après

# Country / language database explorer (Properties)
menu-properties = Propriétés…
prop-title = Base pays / langues
prop-tab-countries = Pays
prop-tab-languages = Langues
prop-filter = Filtre :
prop-official-langs = Langues officielles
prop-spoken-langs = Langues parlées
prop-endonym = Endonyme du pays
prop-font = Police
prop-spoken-in = Parlée dans
prop-select-country = Sélectionnez un pays pour voir ses détails.
prop-select-lang = Sélectionnez une langue pour voir ses détails.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Ouvrir la page Nexus Mods du jeu

# Vérification des fichiers référencés (V2)
verify-no-root = Vérification des fichiers ignorée : aucun répertoire racine défini
loc-header = image d'en-tête
loc-required = fichiers requis
loc-conditional = ensemble conditionnel { $num }
loc-plugin = étape { $step }, groupe { $group }, option « { $plugin } »
verify-missing-file = Fichier introuvable : { $path } ({ $loc })
verify-missing-folder = Dossier introuvable : { $path } ({ $loc })
verify-missing-image = Image introuvable : { $path } ({ $loc })
verify-absolute = Chemin absolu (non portable) : { $path } ({ $loc })
verify-outside = Chemin hors du répertoire racine : { $path } ({ $loc })
verify-orphan = Fichier orphelin (référencé par aucune option) : { $path }
conflict-certain = Conflit de destination : « { $path } » est écrit par { $count } options ({ $locs }) — elles s'écrasent mutuellement.
conflict-potential = Conflit de destination possible : « { $path } » est visé par { $count } références ({ $locs }) — l'écrasement dépend de la sélection/des conditions.

# Onglets multi-FOMOD & alerte de fermeture (V2)
menu-close-fomod = Fermer le Fomod
menu-close-all-fomods = Fermer tous les Fomods
tab-untitled = (sans titre)
msg-drop-not-fomod = L'élément déposé n'est pas un FOMOD (dossier « fomod » introuvable)
exit-title = Modifications non enregistrées
exit-unsaved = Le fichier n'a pas été sauvegardé. Voulez-vous le sauvegarder ?
tab-close-hint = Fermer ce FOMOD
menu-new-from-folder = Nouveau depuis un dossier…
menu-templates = Modèles…
templates-title = Modèles réutilisables
templates-empty = Aucun modèle enregistré. Enregistrez l'étape sélectionnée ci-dessus pour en créer un.
templates-insert = Insérer
templates-save-step = Enregistrer l'étape sélectionnée
templates-name-hint = Nom du modèle (facultatif)
msg-wizard-success = Squelette créé depuis le dossier : { $num } option(s).
msg-wizard-error = Erreur : { $error }
msg-template-saved = Modèle enregistré : { $name }
msg-template-inserted = Modèle inséré dans le projet.
msg-template-no-step = Sélectionnez d'abord une étape pour l'enregistrer comme modèle.
msg-template-no-dir = Impossible de localiser le dossier des modèles.
msg-drop-assigned = { $added } source(s) ajoutée(s) à l'option ({ $rejected } hors racine ignorée(s)).
menu-compare = Comparer avec…
compare-title = Comparaison de FOMOD
compare-none = Aucune différence.
btn-optimize-image = Optimiser l'image
msg-image-optimized = Image d'en-tête optimisée.
msg-image-ok = Image d'en-tête déjà dans les limites.
msg-no-header-image = Aucune image d'en-tête à optimiser.
verify-image-large = Image trop grande ({ $width }×{ $height }) : { $path }
verify-image-format = Format d'image non pris en charge (.{ $ext }) : { $path }
verify-image-unreadable = Image illisible : { $path }
menu-condition-editor = Éditeur de conditions…
condeditor-title = Éditeur de conditions
condeditor-set-by = Défini par :
condeditor-used-by = Utilisé par :
condeditor-filedeps = Dépendances de fichiers
condeditor-empty = Aucun drapeau ni dépendance dans ce projet.
condeditor-orphan-set = défini mais jamais utilisé
condeditor-orphan-used = utilisé mais jamais défini
msg-img-optimized = Image optimisée.
msg-img-ok = Image déjà dans les limites.
msg-img-none = Aucune image à optimiser.
msg-crash-recovery = La session précédente s'est terminée de façon inattendue. Une sauvegarde de votre projet a été enregistrée dans { $path }
export-progress-title = Création de l'archive de distribution…
export-progress-files = { $done } / { $total } fichiers
msg-export-cancelled = Export annulé ; l'archive partielle a été supprimée.
verify-running = Vérification des fichiers sur le disque…
verify-stale = Remarque : le projet a été modifié pendant la vérification des fichiers ; relancez la validation.
prop-col-name = Nom
menu-save-as = Enregistrer sous…
menu-project = Projet
menu-tools = Outils
menu-manual = Manuel utilisateur
msg-manual-missing = Le manuel utilisateur (PDF) est introuvable à côté de l'application.
toolbar-new = Nouveau
toolbar-open = Ouvrir
toolbar-save = Enregistrer
toolbar-validate = Valider
toolbar-preview = Aperçu
toolbar-export = Exporter
dialog-choose-root = Choisissez le dossier racine du mod
exit-unsaved-docs = Non enregistrés : { $names }
status-summary = { $steps } étapes · { $options } options
section-groups = Groupes
section-options = Options
section-flags = Drapeaux de condition
section-files = Fichiers à installer
hint-group-type = Comment l'installateur laisse l'utilisateur choisir les options de ce groupe.
hint-default-type = Comment l'option est proposée quand aucun de ses motifs de dépendance ne correspond : requise, optionnelle, recommandée, inutilisable…
hint-operator = Toutes les conditions doivent être vraies (ET), ou une seule suffit (OU).
hint-flags = Les drapeaux sont des valeurs nommées définies par cette option quand elle est choisie. D'autres étapes ou options peuvent les tester pour s'afficher, se masquer ou devenir requises.
hint-plugin-dependencies = Motifs qui changent le type de l'option selon des drapeaux ou des fichiers présents dans le jeu : par exemple « Requise » quand un autre mod est installé.
hint-files = Fichiers et dossiers copiés dans le dossier Data du jeu quand cette option est choisie. La destination est relative à Data ; en cas de conflit, la priorité la plus haute l'emporte.
hint-visibility = Conditions à remplir pour que cette étape soit affichée. Laissez vide pour toujours l'afficher.
seltype-exactly-one = Un seul choix (obligatoire)
seltype-at-most-one = Au plus un
seltype-any = Nombre libre
seltype-all = Toutes (sans choix)
seltype-at-least-one = Au moins un
plugtype-required = Requise
plugtype-optional = Optionnelle
plugtype-recommended = Recommandée
plugtype-not-usable = Inutilisable
plugtype-could-be-usable = Peut-être utilisable
plugtype-required-hint = Toujours installée ; l'utilisateur ne peut pas la décocher.
plugtype-optional-hint = Proposée non cochée ; l'utilisateur décide.
plugtype-recommended-hint = Proposée cochée ; l'utilisateur peut la décocher.
plugtype-not-usable-hint = Affichée grisée, impossible à sélectionner.
plugtype-could-be-usable-hint = Sélectionnable, mais l'installateur avertit qu'elle risque de ne pas fonctionner.
op-and = Toutes les conditions (ET)
op-or = Une condition suffit (OU)
theme-dark = Sombre
theme-light = Clair
theme-system = Suivre le système
condeditor-setter-loc = Étape { "{step}" } / Groupe { "{group}" } / « { "{name}" } »
condeditor-pattern-of = Motif de « { "{name}" } » → { "{type}" }
condeditor-visibility-of = Visibilité de l'étape { "{step}" }
condeditor-cond-set = Ensemble conditionnel { "{num}" }
condeditor-needs = { "{ctx}" } (attend = { "{value}" })
condeditor-file-dep = { "{ctx}" } : fichier « { "{name}" } » ({ "{state}" })
menu-translate-fomod = Traduire un FOMOD…
ftr-title = Traduire un FOMOD
ftr-open-folder = Ouvrir un dossier de mod…
ftr-from-active = Depuis le projet actif
ftr-from-active-hint = Traduire le FOMOD du projet ouvert dans la fenêtre principale (il doit d'abord être enregistré).
ftr-no-fomod = Aucun FOMOD chargé.
ftr-encoding = Encodage des fichiers d'origine ; les fichiers traduits sont écrits avec le même encodage.
ftr-source-lang = De
ftr-target-lang = vers
ftr-lang-locked = (les langues sont figées une fois un FOMOD chargé)
ftr-translator = Traducteur :
ftr-save = Enregistrer la traduction
ftr-export = Exporter les fichiers traduits
ftr-export-sibling = Dans un dossier fomod_<langue>
ftr-export-sibling-hint = Écrit info.xml et ModuleConfig.xml traduits à côté du dossier fomod d'origine ; les fichiers d'origine ne sont pas touchés.
ftr-export-inplace = Par-dessus les fichiers d'origine
ftr-export-inplace-hint = Remplace fomod/info.xml et fomod/ModuleConfig.xml après une copie .bak horodatée de chacun.
ftr-force-explicit-order = Conserver l'ordre d'origine
ftr-warn-order = Les listes triées par nom (order="Ascending") seraient retriées selon les noms traduits par le gestionnaire de mods. Cette option force order="Explicit" pour que les options gardent leur ordre actuel.
ftr-update = Mettre à jour depuis le dossier
ftr-update-hint = Relit le FOMOD sur le disque et y fusionne la traduction : les chaînes nouvelles, modifiées et supprimées sont signalées.
ftr-preview-translated = Aperçu traduit
ftr-progress = { $done } / { $total } traduites
ftr-filter-all = Toutes
ftr-filter-untranslated = Non traduites
ftr-filter-review = À relire
ftr-filter-issues = Avec problèmes
ftr-filter-locked = Verrouillées
ftr-type-all = Tous les champs
ftr-type-names = Noms
ftr-type-descriptions = Descriptions
ftr-type-meta = Informations du mod
ftr-search-hint = Rechercher dans la source, la traduction ou le contexte…
ftr-next-untranslated = Prochaine non traduite
ftr-show-whitespace = Afficher espaces et sauts de ligne
ftr-discard-question = La traduction en cours comporte des modifications non enregistrées. Les abandonner et charger l'autre FOMOD ?
ftr-discard-yes = Abandonner
ftr-unsaved-close = La traduction comporte des modifications non enregistrées.
ftr-col-num = N°
ftr-col-status = { "" }
ftr-col-context = Contexte
ftr-col-source = Source
ftr-col-target = Traduction
ftr-col-issues = { "" }
ftr-empty-hint = Ouvrez un dossier de mod, ou chargez le projet actif, pour lister ses chaînes traduisibles.
ftr-empty-filter = Aucune chaîne ne correspond au filtre actuel.
ftr-select-row = Sélectionnez une ligne pour éditer sa traduction.
ftr-copy-source = Copier la source
ftr-clear-target = Effacer
ftr-lock = Ne pas traduire
ftr-lock-hint = Les chaînes verrouillées sont écrites telles quelles (auteur, site web, noms propres…).
ftr-note = Note :
ftr-status-untranslated = Non traduite
ftr-status-translated = Traduite
ftr-status-auto = Pré-remplie automatiquement — à relire
ftr-status-fuzzy = Le texte source a changé depuis la traduction — à relire
ftr-status-obsolete = N'existe plus dans le FOMOD
ftr-status-locked = Verrouillée (écrite telle quelle)
ftr-field-info-name = Nom du mod (info.xml)
ftr-field-module-name = Titre de l'installateur (ModuleConfig.xml)
ftr-field-author = Auteur
ftr-field-website = Site web
ftr-field-description = Description du mod
ftr-field-step = Nom d'étape
ftr-field-group = Nom de groupe
ftr-field-plugin = Nom d'option
ftr-field-plugin-desc = Description d'option
ftr-issue-empty = Traduction vide
ftr-issue-whitespace = La traduction ne contient que des espaces
ftr-issue-edge-whitespace = Les espaces en début ou en fin diffèrent de la source
ftr-issue-token = Les jetons protégés diffèrent — manquants : { $missing } ; en trop : { $extra }
ftr-issue-newline-name = Un nom ne peut pas contenir de saut de ligne
ftr-issue-control = Contient des caractères que XML ne peut pas stocker
ftr-issue-length = Longueur inhabituelle par rapport à la source (×{ $ratio })
ftr-issue-identical = Identique à la source
ftr-issue-duplicate = Même texte source traduit différemment dans { $key }
ftr-issue-cdata = La séquence ]]> n'est pas autorisée ici
ftr-load-error = Impossible de charger le FOMOD : { $error }
ftr-extracted = { $num } chaînes traduisibles trouvées.
ftr-sidecar-found = Traduction existante chargée et fusionnée : { $new } nouvelles, { $changed } modifiées, { $removed } supprimées.
ftr-saved = Traduction enregistrée dans { $path }
ftr-save-error = Impossible d'enregistrer la traduction : { $error }
ftr-save-first = Enregistrez d'abord le projet, puis traduisez-le.
ftr-export-success = { $count } chaînes écrites dans { $path }
ftr-export-error = Échec de l'export : { $error }
ftr-export-blocked = { $num } problèmes bloquants doivent être corrigés avant l'export.
ftr-export-stale = { $num } chaînes ont été ignorées car le FOMOD a changé ; utilisez « Mettre à jour depuis le dossier ».
ftr-update-report = Mise à jour : { $new } nouvelles, { $changed } modifiées, { $moved } déplacées, { $removed } supprimées, { $unchanged } inchangées.
menu-edit = Édition
menu-undo = Annuler
menu-redo = Rétablir
tree-title = Projet
tree-mod-info = Informations du mod
tree-steps = Étapes d'installation
tree-required = Fichiers requis
tree-conditional = Installations conditionnelles
tree-empty-steps = Aucune étape — cliquez sur + pour en ajouter une.
tree-duplicate = Dupliquer
tree-delete = Supprimer
tree-save-template = Enregistrer comme modèle…
tree-drop-hint = Déposer ici pour déplacer
cond-set-label = Ensemble conditionnel { $num }
inspector-empty = Sélectionnez un élément dans l'arbre du projet, ou ajoutez une étape pour commencer.
count-options = { $num } options
count-files = { $num } fichiers
msg-deleted-undo = Supprimé. Utilisez Annuler (Ctrl+Z) pour le restaurer.
problems-title = Problèmes
problems-errors = { $num } erreurs
problems-warnings = { $num } avertissements
btn-close = Fermer
ftr-export-package = En paquet de traduction (archive)
ftr-export-package-hint = Construit un .zip ou un .7z prêt à publier : info.xml et ModuleConfig.xml traduits plus un LISEZMOI (correctif seul), ou tout le mod avec les fichiers traduits (complet).
ftr-package-full = Mod complet
ftr-package-full-hint = Inclure tous les fichiers du mod dans l'archive, et pas seulement les deux XML traduits. Vérifiez que l'auteur autorise la redistribution.
ftr-package-name-template = Nom :
ftr-readme-patch = Cette archive contient la traduction en { $langname } de l'installateur de « { $name } » (fomod/info.xml et fomod/ModuleConfig.xml). Installez-la par-dessus le mod d'origine, ou laissez votre gestionnaire de mods la fusionner, pour que les fichiers traduits remplacent les originaux. Seuls les textes de l'installateur changent ; les fichiers du mod ne sont pas inclus. Réalisé avec XIMOD Architect.
ftr-readme-full = Cette archive contient « { $name } » avec son installateur traduit en { $langname } (fomod/info.xml et fomod/ModuleConfig.xml). Installez-la comme le mod d'origine. Seuls les textes de l'installateur ont été modifiés. Réalisé avec XIMOD Architect.
ftr-apply-memory = Remplir depuis la mémoire
ftr-memory-size = Mémoire de traduction : { $num } entrées pour cette paire de langues. Chaque traduction enregistrée y est ajoutée.
ftr-memory-applied = { $num } chaînes remplies depuis la mémoire de traduction (marquées « à relire »).
ftr-memory-suggestion = La mémoire propose :
ftr-use-suggestion = Utiliser
ftr-propagate = Propager aux identiques
ftr-propagate-hint = Copier cette traduction vers toutes les autres chaînes de même texte source encore non traduites.
ftr-propagated = { $num } chaînes identiques remplies.
ftr-csv-export = Exporter en CSV…
ftr-csv-import = Importer un CSV…
ftr-csv-imported = { $num } chaînes mises à jour depuis le fichier CSV.
ftr-csv-error = Erreur CSV : { $error }
ftr-glossary = Glossaire
ftr-glossary-source = Terme
ftr-glossary-target = Traduction
ftr-glossary-case = Casse
ftr-glossary-dnt = Conserver
ftr-glossary-add = Ajouter un terme
ftr-issue-glossary = Glossaire : « { $term } » n'est pas traduit comme attendu

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Ouvrir une archive…
filter-archive = Archives de mod (zip, 7z)
msg-archive-opened = Archive ouverte ({ $num } fichiers extraits) : { $path }
msg-archive-reused = Archive déjà extraite, réutilisation de { $path }
msg-archive-unsupported = Le format d'archive « .{ $ext } » n'est pas pris en charge ; extrayez-la d'abord avec 7-Zip (seuls .zip et .7z peuvent être ouverts).
msg-archive-error = Erreur à l'ouverture de l'archive : { $error }
msg-archive-no-fomod = Aucun dossier « fomod » trouvé dans l'archive ({ $path })
msg-archive-extracting = Extraction de l'archive…
ftr-open-archive = Ouvrir une archive de mod…
ftr-package-full-partial = Le mod a été ouvert depuis une archive dont seul le dossier fomod a été extrait ; un paquet complet nécessite le mod extrait.
info-module-deps = Prérequis du mod
info-module-deps-hint = Fichiers ou drapeaux requis par tout le mod avant que l'installateur ne démarre (moduleDependencies). Laissez vide s'il n'y en a pas.
info-header-advanced = En-tête avancé
info-title-position = Position du titre
info-title-colour = Couleur du titre
info-title-colour-hint = Attendu : six chiffres hexadécimaux (RRGGBB)
info-image-show = Afficher l'image d'en-tête
info-image-fade = Fondu de l'image d'en-tête
info-image-height = Hauteur de l'image d'en-tête
info-attr-default = (par défaut)
file-always-install = Toujours
file-always-install-hint = Toujours installer ce fichier, même si l'option n'est pas sélectionnée (alwaysInstall).
file-install-if-usable = Si utilisable
file-install-if-usable-hint = Installer ce fichier dès que l'option est utilisable, même si elle n'est pas sélectionnée (installIfUsable).
msg-import-lossy = Ce FOMOD contient { $num } constructions que XIMOD ne peut pas éditer ; elles seront perdues à l'enregistrement du projet.
fidelity-nested-deps = Groupe de dépendances imbriqué dans { $context } (un seul niveau est pris en charge)
fidelity-game-dep = Version de jeu requise { $version } dans { $context }
fidelity-fomm-dep = Version de gestionnaire de mods requise { $version } dans { $context }
fidelity-unknown = L'élément « { $element } » dans « { $parent } » n'est pas pris en charge ({ $context })
loc-module = les prérequis du mod
loc-step = l'étape { $step } « { $name } »
loc-installer = l'installateur

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Restaurer une sauvegarde…
backups-title = Restaurer une sauvegarde
backups-empty = Ce projet n'a pas encore de sauvegarde. Une sauvegarde est créée à chaque enregistrement par-dessus une version précédente.
backups-changes = { $num } différence(s) avec le projet actuel
btn-compare = Comparer
btn-restore = Restaurer
btn-delete-backups = Supprimer toutes les sauvegardes
btn-delete-backups-confirm = Cliquez à nouveau pour supprimer toutes les sauvegardes
msg-backup-restored = Sauvegarde du { $time } restaurée dans l'éditeur (pas encore enregistrée ; Annuler la rétablit)
msg-backups-deleted = { $num } sauvegarde(s) supprimée(s)
settings-backup-count = Sauvegardes à conserver :
settings-backup-count-hint = Nombre de versions précédentes des fichiers XML du FOMOD conservées dans fomod/backups à l'enregistrement (0 = aucune sauvegarde).
settings-autosave-minutes = Copie de récupération automatique toutes les (minutes) :
settings-autosave-minutes-hint = Une copie de récupération de chaque projet modifié est écrite dans le dossier de configuration à cet intervalle ; elle n'est proposée au démarrage suivant qu'après une fermeture anormale (0 = désactivé).
settings-auto-masters = Ajouter les masters d'un plugin comme conditions
settings-auto-masters-hint = Quand un plugin (.esp/.esm/.esl) est ajouté à une option, les masters qu'il requiert et que ni le jeu ni ce mod ne fournissent deviennent des conditions de fichier « Active » de l'option.
msg-author-from-plugin = Auteur renseigné depuis l'en-tête du plugin : { $author }
msg-masters-added = { $num } master(s) de { $plugin } ajouté(s) comme condition(s) de fichier
issue-missing-master = { $plugin } requiert { $master }, qui n'est ni dans ce mod ni déclaré comme dépendance
issue-esl-mismatch-flag = { $plugin } porte l'extension .esl mais son indicateur « light » (ESL) n'est pas activé
issue-esl-eligible = { $plugin } pourrait être marqué « light » ({ $num } nouveaux enregistrements, limite { $limit })
issue-esl-too-big = { $plugin } est marqué « light » mais ne respecte pas les règles des plugins légers ({ $num } nouveaux enregistrements, limite { $limit }, ou un FormID hors de la plage autorisée)
menu-plugin-report = Rapport des plugins…
plugins-title = Rapport des plugins
plugins-file = Fichier
plugins-kind = Type
plugins-light = Indicateur light
plugins-masters = Masters
plugins-new-records = Nouveaux enregistrements / limite
plugins-eligible = Éligible light
plugins-empty = Ce projet n'installe aucun fichier plugin (.esp, .esm ou .esl).
plugins-unreadable = illisible

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Arborescence finale des fichiers
preview-total-size = Taille totale installée : { $size }
preview-tree-truncated = L'arborescence est tronquée : trop de fichiers à développer (les tailles ci-dessus sont partielles).
preview-overwritten-by = Écrasé par { $plugin }
preview-scenario = Scénario :
preview-scenario-load = Charger
preview-scenario-save = Enregistrer…
preview-scenario-delete = Supprimer
preview-scenario-name = Nom du scénario
preview-scenario-saved = Scénario « { $name } » enregistré dans fomod/scenarios
preview-scenario-unresolved = { $num } sélection(s) du scénario ne correspondent à aucune option de ce projet (renommée ou supprimée)
preview-scenario-none = (aucun scénario)
issue-unreachable-step = L'étape « { $step } » ne peut jamais s'afficher : ses conditions de visibilité testent une valeur de drapeau qu'aucune option précédente ne définit
issue-unreachable-option = L'option « { $plugin } » ne peut jamais être sélectionnée : ses motifs de type utilisable testent une valeur de drapeau qu'aucune option ne définit
issue-unreachable-cond = Le jeu de fichiers conditionnel { $num } ne peut jamais s'appliquer : ses conditions testent une valeur de drapeau qu'aucune option ne définit
size-option = Taille installée : { $size } ({ $num } fichier(s))
size-missing = { $num } source(s) manquante(s)
size-unknown = Taille installée : — (lancez Valider pour la mesurer)
menu-nexus-desc = Description Nexus…
nexus-title = Description Nexus Mods
nexus-format = Format :
nexus-include-requirements = Prérequis
nexus-include-options = Options d'installation
nexus-include-install = Installation
nexus-include-changelog = Journal des modifications
nexus-previous = Version précédente…
nexus-previous-none = (aucune version précédente : pas de journal des modifications)
nexus-language = Langue :
nexus-language-source = (source)
nexus-sec-requirements = Prérequis
nexus-sec-options = Options d'installation
nexus-sec-install = Installation
nexus-sec-changelog = Journal des modifications
nexus-install-text = Ce mod est livré avec un installateur FOMOD : installez-le avec un gestionnaire de mods (Vortex, Mod Organizer 2) et choisissez vos options dans l'installateur.
nexus-requires = Requiert
nexus-step = Étape
nexus-added = Ajouté
nexus-removed = Supprimé
nexus-changed = Modifié
btn-copy = Copier
btn-save-as = Enregistrer sous…
msg-copied = Copié dans le presse-papiers
msg-saved-to = Enregistré dans { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Renommer…
condeditor-rename-exists = Un drapeau nommé « { $name } » existe déjà
condeditor-renamed = Drapeau « { $from } » renommé en « { $to } » ({ $num } occurrence(s))
condeditor-delete-uses = Supprimer toutes les utilisations
condeditor-deleted-uses = Drapeau « { $name } » supprimé partout ({ $num } occurrence(s))
condeditor-values-set = Valeurs définies :
condeditor-values-tested = Valeurs testées :
condeditor-value-never-set = { $value } — testée mais jamais définie
condeditor-value-never-tested = { $value } — définie mais jamais testée
condeditor-builder = Constructeur de conditions
condeditor-builder-none = Sélectionnez une étape, une option, un ensemble conditionnel ou les informations du mod dans la fenêtre principale pour modifier ses conditions ici.
condeditor-builder-pattern = Motif :
condeditor-sentence-if = SI
condeditor-sentence-and = ET
condeditor-sentence-or = OU
condeditor-sentence-flag = le drapeau { "{name}" } = { "{value}" }
condeditor-sentence-file = le fichier { "{name}" } est { "{value}" }
condeditor-sentence-empty = (aucune condition : toujours vrai)
condeditor-sentence-then-visible = ALORS l'étape est affichée
condeditor-sentence-then-type = ALORS l'option devient { $type }
condeditor-sentence-then-install = ALORS les fichiers sont installés
condeditor-sentence-then-module = ALORS l'installateur peut se lancer (vérifié avant son démarrage)
issue-flag-value-never-set = Le drapeau « { $flag } » est testé avec la valeur « { $value } », qu'aucune option ne définit
issue-flag-never-used = Le drapeau « { $flag } » est défini mais jamais testé
menu-project-strings = Chaînes du projet…
strings-title = Chaînes du projet
strings-search = Rechercher un texte, un emplacement ou une clé…
strings-kind-all = Toutes
strings-kind-names = Noms
strings-kind-descriptions = Descriptions
strings-duplicates-only = Doublons seulement
strings-replace-with = Remplacer par :
strings-case = Respecter la casse
strings-whole-word = Mot entier
strings-replace-current = Remplacer
strings-replace-all = Tout remplacer
strings-replaced = { $num } chaîne(s) remplacée(s)
strings-dup-badge = ×{ $num }
strings-dup-hover = Même texte que :
strings-count = { $num } chaîne(s) · { $dups } groupe(s) de doublons
strings-col-location = Emplacement
strings-col-field = Champ
strings-col-text = Texte

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Contenu d'une archive…
filter-bethesda-archive = Archives Bethesda (bsa, ba2)
archive-view-title = Contenu de l'archive
archive-view-format = Format :
archive-view-entries = { $num } entrées
archive-view-size = { $size } décompressés
archive-view-search = Rechercher un chemin…
archive-view-col-path = Chemin
archive-view-col-size = Taille
archive-view-col-compressed = Compressé
archive-view-truncated = Seules les { $num } premières entrées correspondantes sont affichées — affinez la recherche.
archive-view-error = Cette archive ne peut pas être lue : { $error }
archive-view-hint = Voir le contenu de cette archive
issue-conflict-archive = Même ressource dans plusieurs archives : « { $path } » est empaqueté par { $count } références ({ $locs }) — l'ordre de chargement des archives du jeu décide laquelle est utilisée.
issue-conflict-archive-loose = Archive contre fichier libre : « { $path } » est à la fois empaqueté dans une archive et installé en fichier libre ({ $locs }) — le fichier libre l'emporte sur celui de l'archive.
preview-in-archive = (dans l'archive)
preview-archived-size = dont { $size } empaquetés dans des archives

# --- Project tree: expand / collapse menus
tree-expand = Déployer
tree-collapse = Replier
tree-expand-all = Tout déployer
tree-expand-selected = Déployer la sélection
tree-expand-from = Déployer depuis la sélection
tree-collapse-all = Tout replier
tree-collapse-selected = Replier la sélection
tree-collapse-from = Replier depuis la sélection
tree-expand-all-hint = Déploie tous les titres
tree-expand-selected-hint = Déploie uniquement le titre sélectionné
tree-expand-from-hint = Déploie le titre sélectionné et tous ceux qui dépendent de lui
tree-collapse-all-hint = Replie tous les titres
tree-collapse-selected-hint = Replie uniquement le titre sélectionné
tree-collapse-from-hint = Replie le titre sélectionné et tous ceux qui dépendent de lui

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Ajouter un groupe
btn-remove-group-cond = Supprimer le groupe
dep-type-game = Version du jeu
dep-type-fomm = Version du gestionnaire de mods
dep-group-hint = Un groupe de conditions combinées par Et / Ou ; les groupes peuvent être imbriqués.
condeditor-sentence-game = la version du jeu ≥ { "{value}" }
condeditor-sentence-fomm = la version du gestionnaire de mods ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Textes uniques
ftr-uniques-hint = N'afficher qu'une ligne par texte source distinct. Traduire cette ligne traduit d'un coup toutes les chaînes de même texte.
ftr-uniques-synced = { $num } chaînes identiques mises à jour.
ftr-uniques-group = { $num } chaînes partagent ce texte ; sa traduction s'applique à toutes.
