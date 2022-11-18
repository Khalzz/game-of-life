# Game of life

This is a cellular automata based on 3 rules:

+ Every dead cell with 3 neighbors gets alive.
+ Every alive cell with 2 or 3 keeps alive.
+ Every cell with less than 2 neighbors dies of loneliness.
+ Every alive cell with more than 3 neighbors dies of overpopulation.

---

## Construction

If you are planning on doing this project too, you have to keep in mind this way of implementing it:

+ First you have to do a 2D array (or vector) and a "Next generation array", as a copy of the last one.
+ Then you need to create a way to show it in the screen (and add more cells if you want)
+ Then you have to create the "neighbors counter".
+ Based on the amount of neighbors you follow the ruleset (with this you will apply the changes on the "next gen array").
+ Finally you set the base array equals to the next gen, and end this section of the loop, on the next one you will get updates.

---

## Controls

For this implementation the controls are:

+ `ESC`: Quit.
+ `Space`: Start or pause the execution (if the background is red it's paused, if it's black it's running).
+ `Mouse left clic`: To add cells on a certain place of the grid.
+ `Mouse right clic`: To delete cells on a certain place of the grid.
