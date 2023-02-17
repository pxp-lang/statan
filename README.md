# Statan

Statan is an early-stage static analyser for PHP and [PXP](https://pxplang.org) projects.

## Overview

The design of Statan is heavily inspired by both [PHPStan](https://phpstan.org) and [Psalm](https://psalm.dev). These tools have existed for years and are continously growing in their abilities.

The problem with such tools is the performance bottleneck of the language that they are written in, PHP. By writing a similar tool in a faster programming language such as Rust, it allows static analysis to be used on older, larger and more complex applications.

In the short-term, Statan's goal is on covering the simplest of static analysis rules; ensuring functions and methods exist, arguments are of the correct type and more.

Once a baseline implementation has been established, more advanced features such as generics, type aliases and more will be implemented based on usage in real-world code (generics likely the highest priority).

From there, performance will be at the top of the list. It's important that the analysis being performed is correct before spending time on optimisations.

## Contributors

* [Ryan Chandler](https://github.com/ryangjchandler)