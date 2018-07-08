# microstatus [![](https://img.shields.io/crates/v/microstatus.svg)](https://crates.io/crates/microstatus) [![License](https://img.shields.io/badge/license-ISC-blue.svg)](/LICENSE) [![Build status](https://travis-ci.org/Arkanosis/microstatus.svg?branch=master)](https://travis-ci.org/Arkanosis/microstatus)

**microstatus** is a lightweight [Mastodon](https://github.com/tootsuite/mastodon)- and [GNU social](https://gnu.io/social/)-compatible [OStatus](https://www.w3.org/community/ostatus/) and [ActivityPub](https://www.w3.org/TR/activitypub/) server implementation.

## Current Status

microstatus is still under active design and not yet ready for mainstream usage.

## Design goals

microstatus aims at the following objectives:
* provide a complete, standard compliant implementation of the OStatus and ActivityPub protocols;
* be extremely lightweight, self-contained and easy to deploy;
* federate with Mastodon and GNU social instances (at least);
* use a hackable, UNIX-friendly storage format;
* come with a nice client API.

microstatus (nice to have) non-goals include:
* providing a *scalable* implementation of the OStatus and ActivityPub protocols;
* handling billions of users;
* being compatible with Mastodon's, GNU social's or Twitter's *client* API;
* coming with a nice web or mobile UI.

## Compilation

Run `cargo build --release` in your working copy.

## Installation

Copy the `microstatus` binary wherever you want.

## Usage

```console
Usage: microstatus <working-directory>
       microstatus -h | --help
       microstatus --version

Arguments:
    working-directory working directory (file storage)

Options:
    -h, --help  Show this screen.
    --version   Show version.
```

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/microstatus/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/microstatus/issues).

## License

microstatus is copyright (C) 2017-2018 Jérémie Roquet <jroquet@arkanosis.net> and licensed under the ISC license.
