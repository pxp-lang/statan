# Statan

Statan is an early-stage static analyser for PHP and [PXP](https://pxplang.org) projects.

## Overview

The design of Statan is heavily inspired by both [PHPStan](https://phpstan.org) and [Psalm](https://psalm.dev). These tools have existed for years and are continously growing in their abilities.

The problem with such tools is the performance bottleneck of the language that they are written in, PHP. By writing a similar tool in a faster programming language such as Rust, it allows static analysis to be used on older, larger and more complex applications.

In the short-term, Statan's goal is on covering the simplest of static analysis rules; ensuring functions and methods exist, arguments are of the correct type and more.

Once a baseline implementation has been established, more advanced features such as generics, type aliases and more will be implemented based on usage in real-world code (generics likely the highest priority).

From there, performance will be at the top of the list. It's important that the analysis being performed is correct before spending time on optimisations.

## Installation

The recommended method of installation is building from source. The following instructions will guide you through the process.

1. Ensure you have a Rust toolchain installed. If you do not, run the official Rustup installer using the following command.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone this repository.

```sh
git clone git@github.com:pxp-lang/statan.git
```

3. Use Cargo to install Statan locally.

```sh
cargo install --path .
```

## Usage

Statan currently provides an `analyse` command that accepts the path to a single file or directory that you wish to analyse.

```sh
statan analyse src/example.php # Analyse a single file
statan analyse src/            # Analyse a directory
```

The binary does not ship with stubs or definitions for PHP's native functions, classes or interfaces. In order for Statan to discover and understand those definitions, you must install a third-party stubs package in your project. We recommend using the [stubs provided by PHPStan](https://github.com/phpstan/php-8-stubs).

```sh
composer require phpstan/php-8-stubs --dev
```

## Contributing

You can contribute to Statan in a couple of different ways.

### Reporting issues

If you have tried Statan on a project and encountered one of the things in the list below, please open an issue with the minimum required amount of code to reproduce the problem.
* Runtime errors (panics, seg faults, etc)
* Analysis incorrectly produced an error
* Analysis missed an error

### Code contributions

If you see an issue on the repository and would like to have a go at closing it, all PRs are welcome!

> The project is constantly changing and being improved, so there's a chance the issue you're looking at is already being handled. If you see an issue that has been assigned to somebody, there's a high chance it's already being worked on. 

## Credits

* [Ryan Chandler](https://github.com/ryangjchandler)