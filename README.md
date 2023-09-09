# Cell evolution simulation

This program emulates a virtual 2D field with cells. Each cell is like a tiny virtual machine.
It has its own instruction set, memory with instructions, branching logic and instruction modifiers.
It can move, it can attack other cells, it can check it's surroundings, and most importantly,
it can make children!  
When a cell reproduces, a child gets a copy of parent's genome. But there's a 25% chance that one
gene will be mutated. That's how the whole "evolution" works. Cells get replicated, some cells mutate,
thus creating different better (or worse) variations of cells.

Currently, simulation parameters can be changed at compile time, by changing the variables declared
in `src/config.rs`. I plan to implement setting all those parameters at runtime.

## Some interesting facts about the implementation:
- The simulation runs in a separate thread. This allows for better performance, and also allows
  to run multiple simulations at the same time.
- `mimalloc` is used instead of the default allocator.
  This is because each frame, a whole copy of the map, along with some other information gets sent
  between threads. This is a lot of memory allocation/copying, as you can imagine.
  `mimalloc` basically gives a 2x performance boost, for free.

## TODO
- Configuration of various simulation parameters at runtime
- Selecting a cell, and inspecting it's internal state
    - Modifying that cell's internal state
    - Saving/loading cell to the filesystem
- Saving/loading whole map to filesystem
- Possibly, even more performance optimizations