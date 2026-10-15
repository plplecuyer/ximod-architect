# XIMOD Architect 2.2.0

XIMOD Architect crée des installateurs FOMOD pour les mods Bethesda (Skyrim,
Fallout, Starfield, Oblivion, Morrowind) sous Windows, Linux et macOS. La
version 2.2.0 facilite la réutilisation d'éléments d'installateur d'un mod à
l'autre et garde les références de fichiers justes quand des fichiers sont
renommés ou déplacés — des idées de Wenderer, auteur du FOMOD Creation Tool
d'origine, et d'un de ses utilisateurs.

## Points forts

**Couper, copier et coller entre FOMOD.** Les étapes, groupes et options se
coupent, se copient et se collent (menu Édition, clic droit dans l'arbre, ou
Ctrl+X / Ctrl+C / Ctrl+V). Le presse-papiers est partagé par tous les onglets
FOMOD ouverts, et même entre deux fenêtres XIMOD. L'élément collé se place à
côté de la sélection, en une seule modification annulable, et XIMOD signale
les fichiers qu'il utilise et qui ne sont pas dans le dossier de ce mod, ainsi
que les drapeaux qu'il teste et qu'aucune option de ce projet ne définit.

**Renommer sur le disque.** Dans les tableaux de fichiers, « Renommer sur le
disque… » renomme le fichier ou le dossier sélectionné dans le dossier du mod
et met à jour toutes ses références — pour un dossier, tout ce qu'il contient
aussi.

**Retrouver les fichiers manquants.** Projet → Retrouver les fichiers
manquants… liste tout ce que le projet utilise et qui n'existe plus, et propose
où c'est passé : le même nom ailleurs (déplacé) ou un nom proche dans le même
dossier (renommé). Choisissez le bon, ou parcourez, puis appliquez toutes les
corrections d'un coup.

## Remarques

- Les projets et FOMOD enregistrés par la 2.1.0 s'ouvrent sans changement.
- Une cellule Destination vide affiche « (racine de Data) », avec une info-bulle : cliquez dans la cellule pour indiquer où installer un fichier ou un dossier.
- Les fichiers XML générés sont signés avec le nom, la version et la page Nexus de l'application, comme le faisait FOMOD Creation Tool.
- Libellés de l'interface corrigés dans toutes les langues, et manuels alignés dessus.
- L'assistant preset peut être relancé pour ajouter ou modifier des presets : il reconstruit sa structure au lieu d'en empiler une seconde, et conserve les noms des drapeaux.
- Une destination pour tous les ensembles conditionnels d'un coup ; les destinations de groupe et de page atteignent aussi les fichiers que l'assistant preset a déplacés vers les installations conditionnelles.
- Corrigé : les archives ZIP exportées dataient tous les fichiers du 1er janvier 1980 ; ils gardent désormais leur date de modification.
- Interface en 33 langues ; chaque nouvelle chaîne est traduite partout.
- La compilation depuis les sources requiert Rust 1.99.0 ou ultérieur.
