# Designs and Considerations of Stacc
For anyone reading this, this is just meant to be my own design notes for the language, so I can keep track of my train of thought.

## Language Design
The language is meant to be purely stack-based; no cheating allowed!
Everything that happens in the language, must be applicable to a stack; so only pushing, popping, and appending is permitted.

### Variables
Variables are of course present in the language, and they (sorta) break the stackiness of the language.
They can be thought of as registers in assembly, where if you need something to be stored independently of the stack, they'd be the way to go.

The initial idea of variables, is that they'd be identified like in PHP: starting with a `$`.
This has two nice upshots:

1. They're easy for the parser to identify, as anything that starts with `$` is automatically going to get looked up in the symbol table.
2. They allow for easy string-interpolation, as writing `"Name: $name, age: $age"` would cause the parser to just drop the values into the string.

The syntax for defining a variable would be 
* `value $name def`,
* `$name def` if the value is already on the stack,
* `def` if the value and variable-name are already on the stack.

**NOTE:** Defining a variable removes its data from the stack entirely. 
It will only exist in the lookup table from then on.

**PROBLEM:** How do I prevent lookup resolution from removing the variable name before `def` can define it?

Possible solutions:
* let `def` redefine it in the current scope, or if in a sub-scope, let it define it anew in there.

### Functions
Functions are a funny subject in the world of stack-based evaluation.

Like in the assembly language, "function arguments" as a concept doesn't really make any sense, instead functions will pop their required number of arguments off the stack and use them as necessary.

Because of this, functions as a concept do not need to exist at all. 
Instead, re-callable stacks are introduced as first-class citizens.

These re-callable stacks would have their own local scopes (see "Scoping" section for details), in which locally defined variabels would rule.
Each scope would still be tied to the global stack though, so one has to be careful with how one manipulates it to not screw something up royally.

The syntax of functions is essentially the same as variables, except the scopes are surrounded in parentheses:
```
(copy *) $square def
```
defines a function called `$square` that copies the topmost value of the stack, and then multiplies it by itself, squaring it.

### Scoping
NOTE: before we start this discussion, let me make it clear that all the hexadecimal IDs in the examples are randomly generated, and not actually indicative of any _actual_ IDs. In fact, using Rust's UUID library, the IDs would be 128-bit, whereas the example ones are only 32-bit.

Scoping (if applicable) would be done by assigning a UUID to each scope, and providing a look-up table, which gets the corresponding variable from the given scope.

Scopes would require 3 distinct fields: an ID, a parent ID (if applicable), and its own local stack.
A notable design decision would be that a variable (if looked up) need only be searched for in its or its parents' lookup tables, not its siblings.

So let us imagine we have the following scopes:

```yaml
Main Scope: 
    parent: None # we're in the root scope, so no parent exists
    id: 5eff1f4b # needs to know its own id
    stack: [44.5, 2, /, $a, +] #NB: this is just the current stack, not the full one
Inner Scope:
    parent: Some(5eff1f4b) # refers back to the root scope
    id: 6ddf29b8
    stack: [2, pi, *, $a, $b, -, +]
```

Note that both stacks here contain the same variable `$a`, but it is unclear what scope this variable was derived from, for that we need to have a peek in the symbol table:

```yaml
5eff1f4b: # main scope
    $a: 13
6ddf29b8:
    $b: 45
```

Upon lookup, it becomes clear that `$a` is only defined in the main scope, so the application of `$a` in the inner scope would need to be relegated to looking up the parent.

This could be done by either somehow appending the parent's scope to the variable name a-la `5eff1f4b$a`, or looking it up directly

#### The Purpose of Scoping
The general idea would be to let the programmer define variables of the same name within sub-scopes.

Imagine the following program:

```
(
    $angle def             # pop angle off the stack and assign it to $angle
    $distance def          # pop the distance off the stack and assign it to distance
    $angle cos $distance * # push cos(angle) * distance onto the stack
    $angle sin $distance * # push sin(angle) * distance onto the stack
) $calc_coords def

72 $angle def # define local angle

$angle 300 $calc_coords $result def
```

First we define an angle and a distance within the local scope (delimited by `()`), then we assign 72 to be our angle in the global scope.

Without scoping rules, the resolution of `$angle` would not be possible; but **with** scoping rules, the `$angle` outside of `$calc_coords` would not clash with the name inside. 
During symbol resolution, the angle inside `$calc_coords` would be called `b3bd15d9$angle`, while the one in the outer scope would be called `5eff1f4b$angle`.