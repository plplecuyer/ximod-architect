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
menu-open = Ouvrir dossier...
menu-open-file = Ouvrir fichier...
menu-save = Enregistrer
menu-recent = Récents
menu-exit = Quitter
menu-merge = Fusionner FOMOD...
menu-export = Exporter l'archive de distribution...

# Menu - Options
menu-options = Options
menu-settings = Paramètres
menu-pre-save-script = Script pré-Sauvegarde...
menu-post-save-script = Script post-Sauvegarde...
menu-translation = Traduction...

# Menu - Help
menu-help = Aide
menu-about = À propos

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
label-plugin-name = Nom du plugin :
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
btn-add-plugin = Ajouter plugin
btn-remove-plugin = Supprimer plugin
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
validation-no-plugins = L'étape { $step }, groupe « { $name } » n'a pas de plugins

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
default-plugin-name = Plugin { $num }
pattern-label = Motif { $num }

# Selection prompts
msg-select-group-first = Sélectionnez d'abord un groupe.
msg-select-plugin-edit = Sélectionnez un plugin à éditer.
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
