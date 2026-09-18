# assets/

Ressources chargées au runtime (jamais embarquées dans le binaire).

```
assets/
├── data/
│   ├── Countries.json          ✅ fourni
│   ├── Languages.json          ✅ fourni
│   └── CountryLanguages.json   ⬜ optionnel (dégradation en liste vide)
├── fonts/                      ⬜ polices Noto (partagées avec XIMOD Architect)
├── icons/
│   ├── app_icon.png            ⬜ icône fenêtre (Linux/dev)
│   ├── ximod-translator.ico    ⬜ icône Windows (embarquée par build.rs)
│   ├── ximod-translator.icns   ⬜ icône macOS
│   └── ximod-translator_*.png  ⬜ tailles Linux (16..512)
├── images/
│   ├── icons/                  ⬜ Arrow_*.png, delete.png
│   ├── svg/                    ⬜ 244 drapeaux de pays
│   └── splash.png              ⬜ écran d'ouverture
└── locales/<iso639-3>/main.ftl ✅ eng, fra fournis (à compléter : 32 langues)
```

Les gros binaires (polices, drapeaux, icônes, splash) sont **communs à XIMOD
Architect** : recopiez-les depuis son arborescence `assets/`. Tout ce qui manque
dégrade proprement — polices intégrées d'egui, pas de splash, icône par défaut.
