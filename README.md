![grafik](https://user-images.githubusercontent.com/50629201/213842987-5344e80f-f420-4171-83b8-d5423caecab2.png)

A simple command-line tool for solving simple sokoban levels written in rust.  
(and when I say simple levels, I mean levels that require <40 steps to beat)  
  
This uses a simple breadth-first search to find the right combination of steps  
to take for the board to get into it's finished state.  
This will always find a solution with the shortest possible path but has a terrible  
runtime complexity. My goal with this project was to learn a little bit more about  
the rust programming language and not to solve a [PSPACE-complete](http://cl-informatik.uibk.ac.at/teaching/ss07/alth/material/culberson97sokoban.pdf) problem :P  
  
**solution for examples/1.txt:**  
![grafik](https://user-images.githubusercontent.com/50629201/213843493-46ac021a-1291-44df-ba18-6218b668832b.png)
  
###### examples/1.txt and examples/2.txt represent the fist two levels in David W. Skinner's Microban level pack  
