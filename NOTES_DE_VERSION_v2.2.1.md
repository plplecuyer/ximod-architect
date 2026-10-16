# XIMOD Architect 2.2.1

XIMOD Architect crée des installateurs FOMOD — le format lu par Vortex et Mod
Organizer 2 — sous Windows, Linux et macOS. La version 2.2.1 corrige l'assistant
preset et les installateurs qu'il construit, et ajoute ce qui manquait pour les
maintenir, après les tests de TheRealDoctorNormal sur un vrai installateur de
plus de 40 pages.

## Corrections

**Assistant preset : fichiers d'une page masquée.** Dans un installateur
construit par l'assistant, un utilisateur qui ouvrait une page de
fonctionnalité, puis revenait à la première page pour la masquer, pouvait
encore recevoir les fichiers que le preset y avait choisis — les gestionnaires
de mods conservent les drapeaux d'une page déjà vue. La première page pose
désormais explicitement chaque drapeau Personnaliser, et chaque fichier dépend
du fait que sa page est personnalisée ou non : ce qui est coché sur les pages
personnalisées est ce qui s'installe. Pour mettre à jour un installateur
existant, relancez Projet → Assistant preset… puis Appliquer.

**Assistant preset : deux options « Aucune » dans Vortex.** Vortex ajoute sa
propre option Aucune aux groupes « au plus un » ; un groupe qui reçoit
l'option Aucune de l'assistant devient « exactement un », et une seule option
Aucune s'affiche.

**Dates des exports ZIP.** Les archives ZIP exportées (et les paquets de
traduction) dataient tous les fichiers du 1er janvier 1980 ; chaque fichier
garde désormais sa date de modification.

## Ajouts

**Relancer l'assistant preset.** Sur un projet qu'il a déjà transformé,
l'assistant s'ouvre avec les pages, les noms et les presets existants déjà
chargés ; ajoutez ou modifiez des presets et Appliquer reconstruit la
structure en une seule opération annulable, en conservant les noms des
drapeaux, pour qu'une mise à jour du mod reste compatible avec la version
précédente.

**« Aucun preset (tout composer) ».** Le groupe Preset est désormais un choix
unique qui se termine par cette option, pour les utilisateurs qui veulent tout
choisir eux-mêmes.

**Une destination pour tous les ensembles conditionnels.** L'onglet
Installations conditionnelles applique une destination à tous les fichiers de
tous les ensembles conditionnels d'un coup, et les destinations de groupe et de
page atteignent aussi les fichiers que l'assistant a déplacés vers les
installations conditionnelles.

## Remarques

- Les projets et FOMOD enregistrés par la 2.2.0 s'ouvrent sans changement.
- Le manuel (32 langues) documente ces changements aux § 4.2, § 4.4, § 7.5 et § 8.29.
- Interface en 33 langues ; chaque nouvelle chaîne est traduite partout.
- Dépendances mises à jour. La compilation depuis les sources requiert Rust 1.99.0 ou ultérieur.
