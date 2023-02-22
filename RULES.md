# Rules

This document contains a list of all rules that Statan currently supports. Each section contains a brief description for the rule as well as a code sample demonstrating the issues it looks for.

## Table of Contents

* [`ValidFunctionRule`](#validfunctionrule)

### `ValidFunctionRule`

This rule is responsible for checking all function call expressions in your code. It runs the following checks:

1. If the function you're calling exists.
2. That the number of arguments you're passing is correct.
3. That positional arguments do not follow a named argument.
4. That named parameters actually exist on the function.
5. That arguments are of the correct type when compared to the function parameter.

```php
function foo(int $a, int $b) {
    // ...
}

foo("a");
```

The code above will let you know that the first argument of type `string` is not compatible with the parameter `$a` of type `int`. It will also warn you that you're missing the 2nd required argument for parameter `$b`.

```php
function foo(string ...$args) {
    // ...
}

foo(1, 2, 3);
```

The code above will let you know that the arguments that are collected into `$args` must be of type `string`, since only `int` values are being passed through.
