<div align="center">
    <img src="./resources/icons/hicolor/scalable/apps/icon.svg">
    <h1>Spacelink</h1>
    <p>
        A graphical application for sending SMS/MMS messages, built with libcosmic
        for the COSMIC desktop environment, and in the future, the COSMIC mobile
        shell.
    </p>
    <a href="https://gitlab.com/maxrdz/spacelink/-/pipelines/latest"><img src="https://gitlab.com/maxrdz/spacelink/badges/master/pipeline.svg" alt="Pipeline" /></a>
</div>

## Building

Spacelink uses Git for version control, Meson/Ninja as the build
system, and Flatpak for packaging the application.
The quickest way to build for release is to do the following:

### Getting the source

```sh
git clone https://gitlab.com/maxrdz/spacelink.git
cd spacelink/
```

### Dependencies

- [libcosmic](https://github.com/pop-os/libcosmic)
- [ModemManager](https://gitlab.freedesktop.org/mobile-broadband/ModemManager/)
- [mmsd-tng](https://gitlab.com/kop316/mmsd/)
- [feedbackd](https://source.puri.sm/Librem5/feedbackd)

To build Spacelink, run the following in the repo root directory:

```sh
meson setup build
meson compile -C build
```

You can append the `-Dprofile=debug` argument to build for debug.

### Building the Flatpak

First, install the required dependencies on your Flatpak environment:

```sh
# Install flatpak builder and runtime
flatpak install flathub org.flatpak.Builder org.freedesktop.Platform
# Install runtime extensions (rust toolchain & LLVM)
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable org.freedesktop.Sdk.Extension.llvm18
```

To build the flatpak, run:

```sh
flatpak run org.flatpak.Builder --user --install --force-clean flatpakbuild/ com.maxrdz.Spacelink.json
```

## Installing

To install a build of Spacelink, run:

```sh
meson install -C build
```

## Cross Compiling

PLEASE install cross-rs via:

```sh
cargo install cross --git https://github.com/cross-rs/cross
```

The cross project is in a weird state where it doesn't have much motivation
and/or time to cut a release, so you need to pull from the main branch to
get a lot of bug fixes since the 'latest' release as of December 2024.

Then, you can run:

```sh
meson setup build -Dtarget=aarch64-unknown-linux-gnu
meson compile -C build
```

## Copyright and Software License

Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

"Spacelink" can be found at https://gitlab.com/maxrdz/spacelink

"Spacelink" is distributed under the terms of the GNU Affero General Public
License, either version 3.0 or, at your option, any later
version WITHOUT ANY WARRANTY. You can read the full copy of
the software license in the COPYING file.
