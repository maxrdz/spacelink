# Spacelink

A graphical application for sending SMS/MMS messages, built with libcosmic
for the COSMIC desktop environment, and in the future, the COSMIC mobile
shell.

## Building

To build Spacelink, run:

    meson setup build -Dprofile=debug
    meson compile -C build

## Installing

To install a build of Spacelink, run:

    meson install -C build

## Cross Compiling

PLEASE install cross-rs via:

    cargo install cross --git https://github.com/cross-rs/cross

The cross project is in a weird state where it doesn't have much motivation
and/or time to cut a release, so you need to pull from the main branch to
get a lot of bug fixes since the 'latest' release as of December 2024.

Then, you can run:

    meson setup build -Dtarget=aarch64-unknown-linux-gnu
    meson compile -C build
