#!/usr/bin/env bash
# =============================================================================
# XIMOD Architect - dependances de compilation (Linux)
# -----------------------------------------------------------------------------
# Installe les bibliotheques de developpement necessaires a la compilation,
# selon la distribution : Debian/Ubuntu (apt), Fedora/RHEL (dnf), Arch (pacman).
#
# Usage :
#   ./build-deps.sh          # demande confirmation au gestionnaire de paquets
#   ./build-deps.sh -y       # mode non interactif (utile en CI)
#
# N'installe PAS Rust : faites-le ensuite avec rustup :
#   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# =============================================================================
set -euo pipefail

ASSUME_YES=0
case "${1:-}" in
  -y|--yes) ASSUME_YES=1;;
  "" ) ;;
  * ) echo "Option inconnue : $1 (utilisez -y pour le mode non interactif)"; exit 2;;
esac

# sudo si l'on n'est pas root
SUDO=""
if [[ "$(id -u)" -ne 0 ]]; then
  if command -v sudo >/dev/null 2>&1; then SUDO="sudo"; else
    echo "Lancez ce script en root, ou installez sudo." >&2; exit 1
  fi
fi

# Detection de la famille de distribution
ID=""; ID_LIKE=""; PRETTY_NAME=""
if [[ -r /etc/os-release ]]; then
  # shellcheck disable=SC1091
  . /etc/os-release
fi
FAMILY=""
case " ${ID:-} ${ID_LIKE:-} " in
  *" debian "*|*" ubuntu "*) FAMILY="debian";;
  *" fedora "*|*" rhel "*|*" centos "*) FAMILY="fedora";;
  *" arch "*) FAMILY="arch";;
esac
if [[ -z "$FAMILY" ]]; then      # repli : detection par gestionnaire de paquets
  if   command -v apt-get >/dev/null 2>&1; then FAMILY="debian"
  elif command -v dnf     >/dev/null 2>&1; then FAMILY="fedora"
  elif command -v pacman  >/dev/null 2>&1; then FAMILY="arch"
  fi
fi

echo "Distribution : ${PRETTY_NAME:-inconnue}  (famille : ${FAMILY:-non reconnue})"

case "$FAMILY" in
  debian)
    YES=""; [[ "$ASSUME_YES" -eq 1 ]] && YES="-y"
    $SUDO apt-get update
    $SUDO apt-get install $YES \
      build-essential pkg-config curl git \
      libgtk-3-dev \
      libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
      libspeechd-dev libxkbcommon-dev libssl-dev libx11-dev
    ;;
  fedora)
    YES=""; [[ "$ASSUME_YES" -eq 1 ]] && YES="-y"
    $SUDO dnf install $YES \
      gcc gcc-c++ make pkgconf-pkg-config curl git \
      gtk3-devel libxcb-devel \
      speech-dispatcher-devel libxkbcommon-devel openssl-devel libX11-devel
    ;;
  arch)
    YES=""; [[ "$ASSUME_YES" -eq 1 ]] && YES="--noconfirm"
    $SUDO pacman -S --needed $YES \
      base-devel pkgconf curl git \
      gtk3 libxcb speech-dispatcher libxkbcommon openssl libx11
    ;;
  *)
    cat >&2 <<'EOF'
Distribution non reconnue. Installez manuellement l'equivalent de :
  - outils de compilation C : build-essential / gcc gcc-c++ make / base-devel
  - pkg-config (pkgconf)
  - GTK 3        (libgtk-3-dev / gtk3-devel / gtk3)
  - libxcb       (render, shape, xfixes)
  - libxkbcommon (libxkbcommon-dev / libxkbcommon-devel)
  - speech-dispatcher (libspeechd-dev / speech-dispatcher-devel)
  - OpenSSL      (libssl-dev / openssl-devel / openssl)
  - libX11       (libx11-dev / libX11-devel / libx11)
EOF
    exit 1
    ;;
esac

echo
echo "Dependances installees."
echo "Si Rust n'est pas encore present :"
echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
echo "Puis, dans le dossier du projet :  cargo build --release"
