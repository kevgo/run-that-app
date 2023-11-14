#!/usr/bin/env bash
set -Eeuo pipefail

print_welcome() {
	echo "RUN-THAT-APP INSTALLATION SCRIPT"
	echo
	echo "This installer is under development. Please report issues at"
	echo "https://github.com/kevgo/run-that-app/issues"
	echo
}

VERSION_TO_INSTALL="0.0.0" # the version of run-that-app to install
TMP_DIR=./run_that_app_install

main() {
	print_welcome

	# verify the environment
	OS="$(os_name)"
	if [ "$OS" = "other" ]; then
		err "Unsupported operating system, please install from source"
	fi
	CPU="$(cpu_name)"
	if [ "$CPU" = "other" ]; then
		err "Unsupported CPU architecture, please install from source."
	fi
	need_cmd uname
	need_cmd curl
	need_cmd command

	# determine the configuration
	DEST_FILE=$(executable_filename "$OS")

	# verify and set up the target location
	check_already_installed "$DEST_FILE"

	# download and install the executable
	DOWNLOAD_URL="$(download_url "$OS" "$CPU")"
	download_and_extract "$DOWNLOAD_URL" "$OS" "$DEST_FILE"

	# print summary
	echo
	echo "Successfully installed run-that-app $VERSION_TO_INSTALL for $OS/$CPU."
}

create_folder() {
	[ ! -d "$1" ] && mkdir "$1"
}

# provides the name of the operating system in the format used in the release archive filenames
os_name() {
	OS=$(uname -s)
	if [[ $OS == MINGW64_NT* ]]; then
		echo "windows"
	else
		case "$OS" in
		Darwin*) echo "macos" ;;
		Linux*) echo "linux" ;;
		MSYS*) echo "windows" ;;
		cygwin*) echo "windows" ;;
		*) echo "other" ;;
		esac
	fi
}

# provides the CPU architecture name in the format used in the release archive filenames
cpu_name() {
	cpu_name=$(uname -m)
	case $cpu_name in
	x86_64 | x86-64 | x64 | amd64) echo "intel_64" ;;
	aarch64 | arm64) echo "arm_64" ;;
	*) echo "other" ;;
	esac
}

# provides the URL from which to download the installation archive for the given OS and cpu type
download_url() {
	OS=$1
	CPU=$2
	EXT=$(archive_ext "$OS")
	echo "https://github.com/kevgo/run-that-app/releases/download/v${VERSION_TO_INSTALL}/run_that_app_${OS}_${CPU}.${EXT}"
}

archive_ext() {
	OS=$1
	if [ "$OS" = windows ]; then
		echo "zip"
	else
		echo "tar.gz"
	fi
}

download_and_extract() {
	URL=$1
	OS=$2
	FILENAME=$3
	create_folder "$TMP_DIR"
	if [ "$OS" = "windows" ]; then
		need_cmd unzip
		curl -Lo "$TMP_DIR/run-that-app.zip" "$URL"
		(cd $TMP_DIR && unzip run-that-app.zip "$FILENAME")
	else
		need_cmd tar
		curl -L "$URL" | tar xz --directory "$TMP_DIR"
	fi
	create_folder "$DEST_FOLDER"
	mv "$TMP_DIR/$FILENAME" "$DEST_FOLDER"
	rm -rf $TMP_DIR
}

executable_filename() {
	OS=$1
	if [ "$OS" = "windows" ]; then
		echo "run-that-app.exe"
	else
		echo "run-that-app"
	fi
}

check_already_installed() {
	DEST_PATH=$1
	if [ -f "$DEST_PATH" ]; then
		INSTALLED_VERSION=$($DEST_PATH -V)
		if [ "$INSTALLED_VERSION" = "$VERSION_TO_INSTALL" ]; then
			echo "You already have run-that-app $VERSION_TO_INSTALL installed."
			exit 0
		fi
	fi
}

# verifies that the command with the given name exists on this system
need_cmd() {
	if ! check_cmd "$1"; then
		err "required command not found: $1"
	fi
}

# indicates whether the command with the given name exists
check_cmd() {
	command -v "$1" >/dev/null 2>&1
}

# aborts with the given error message
err() {
	echo "$@" >&2
	exit 1
}

main "$@" || exit 1
