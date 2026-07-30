#!/usr/bin/env bash
# =============================================================================
# XIMOD Architect — constructeur de bundle macOS
# -----------------------------------------------------------------------------
# Compile un binaire (universel arm64 + x86_64 par défaut avec --target
# universal), assemble un .app conforme, le signe (Hardened Runtime), et, en
# option, fabrique et signe un DMG puis le notarise et l'agrafe (staple).
#
# Doit être exécuté SUR un Mac (Xcode Command Line Tools requis pour la
# compilation, la signature et la notarisation).
#
# Exemples :
#   ./create-bundle.sh                                  # arch. hôte, signature ad-hoc, .app
#   ./create-bundle.sh --target universal               # binaire universel, signature ad-hoc
#   ./create-bundle.sh --target universal \
#       --identity "Developer ID Application: Jane Doe (TEAMID1234)" --dmg
#   ./create-bundle.sh --target universal \
#       --identity "Developer ID Application: Jane Doe (TEAMID1234)" \
#       --dmg --notarize --keychain-profile ximod-notary
#
# Préparer un profil de notarisation (une seule fois) :
#   xcrun notarytool store-credentials ximod-notary \
#       --apple-id "vous@exemple.com" --team-id TEAMID1234 \
#       --password "mot-de-passe-app-specifique"
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# --------------------------- valeurs par défaut ------------------------------
APP_NAME="XIMOD Architect"
BUNDLE_NAME="XIMOD Architect.app"
BIN_NAME="ximod-architect"
VERSION="$(grep -m1 '^version' "$PROJECT_DIR/Cargo.toml" | sed -E 's/.*"([^"]+)".*/\1/' || echo "1.0.0")"

TARGET_MODE="host"                                  # host | universal | arm64 | x86_64
IDENTITY=""                                         # identité codesign ; vide => ad-hoc "-"
ENTITLEMENTS="$SCRIPT_DIR/entitlements.plist"
OUTPUT_DIR="$PROJECT_DIR/dist"
ICON_ICNS="$PROJECT_DIR/assets/icons/${BIN_NAME}.icns"
ICON_PNG="$PROJECT_DIR/assets/icons/${BIN_NAME}.png" # sert à générer l'.icns s'il manque

DO_BUILD=1
DO_SIGN=1
MAKE_DMG=0
NOTARIZE=0
KEYCHAIN_PROFILE=""
APPLE_ID=""; TEAM_ID=""; APP_PW=""

usage() {
  sed -n '2,30p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

# ------------------------------ arguments ------------------------------------
while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)           VERSION="$2"; shift 2;;
    --target)            TARGET_MODE="$2"; shift 2;;
    --identity)          IDENTITY="$2"; shift 2;;
    --entitlements)      ENTITLEMENTS="$2"; shift 2;;
    --output)            OUTPUT_DIR="$2"; shift 2;;
    --icon)              ICON_ICNS="$2"; shift 2;;
    --no-build)          DO_BUILD=0; shift;;
    --no-sign|--skip-sign) DO_SIGN=0; shift;;
    --dmg)               MAKE_DMG=1; shift;;
    --notarize)          NOTARIZE=1; shift;;
    --keychain-profile)  KEYCHAIN_PROFILE="$2"; shift 2;;
    --apple-id)          APPLE_ID="$2"; shift 2;;
    --team-id)           TEAM_ID="$2"; shift 2;;
    --password)          APP_PW="$2"; shift 2;;
    -h|--help)           usage; exit 0;;
    *) echo "Option inconnue : $1" >&2; echo "  (--help pour l'aide)" >&2; exit 2;;
  esac
done

log()  { printf '\033[1;34m>\033[0m %s\n' "$*"; }
ok()   { printf '\033[1;32m+\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m!\033[0m %s\n' "$*"; }
die()  { printf '\033[1;31mx %s\033[0m\n' "$*" >&2; exit 1; }

# ------------------------------ vérifications --------------------------------
if [[ "$(uname -s)" != "Darwin" ]]; then
  if [[ "$DO_BUILD" -eq 1 || "$DO_SIGN" -eq 1 ]]; then
    die "Ce script doit etre lance sur macOS (compilation/signature Apple requises)."
  fi
fi
if [[ "$NOTARIZE" -eq 1 ]]; then
  [[ -n "$IDENTITY" ]] || die "La notarisation exige une vraie identite (--identity \"Developer ID Application: ...\")."
  if [[ -z "$KEYCHAIN_PROFILE" && ( -z "$APPLE_ID" || -z "$TEAM_ID" || -z "$APP_PW" ) ]]; then
    die "Notarisation : fournissez --keychain-profile OU (--apple-id, --team-id, --password)."
  fi
  MAKE_DMG=1   # on notarise un conteneur : on force la creation du DMG
fi

log "XIMOD bundle - version $VERSION - cible $TARGET_MODE"
mkdir -p "$OUTPUT_DIR"

# ------------------------------ compilation ----------------------------------
ARM64_TRIPLE="aarch64-apple-darwin"
X86_TRIPLE="x86_64-apple-darwin"

build_triple() { # $1 = triple ; imprime le chemin du binaire
  local t="$1"
  if command -v rustup >/dev/null 2>&1; then
    rustup target add "$t" >/dev/null 2>&1 || true
  fi
  ( cd "$PROJECT_DIR" && cargo build --release --target "$t" ) 1>&2
  echo "$PROJECT_DIR/target/$t/release/$BIN_NAME"
}

FINAL_BIN=""
if [[ "$DO_BUILD" -eq 1 ]]; then
  case "$TARGET_MODE" in
    host)
      ( cd "$PROJECT_DIR" && cargo build --release ) 1>&2
      FINAL_BIN="$PROJECT_DIR/target/release/$BIN_NAME"
      ok "Binaire (hote) compile"
      ;;
    arm64)     FINAL_BIN="$(build_triple "$ARM64_TRIPLE")"; ok "Binaire arm64 compile";;
    x86_64)    FINAL_BIN="$(build_triple "$X86_TRIPLE")";   ok "Binaire x86_64 compile";;
    universal)
      BIN_ARM="$(build_triple "$ARM64_TRIPLE")"
      BIN_X86="$(build_triple "$X86_TRIPLE")"
      FINAL_BIN="$PROJECT_DIR/target/${BIN_NAME}-universal"
      lipo -create -output "$FINAL_BIN" "$BIN_ARM" "$BIN_X86"
      ok "Binaire universel cree : $(lipo -archs "$FINAL_BIN")"
      ;;
    *) die "Cible inconnue : $TARGET_MODE (host|universal|arm64|x86_64)";;
  esac
else
  # --no-build : on recupere un binaire deja present
  for cand in \
    "$PROJECT_DIR/target/${BIN_NAME}-universal" \
    "$PROJECT_DIR/target/release/$BIN_NAME" \
    "$PROJECT_DIR/target/$ARM64_TRIPLE/release/$BIN_NAME" \
    "$PROJECT_DIR/target/$X86_TRIPLE/release/$BIN_NAME"; do
    [[ -f "$cand" ]] && { FINAL_BIN="$cand"; break; }
  done
  [[ -n "$FINAL_BIN" ]] || die "--no-build : aucun binaire trouve. Compilez d'abord avec cargo."
  ok "Binaire reutilise : $FINAL_BIN"
fi
[[ -f "$FINAL_BIN" ]] || die "Binaire introuvable : $FINAL_BIN"

# ------------------------------ icone (.icns) --------------------------------
make_icns() { # $1 = png source (idealement 1024x1024) ; $2 = icns de sortie
  local src="$1" out="$2" set_dir
  command -v iconutil >/dev/null 2>&1 || { warn "iconutil absent : icone ignoree"; return 1; }
  set_dir="$(mktemp -d)/icon.iconset"; mkdir -p "$set_dir"
  local s
  for s in 16 32 128 256 512; do
    sips -z "$s" "$s"                 "$src" --out "$set_dir/icon_${s}x${s}.png"    >/dev/null
    sips -z "$((s*2))" "$((s*2))"     "$src" --out "$set_dir/icon_${s}x${s}@2x.png" >/dev/null
  done
  iconutil -c icns "$set_dir" -o "$out"
}

if [[ ! -f "$ICON_ICNS" && -f "$ICON_PNG" ]]; then
  log "Generation de l'icone .icns depuis $ICON_PNG"
  if make_icns "$ICON_PNG" "$OUTPUT_DIR/${BIN_NAME}.icns"; then
    ICON_ICNS="$OUTPUT_DIR/${BIN_NAME}.icns"; ok "Icone .icns generee"
  fi
fi

# ------------------------------ assemblage .app ------------------------------
BUNDLE_DIR="$OUTPUT_DIR/$BUNDLE_NAME"
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR/Contents/MacOS"
mkdir -p "$BUNDLE_DIR/Contents/Resources"

cp "$FINAL_BIN" "$BUNDLE_DIR/Contents/MacOS/$BIN_NAME"
chmod +x "$BUNDLE_DIR/Contents/MacOS/$BIN_NAME"
ok "Binaire copie dans le bundle"

# Info.plist (copie + mise a jour de la version)
cp "$SCRIPT_DIR/Info.plist" "$BUNDLE_DIR/Contents/Info.plist"
if [[ -x /usr/libexec/PlistBuddy ]]; then
  /usr/libexec/PlistBuddy -c "Set :CFBundleVersion $VERSION"            "$BUNDLE_DIR/Contents/Info.plist" 2>/dev/null || true
  /usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$BUNDLE_DIR/Contents/Info.plist" 2>/dev/null || true
fi
printf 'APPL????' > "$BUNDLE_DIR/Contents/PkgInfo"
ok "Info.plist et PkgInfo ecrits"

# Icone
if [[ -f "$ICON_ICNS" ]]; then
  cp "$ICON_ICNS" "$BUNDLE_DIR/Contents/Resources/${BIN_NAME}.icns"
  ok "Icone copiee"
else
  warn "Aucune icone .icns (l'app utilisera l'icone generique du systeme)"
fi
# PNG d'icone pour le rendu dans la fenetre (charge au runtime depuis Resources)
[[ -f "$ICON_PNG" ]] && cp "$ICON_PNG" "$BUNDLE_DIR/Contents/Resources/${BIN_NAME}.png"

# Assets - sous Contents/Resources/assets (emplacement attendu par l'app dans un
# bundle, et recommande pour la signature : aucun fichier non-code sous MacOS/).
if [[ -d "$PROJECT_DIR/assets" ]]; then
  mkdir -p "$BUNDLE_DIR/Contents/Resources/assets"
  ( cd "$PROJECT_DIR/assets" && \
    cp -R data locales fonts images "$BUNDLE_DIR/Contents/Resources/assets/" 2>/dev/null || true )
  ok "Assets copies (data, locales, fonts, images)"
else
  warn "Dossier assets/ introuvable - l'app risque de manquer ses donnees"
fi

# ------------------------------ signature ------------------------------------
if [[ "$DO_SIGN" -eq 1 ]]; then
  if [[ -n "$IDENTITY" ]]; then
    SIGN_ID="$IDENTITY"; SIGN_OPTS=(--options runtime --timestamp)
    log "Signature avec l'identite : $IDENTITY"
  else
    SIGN_ID="-"; SIGN_OPTS=()
    warn "Aucune identite fournie : signature AD-HOC (locale uniquement, non distribuable/notarisable)"
  fi
  ENT_OPT=()
  [[ -f "$ENTITLEMENTS" ]] && ENT_OPT=(--entitlements "$ENTITLEMENTS")

  # De l'interieur vers l'exterieur : d'abord tout code imbrique (dylibs/frameworks
  # eventuels), puis l'executable, puis le bundle.
  # NB : idiome "${arr[@]+...}" pour rester compatible avec bash 3.2 (macOS) sous set -u,
  # ou l'expansion d'un tableau vide declencherait "unbound variable".
  while IFS= read -r -d '' nested; do
    codesign --force ${SIGN_OPTS[@]+"${SIGN_OPTS[@]}"} --sign "$SIGN_ID" "$nested"
  done < <(find "$BUNDLE_DIR/Contents" \( -name '*.dylib' -o -name '*.framework' \) -print0 2>/dev/null)

  codesign --force ${SIGN_OPTS[@]+"${SIGN_OPTS[@]}"} ${ENT_OPT[@]+"${ENT_OPT[@]}"} --sign "$SIGN_ID" "$BUNDLE_DIR/Contents/MacOS/$BIN_NAME"
  codesign --force ${SIGN_OPTS[@]+"${SIGN_OPTS[@]}"} ${ENT_OPT[@]+"${ENT_OPT[@]}"} --sign "$SIGN_ID" "$BUNDLE_DIR"
  ok "Bundle signe"

  log "Verification de la signature"
  codesign --verify --deep --strict --verbose=2 "$BUNDLE_DIR" && ok "Signature valide"
else
  warn "Signature ignoree (--no-sign)"
fi

# ------------------------------ DMG ------------------------------------------
DMG_PATH="$OUTPUT_DIR/${APP_NAME// /_}-${VERSION}.dmg"
if [[ "$MAKE_DMG" -eq 1 ]]; then
  log "Creation du DMG"
  rm -f "$DMG_PATH"
  hdiutil create -volname "$APP_NAME" -srcfolder "$BUNDLE_DIR" -ov -format UDZO "$DMG_PATH" >/dev/null
  if [[ "$DO_SIGN" -eq 1 && -n "$IDENTITY" ]]; then
    codesign --force --timestamp --sign "$IDENTITY" "$DMG_PATH"
  fi
  ok "DMG cree : $DMG_PATH"
fi

# ------------------------------ notarisation ---------------------------------
if [[ "$NOTARIZE" -eq 1 ]]; then
  log "Soumission a la notarisation Apple (peut prendre quelques minutes)..."
  if [[ -n "$KEYCHAIN_PROFILE" ]]; then
    xcrun notarytool submit "$DMG_PATH" --keychain-profile "$KEYCHAIN_PROFILE" --wait
  else
    xcrun notarytool submit "$DMG_PATH" --apple-id "$APPLE_ID" --team-id "$TEAM_ID" --password "$APP_PW" --wait
  fi
  ok "Notarisation acceptee"
  log "Agrafage (staple) du ticket"
  xcrun stapler staple "$BUNDLE_DIR"
  xcrun stapler staple "$DMG_PATH"
  ok "Ticket agrafe au .app et au DMG"
  log "Controle Gatekeeper"
  spctl -a -vvv -t exec "$BUNDLE_DIR" || warn "spctl a signale un souci - verifiez le rapport de notarisation"
fi

echo
ok "Termine."
echo "  .app : $BUNDLE_DIR"
[[ "$MAKE_DMG" -eq 1 ]] && echo "  DMG  : $DMG_PATH"
if [[ "$DO_SIGN" -eq 1 && -z "$IDENTITY" ]]; then
  echo
  warn "Rappel : ce build est signe ad-hoc. Pour une distribution hors de votre Mac,"
  warn "relancez avec --identity \"Developer ID Application: ... (TEAMID)\" --dmg --notarize."
fi
