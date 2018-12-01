# Designs and Considerations of Stacc
For anyone reading this, this is just meant to be my own design notes for the language, so I can keep track of my train of thought.

## Language Design
The language is meant to be purely stack-based; no cheating allowed!
Everything that happens in the language, must be applicable to a stack; so only pushing, popping, and appending is permitted.

The general idea is to have everything be performed as stack operations without the need of any explicit pushing or popping; this will all be done by the operators that act on the stack.

Example program:
```
72 to_rad # Pushes 1.257 to the stack
duplicate # Pushes a copy of the top onto the stack
cos 300 * # Take cos of the top of the stack and multiply the result by 300
swap      # Swaps the two topmost elements
sin 300 * # Take sin of the topmost element and multiply by 300
"Coordinate is ({}, {})" print # Prints the 2 topmost elements
```

The benefit of using the stack, is that stack manipulations spring out naturally from this approach.

Writing a number pushes it to the stack, writing an operator, pops the number off the stack, performs some operation on it, and pushes it back on.
The main draw of this, is that it lets the programmer perform implicit operations on values already present.

For example, writing `44 30 *` multiplies 44 by 30, but writing `20 +` adds 20 to whatever's already on the stack, and writing `+` simply adds together the top two items.
By using a stack, results need not be assigned to a variable, they can simply exist on the stack until they are required.
In the example program, `duplicate` copies the topmost value of the stack and pushes another copy of the value on top. 
The original value remains untouched until needed later.

Observe:
```
72 to_rad                      # operation: 72,        stack: [72]
                               # operation: to_rad,    stack: [1.257]
duplicate                      # operation: duplicate, stack: [1.257, 1.257]
cos 300 *                      # operation: cos,       stack: [1.257, 0.309]
                               # operation: 300,       stack: [1.257, 0.309, 300]
                               # operation: *,         stack: [1.257, 92.71]
swap                           # operation: swap,      stack: [92.71, 1.257]
sin 300 *                      # operation: sin,       stack: [92.71, 0.951]
                               # operation: 300,       stack: [92.71, 0.951, 300]
                               # operation: *,         stack: [92.71, 285.3]
"Coordinate is ({}, {})" print # operation: "Coor...", stack: [92.71, 285.3, "Coordinate is ({}, {})"]
                               # operation: print,     stack: []
```

I have attempted to annotate each line of the program with the results of the stack operations. 
Everything following a `#` is a comment, and is not parsed by the interpreter.

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

**NOTE:** One could argue that perhaps variables should be defined as `$name value def` instead, to make it consistent with the RPN operand rules;
and while that is indeed a valid ciritcism, I feel that being able to write `$name def` and use what's already on the stack is more useful.

**PROBLEM:** How do I prevent lookup resolution from removing the variable name before `def` can define it?

Possible solutions:
* let `def` redefine it in the current scope, or if in a sub-scope, let it define it anew in there.

**SOLUTION:** The solution to this has been disussed [here](#the-issue-of-namespace-resolution-during-variable-declaration).

TL;DR: it involves using a different kind of notation for a variable when setting than when calling.

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
    stack: [44.5, 2, /, $a, +] # NB: this is just the current stack, not the full one

Inner Scope:
    parent: Some(5eff1f4b) # refers back to the root scope
    id: 6ddf29b8
    stack: [2, pi, *, $a, $b, -, +]
```

Note that both stacks here contain the same variable `$a`, but it is unclear what scope this variable was derived from, for that we need to have a peek in the symbol table:

```yaml
5eff1f4b: # main scope
    $a: 13
    $inner_scope: # the same inner scope as above, here defined as a variable
        parent: Some(5eff1f4b)
        id: 6ddf29b8
        stack: [2, pi, *, $a, $b, -, +]
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
    $distance def          # pop the distance off the stack and assign it to $distance
    $angle cos $distance * # push cos(angle) * distance onto the stack
    $angle sin $distance * # push sin(angle) * distance onto the stack
) $calc_coords def

72 $angle def # define local angle

$angle 300 $calc_coords $result def
```

First we define an angle and a distance within the local scope (delimited by `()`), then we assign 72 to be our angle in the global scope.

Without scoping rules, the resolution of `$angle` would not be possible; but **with** scoping rules, the `$angle` outside of `$calc_coords` would not clash with the name inside. 
During symbol resolution, the angle inside `$calc_coords` would be called `b3bd15d9$angle`, while the one in the outer scope would be called `5eff1f4b$angle`.

**PROBLEM:** I need to figure out how to handle removing a scope. If a scope gets re-defined or deleted, its local variables need to get invalidated somehow.

Possible solutions:
* Have the lookup operation performed by `def` clear out any values if they already exist
* The `insert` method on hashmaps overwrites values stored within when written to, this should clear out anything within. **WARNING:** This approach could cause issues with sub-scopes. Further testing required.

#### Keeping Track of Scope
In order to keep scope resolution safe, sane, and healthy; we need to somehow be able to keep track of the current context, while also executing the new scope.

Let us consider this simple code:
```
(+ 3 -) $foo def
2 3 $foo 4 *
```
we first define `$foo` to be a scoped variable, then on the very next line we have to resolve the name and expand it.

There are three general approaches to do this: 
1. Either we splice the result of `$foo` directly into the code, like this: `2 3 + 3 - 4 *`, 
2. The code switches context to the scope, executes it, then returns back to the main scope without splicing anything.
3. We don't have one scope, but instead a stack of scopes.

**The first approach:** 
It doesn't really adhere well to the stackiness of the program, as it somehow has to push the execution of the scope ahead of where it's gotten to.
It also creates issues where the interpreter has to deal with both fully lexed code and plaintext, which makes designing the interpreter much harder.

**The second approach:**
Jumps to a different place in its execution, which doesn't violate the premise of the stack.
It does require appending the results of whatever operations it did back onto the stack so they don't disappear into the æther.
This approach doesn't cause any issues with the unlexed plaintext, but it does create the issue of having the code intelligently switch contexts.

This in turn can also be done in two ways:
1. Have the parser simply call into the new scope, and return once done
2. Have the parser save the name of the current scope before starting the execution of the new one, restoring the previous scope afterwards.

**The third approach:**
Instead of doing any strange jumping, we simply have a stack of scopes allocated somewhere.
Once the scope is done executing it is popped off of the stack, and whatever value (or values) are left on that scope's stack, are appended to the stack of the surrounding scope.

This approach solves 2 problems:
1. Keeping track of the scope becomes a matter of simply reading the topmost stack frame
2. Whatever values remain are treated as "results", which are then added to the parent scope. This does mean the programmer has to carefully clean the scope up before returning to avoid unnecessarily cluttering the parent stack, but that's a compromise I'm willing to take.

All of the approaches can be simplified if the language switches to a two-pass interpreter.
That is, a lexing step, followed by an execution step.

#### Finding the Parent's Parent
I stumbled upon a roadblock: If a scope is nested two levels, how do we find the parent's parent?
After all, while scopes can be arbitrarily nested on paper, by the virtue of how the data structure is designed, they can't in practice.

A parent can get a reference to its child via the symbol table, but a child can't get a reference to its parent, only its parent's symbol table.
This means a child can get a reference to itself and its siblings, but not its parent.
As such, children don't have access to their parent's parent because the parent ID isn't stored in the symbol table, it's stored in the parent object.

To solve this, I can do one of two things:
1. I can try to make the object hierarchy more like the way a filesystem does it: a folder has a reference to itself, its parent, and all its children,
2. I can add a special symbol in the symbol table, called something like `$$PARENT$$`, which stores the parent ID if any.

The benefit of the references is that it on paper seems to be the most sane solution; 
however, with Rust's borrow checker and lifetimes, it'll most likely lead to `Rc` hell instead.

The benefit of the special entry however, essentially means I can stay within the lookup table entirely, without having to worry about referencing any of the other children.
Granted, this method is potentially much less efficient, as it potentially requires the parser to repeatedly check the symbol table to find the value.
This problem is undecidable, since there's a possibility a scope could set itself as the parent, causing an infinite loop.

Why would I want to look up a parent's parent though?
Simple: To find a variable.
Given that scopes can be nested, I have to be able to find the variable in the right scope.
This essentially means traversing the symbol table looking for the entry.
Since the variable could be mentioned 80 scopes in, the interpreter needs to be able to handle finding it.

Using the example from before, now with the new entry:

```yaml
5eff1f4b: # main scope
    $a: 13
    $inner_scope: # the same inner scope as above, here defined as a variable
        parent: Some(5eff1f4b)
        id: 6ddf29b8
        stack: [2, pi, *, $a, $b, -, +]
6ddf29b8:
    $$PARENT$$: 5eff1f4b
    $b: 45
```
It's clear that looking up the parent of `6ddf29b8` only requires the interpreter to find the `$$PARENT$$` entry, and not the actual reference to the object, which is inaccessible from that key.

**PROBLEM:** How do I repeatedly iterate through the symbol table in a safe, sane, and reliable way that agrees with Rust's borrow checker?

Possible solutions:
* Clever application of a loop (possible solution [[here](https://stackoverflow.com/a/37987197/1351298)]),
* Using reference counting (yikes),
* Recursion via a helper function of the form `fn(&mut ScopeTable, Uuid, String) -> Result<()>`.

**SOLUTION:**
The actual solution to this problem was less-than straightforward, but it ended up being a more modular solution in the end.
I ended up getting help on Reddit (Thank you /u/christophe_biocca!).

The entire solution hinges on the introduction of a new sum type, which I've named `Lookup`:
```rust
pub enum Lookup {
    Found(Uuid),
    CheckParent(Uuid),
    NotFound,
}
```
The idea here, is to have this be the result of a lookup, which can have one of three different outcomes: 
1. It was found; in which case we simply return the ID of the scope in which it is located.
2. It was not found, but the scope has a parent; in which case the parent ID is returned.
3. It was not found and the scope has no parent; in which case `NotFound` is returned.

This is all done via a `lookup` function a-la this:
```rust
pub fn lookup(table: &mut ScopeTable, id: Uuid, symbol: &str) -> Lookup {
    let symbol_table = table.get_mut(&id).unwrap();

    if symbol_table.contains_key(symbol) {
        Found(id)
    } else if let Some(parent_id) = get_parent_id(symbol_table) {
        CheckParent(parent_id)
    } else {
        NotFound
    }
}
```
The result of this function is then used in a function called `find_symbol`, which will keep trying to find the symbol recursively until it hits the root scope, if nothing is found, it returns an error.

It has been implemented as follows:
```rust
pub fn find_symbol<'a>(table: &'a mut ScopeTable, id: Uuid, symbol: &str) -> Result<&'a mut SymbolTable> {
    match lookup(table, id, symbol) {
        Found(id) => Ok(table.get_mut(&id).unwrap()),
        CheckParent(id) => find_symbol(table, id, symbol),
        NotFound => Err(Error::UnknownIdentifier),
    }
}
```
The lifetime parameter `'a` is required here, because the result is a reference **into** the scope table.
The lifetime is basically a guarantee to the compiler that says the result will only ever live as long as the context it came from.

Armed with these two new functions, it is possible to gracefully deal with the `def` and `set` keywords, as well as dealing with namespace resolution (discussed in the next chapter).

#### The Issue of Namespace Resolution During Variable Declaration
Put simply, given an expression like `45 $angle def`, there's nothing that prevents the interpreter from looking up `$angle` in the symbol table, and replacing it with the existing value within the table.
Most languages seem to deal with this fairly well without any issues, as they let the programmer define nested variables as they please.

Consider the following examples:

```c
// C/C++
int foo() {
    int a = 5;
    int b = 5;
    int bar() {
        a = 18;
    }
    bar();
    return a + b;
}
```
```python
# Python
def foo():
    a = 5
    b = 5
    def bar():
        a = 18
    bar()
    return a + b
```
```ruby
# Ruby
def foo()
    a = 5
    b = 5
    def bar()
        a = 18
    end
    bar()
    return a + b
end
```
In the C/C++ case, we define a nested function, which sets the local variable a to be 18; running `foo()` will give back the number 23.
However, in the Python and Ruby cases, neither of which use a specific keyword for defining variables, the `a` set in the inner function does _not_ reference the `a` in the outer scope, so both languages will return 10 upon calling `foo()`.

This difference is important, because it essentially showcases two different approaches to the same thing: Either variable assignment/definition obeys scoping rules, or they don't.
Now, in most cases, defining a nested inner function is going to be rare, so the Python/Ruby folk will likely get away with this most of the time.

Why am I bringing this up? Well, either I could go the Ruby and Python route, and say that whenever I encounter a variable in an inner scope, I'm simply not going to care whether or not it exists in the outer scope; namespace resolution will not happen, because it's the first time the variable is defined.
However, I like the idea of being able to reference a previously defined variable within my functions, as it gives rise to some more interesting programming patterns.

Now, in this language, we _do_ use a special keyword for defining variables, `def`; the main issue arises from the stack-based nature of how the language is read.
In C the keyword comes before the variable name, so had the language been interpreted the interpreter would be able to see that the keyword that follows shouldn't be looked up, only defined.
We do not have this luxury here.
The variable name is read _before_ the `def` keyword, and as such there's no logical way of avoiding namespace resolution, unless I'm planning on tagging variables with metadata about their variable of origin.
Doing that would let the `def` keyword look up the origin and reassign the value.
This approach however, feels extremely clunky, and would require every token in the interpreter be of the form `Kind(Data, Option<Origin>)`.

There are two languages that spring to mind that solve this issue in a rather interesting way: _Bash_ and _PostScript_.

In both of these languages, variable declaration differs from variable calls in that they either add or remove a symbol.
in Bash you write `FOO=bar` which declares a variable called `$FOO`, which adds the `$`.
In PostScript you write `/foo bar def`, which defines a variable called `foo` without the `/`.

My best guess for PostScript would be that it suffers the same issue this does, so it uses the `/` to tell the interpreter to not evaluate the symbol and wait for the `def` keyword.

The issue remains to find a syntax that isn't too awkward or hard to understand.
I want to keep the `$` prefix, for the reasons mentioned in the _Variables_ section.

**DECISION:** I have decided to have variable declaration and invocation follow the _Bash_ way of doing things, meaning
```
42 meaning def
```
will define the variable assuming there's no pre-existing keyword with that name, and writing `$meaning` will invoke it.
It solves several problems:
1. There's no need for some ugly prefix syntax like `#foo` or `:foo` that stands out for defining the variable, which would lead to the fun situation of `$foo 1 + #foo set` being a required syntax for updating.
2. I don't have to define 3 different kinds of symbols in my code, each to handle a separate usecase, now everything can piggyback on the same infrastructure. Granted, this may change later.

It does however come with a significant con though: writing something like `foo bar baz` is now legal syntax, because all it does is push unevaluated keywords onto the stack.

### Conditionals
As with any other programming language, conditionals are a must-have for the language.
Though as one might expect, being stack-based leads to some interesting considerations:

1. Is it possible to have variable-length conditionals a-la `switch/match` or `cond`?
    1. If so, what are the options?
    2. If not, what other options do we have?
2. `if` is the obvious first case, but should it have the `then` branch first or the `else` branch first?
3. Should I go in Lisp's footsteps and also include `when` and `unless` for single-branch if-statements?

Let's for a second go with first having the `then` branch, then the `else` branch afterwards.
For some reason it feels more natural that way. 
Later iterations could change this.

Not much to say except to look at an example:
```
44
(3 +) (2 -) $a 19 < if
```
Here we push 44 onto the stack, and then we either add 3 or subtract 2 depending on whether or not `$a` is less than 19.

Now, it may look like the `if` is taking 5 arguments here, but remember that we're dealing with a stack, so before the `if` keyword is even reached, the expression `$a 19 <` has already been evaluated.

My idea for putting the condition near the `if` keyword here, is the same as for `def`, because it lets us utilise the stack right away.

If we only want a single branch in our code though, writing an if-statement wouldn't work, since it explicitly pops 3 values off the stack.

For that we can have `when` and `unless`, which act like each others' inverses:

```
44
(3 +) $a 19 < when
```
adds 3 to 44 if the condition is true
```
44
(2 -) $a 19 < unless
```
subtracts 2 from 44 if the condition is false.

**PROBLEM:** Is it doable to make a switch statement?

Possible solutions:
* put all the cases into their own scope, and use the length of said scope to determine number of cases

### Matching Brackets
Bracket matching is not entirely trivial.
First there's the issue of actually matching brackets (parehtneses and braces fall under this category too).
given the input `[foo bar baz bing]` the parser should correctly store this as a single `List` token on the stack.

Trying to write `[foo bar baz bing` will not go noticed by the parser at all until someone either tries to perform an operation, and the parser complains about not being able to perform addition on `[`, or they try to close with the wrong kind of bracket, in which case there will be a bracket mismatch.

**PROBLEM:** In general there are 5 issues to be resolved:
1. Matching `[` with `)` or `(` with `]` should throw an error
2. Trying to perform any kind of operation on an opening bracket should throw an error
3. Writing `)` or `]` without a staring bracket should also throw an error.
4. Certain parentheses, like `()` should not resolve symbols; this requires a notion of "verbatim-ness"
    * Writing `(2 3 +)` does not result in a scope containing `5`.
    * A scope isn't very useful if everything within it is already evaluated before it's needed.
    * This could also be used for adding verbatim lists, which can later be evealuated with `eval`
5. In interactive mode (if that ever becomes a thing) a match error should revert the stack back to just before the offending operation was introduced.
    * Writing `( 2 +` should return the stack back to `( 2` for example.
    * This is to allow the user to correct their mistakes without having to redo everything.
    * In the non-interactive version, this isn't a problem.

Point 1, 2, and 3 are fairly trivial (I hope), simply involving popping values off the stack until a matching brace is found, then wrapping the popped values in their corresponding token.

Point 4 is a bit trickier, since it involves somehow telling the symbol parser not to evaluate the symbols at all.
Bypassing it entirely might be in order.