# XIMOD Architect 2.0.0

XIMOD Architect crée des installateurs FOMOD pour les mods Bethesda (Skyrim,
Fallout, Starfield, Oblivion, Morrowind) sous Windows, Linux et macOS. La
version 2.0.0 est une refonte complète de la version 1 : nouvelle interface,
couverture complète du format FOMOD, et des outils pour vérifier, simuler,
traduire et publier les installateurs. Le manuel de l'utilisateur (français et
anglais) est fourni dans `Manuals/`.

## Points forts

**Une nouvelle interface.** Un arbre du projet à gauche et un inspecteur à
droite remplacent les onglets empilés. Annuler / rétablir, réordonnancement par
glisser-déposer, barre d'outils, libellés en langage courant avec infobulles sur
chaque notion FOMOD, thèmes sombre / clair / système, et navigation au clavier
partout (↑ ↓ ← → Entrée dans les arbres, Page haut / Page bas dans les tables).
Les textes japonais, chinois, coréens, arabes, grecs et cyrilliques s'affichent
désormais correctement dans toute l'application.

**Tout le format FOMOD.** Groupes de conditions imbriqués ET / OU, dépendances
du module, `alwaysInstall` / `installIfUsable`, ordre explicite, conditions de
version FOMM et du jeu. Les installateurs tiers s'ouvrent sans perte — y compris
les très gros — et ce que XIMOD ne sait pas modéliser est signalé au lieu
d'être supprimé.

**Moins d'installateurs cassés.** Un panneau Problèmes aux constats
cliquables ; une détection des conflits de destination qui regarde dans les
archives BSA / BA2 et suit les pages conditionnées par des drapeaux ; la
vérification des fichiers référencés ; les masters lus dans les en-têtes de
plugins et transformés en conditions ; l'éligibilité ESL et un rapport des
plugins ; le contrôle des images. Un éditeur de conditions montre, pour chaque
drapeau, qui le définit et qui le teste, le renomme partout et construit les
conditions visuellement.

**Voir l'installateur comme les joueurs.** L'aperçu démarre sur la page
d'informations du mod, parcourt chaque étape (pages masquées signalées) et se
termine par l'arbre final des fichiers : tailles, écrasements par priorité,
contenu des archives, poids par option. Les scénarios s'enregistrent et se
rejouent en ligne de commande.

**Traduire des FOMOD existants.** Ouvrez n'importe quel installateur (dossier
ou `.zip` / `.7z`), traduisez ses chaînes dans une table avec mémoire,
glossaire, mode « Textes uniques » et échange CSV, puis exportez un paquet prêt
pour Nexus. Le XML d'origine est corrigé sur place, jamais réécrit ; un dialogue
de mise à jour reporte les traductions quand le mod change.

**Publier.** Export ZIP ou 7z, sauvegardes tournantes avec restauration et
différentiel, comparaison de FOMOD, modèles réutilisables, assistant « Nouveau
depuis un dossier », générateur de description Nexus (BBCode / Markdown), et
une ligne de commande (`validate`, `build`, `package`, `batch`, `translate`,
`simulate`, `inspect`, `archive`, `ba2`) pour l'automatisation.

## Corrections à connaître

La version 1 échappait deux fois les attributs XML (`&`, `<`, `"`…) à chaque
enregistrement et écrivait des caractères invisibles dans les noms par défaut ;
les deux sont corrigés, et les fichiers sont désormais écrits de façon atomique,
avec une protection anti-plantage qui conserve le travail non enregistré. Les
descriptions CDATA, les chemins d'images avec barres obliques inverses sous
Linux / macOS et les noms `Fomod` / `Info.xml` en toute casse sont pris en
charge.

## Remarques

- Les projets enregistrés par la version 1 s'ouvrent tels quels. Les fichiers
  enregistrés par la 2.0.0 gardent la même organisation, avec les corrections
  ci-dessus.
- Interface en 33 langues ; le français et l'anglais sont les traductions de
  référence.
- La compilation depuis les sources requiert Rust 1.98.1 ou supérieur.

La liste complète des changements figure dans `CHANGELOG.md` (en anglais).
