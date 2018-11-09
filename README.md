# The Stacc Programming Language
Stacc is a toy language developed to explore some ideas I had about how a purely stack-based programming language might work.

## Design Considerations
The main draw of this design, is to make something that is incredibly quick and light-weight to parse and execute. 
Make this a single-pass interpreter if possible.

## Preliminary Example 
```
Math import

72 to_rad # Pushes 1.257 to the stack
duplicate # Pushes a copy of the top onto the stack
cos 300 * # Take cos of the top of the stack and multiply the result by 300
swap      # Swaps the two topmost elements
sin 300 * # Take sin of the topmost element and multiply by 300
"Coordinate is ({}, {})" print # Prints the 2 topmost elements
```