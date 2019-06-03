## Rusty Mandelbulb

This project implement the Mandelbulb algorithm in Rust. It works by generating X-axis slices of the object (like an MRI scan). Afterwards these slices can be combined with a FIJI/Meshlab workflow to produce a ready to print STL file.

## Disclaimer
This project is as of now a (fun) way for me to discover and explore the Rust language. It has rooms for (huge) improvements that will be added as I get more comfortable with the language, please bear with me ;-) !

## How to generate the fractal

 - Run the Rust program : `cd $RSHOME/mandelbulb ; mkdir temp ; cargo run`
 - Open FIJI : *File->Import->Image sequence*
 - Navigate to the mandelbulb folder and select *1.png* ; fill the window with other informations (Number of images, Starting image, Increment, scale). Be sure to check **Sort names numerically** and **Use virtual stack**
 - *File->Save as->PGM* will create a ***.pnm** file (change file name if relevant)
 - Once the PNM is saved *File->Save as->Wavefront .OBJ*
 - In the triangulate window leave everything as default and click OK
 - Wait for the generation to be processed before quitting FIJI
 - Open the resulting OBJ file in Meshlab
 - *Filters->Normals, Curvature and Orientation->Invert Faces Orientation* check **Force Flip**, click *Apply* and close the window
 - *File->Export Mesh As* select **STL** and click *Save*
 
 You now have a STL Mandelbub that is ready to 3D print.
## Rust code constants

You can modify theses constants in **[main.rs](https://gitlab.com/thornale/rusty-mandelbulb/blob/master/src/main.rs)**

 - **STEP**:  The number of slices and the size of each slices
 - **EXPONENT**: The exponent of the Mandelbrot formula Z(n+1) = Z(n)^EXPONENT + C 
 - **MAX_ITER**: The maximum number of iterations of the formula for a given (x,y,z) point.