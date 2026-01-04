# 3D_raycast_game

## frame_file

### screen struct

the width and height represent how big the screen is in pixels

the frame_buf field is a vector of of pixels each u32 representing a colour

frame_buf is also width * height big in this case 1000 * 1000 so to index into the 1D array we do y * width(which is the width of the screen aka 1000) + x





