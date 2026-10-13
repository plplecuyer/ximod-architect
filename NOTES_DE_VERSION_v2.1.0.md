# XIMOD Architect 2.1.0

XIMOD Architect construit des installeurs FOMOD pour les mods Bethesda (Skyrim,
Fallout, Starfield, Oblivion, Morrowind) sous Windows, Linux et macOS. La version
2.1.0 apporte quatre fonctionnalités suggérées par des auteurs de mods qui
utilisent la version 2, ainsi qu'un éditeur de jeux / catégories. Le manuel
utilisateur (anglais et français) est fourni dans `Manuals/` ; le chapitre 7
explique les techniques et les § 8.26 à § 8.29 les nouveaux outils.

## Points forts

**Assistant preset.** Projet → Assistant preset… construit en une étape un
installeur « preset + personnalisation » : prendre un preset et en rester là,
prendre un preset et ajuster quand même quelques fonctionnalités, ou tout
composer à la main — sans qu'aucun fichier soit jamais installé deux fois. On
part des pages de fonctionnalités existantes (on les coche et on définit autant
de presets que voulu dans une matrice de choix) ou on décrit les
fonctionnalités et l'assistant crée les pages. Il insère une page de mode
d'installation, n'affiche chaque page de fonctionnalité qu'à la demande avec les
choix du preset précochés (types dynamiques *Recommended*), ajoute des options
*Aucune* et déplace les fichiers vers les installations conditionnelles — une
seule opération annulable.

**Couleurs de l'arbre.** Outils → Couleurs de l'arbre… donne à chaque type de
ligne de l'arbre du projet sa propre couleur (informations du mod, étapes,
groupes, options, fichiers requis, installations conditionnelles), un jeu par
thème, choisie dans la fenêtre « Sélectionner une couleur » commune avec XIMOD
Translator (index des couleurs AutoCAD ou couleurs vraies).

**Des boîtes de dialogue qui se souviennent.** Ajouter fichier / Ajouter
dossier / Parcourir (image) se rouvrent dans le dossier parent du dernier
élément choisi : les dossiers frères sont à un clic.

**Destination pour tout un ensemble.** Les onglets Fichiers requis et
Installations conditionnelles reçoivent la destination en un clic dont
disposaient déjà les étapes et les groupes.

**Éditeur de jeux / catégories.** Outils → Jeux / catégories… ajoute, corrige
ou supprime un jeu et ses catégories Nexus sans modifier `Categories.json` ; les
modifications vont dans une copie par utilisateur si le dossier d'installation
est en lecture seule.

## Manuel

Nouvelles sections en anglais et en français : § 7.5 « Preset avec valeurs par
défaut personnalisables », § 7.6 « Pages d'introduction et de clôture », une
règle de conception au § 6.6, et les § 8.26 à § 8.29. Les 30 autres éditions
traduites sont également à jour.

## Remarques

- Les projets et FOMOD enregistrés par la 2.0.0 s'ouvrent sans changement.
- Corrigé sous Windows et macOS : le paquet de traduction complet d'un FOMOD dont le dossier ou les fichiers ont une autre casse (`Fomod/Info.xml`, `moduleconfig.xml`) ne contient plus les deux fichiers XML en double.
- Interface en 33 langues ; chaque nouvelle chaîne est traduite partout.
- La compilation depuis les sources requiert Rust 1.99.0 ou ultérieur.
