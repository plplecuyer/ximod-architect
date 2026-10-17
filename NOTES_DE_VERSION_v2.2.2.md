# XIMOD Architect 2.2.2

XIMOD Architect crée des installateurs FOMOD — le format lu par Vortex et Mod
Organizer 2, pour Skyrim, Fallout, Starfield et bien d'autres jeux — sous
Windows, Linux et macOS. La version 2.2.2 corrige définitivement les
installateurs construits par l'assistant preset, après les tests de la 2.2.1
par TheRealDoctorNormal sur un vrai mod.

## Corrections

**Assistant preset : ce qui est affiché est ce qui s'installe.** Parcourir les
pages, revenir en arrière et choisir de nouveau pouvait encore installer des
fichiers qui n'apparaissaient pas comme sélectionnés, ou omettre des fichiers
qui l'étaient. La cause était dans l'installateur généré : plusieurs options
sélectionnables ensemble écrivaient le même drapeau, alors que les
gestionnaires de mods tiennent les drapeaux dans une table où le dernier clic
gagne. L'assistant suit désormais une règle : un drapeau, un seul écrivain.
Les presets n'écrivent que le drapeau du preset ; chaque page de
fonctionnalité reçoit sur la première page un groupe à deux options, « Suivre
les choix du preset » / « Personnaliser », à la place d'une case à cocher ; les
options de la page écrivent seules les drapeaux de la fonctionnalité. Les
drapeaux correspondent alors toujours à l'écran, quel que soit l'ordre des
clics.

**Relancez l'assistant pour mettre à jour.** Les structures créées par les
versions 2.1.0, 2.2.1 et 2.2.2 sont reconnues : Projet → Assistant preset…
puis Appliquer reconstruit la vôtre à la nouvelle manière, conserve les noms
des drapeaux et laisse la page de mode d'installation là où vous l'aviez
placée.

## Remarques

- Les projets et FOMOD enregistrés par la 2.2.1 s'ouvrent sans changement.
- Le manuel (32 langues) explique la règle au § 7.5, avec un nouvel exemple XML, et l'assistant au § 8.29.
- Interface en 33 langues ; chaque nouvelle chaîne est traduite partout.
- La compilation depuis les sources requiert Rust 1.99.0 ou ultérieur.
