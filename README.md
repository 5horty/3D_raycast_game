# 3D_raycast_game


## Utils_file

#### rgb_from_u8

takes in 3 u8s converts them to u32 and destructes them
then does bitwise minipilization (idk how to spell and i dont got autocorrect in nvim for english and i dont like too many plugins and i mean how often do i write english in here anyway)
rust stores the conversion as hexidecimal representation 
it shifts the r  to the 3rd byte location to starts at the 17th bit thru to the 24th bit aka readign form right to left (00000000 11111111 00000000 00000000)
then the same for g but jsut 9th thru 16th bits then b is 1st thru 8th the "|" joins the number together giving us a u32 represented in hex for full white its 0xFFFFFF
the rest of the bits are set to 0s 


## frame_file

### screen struct

the width and height represent how big the screen is in pixels

the frame_buf field is a vector of of pixels each u32 representing a colour

frame_buf is also width * height big in this case 1000 * 1000 so to index into the 1D array we do y * width(which is the width of the screen aka 1000) + x the y * width 
gives the row then x gives the column

#### Screen::Defualt

this method just is a Defualt init method that provides Defualt valued like 1000 width
it also sets the frame_buf to black as 0 

#### Screen::clear

this loops over everthing from 0 to self.height(1000)
it then binds a variable colour to the result of an if else statment
if y is less than self.height / 2 (500) then binds colour to blue using rgb_from_u8
else colour is bound to a green colour using that same function

then after binding colour it then loops over the width as this is a 2D space and uses that formula to get the index of each pixel and assigns it a colour bases on the if statment

#### Screen::draw_pixel

takes in x and y cords and a colour
checks if the x and y cords are within the screen if not prints "out of bounds" and returns
calculates index using the earlier formula then sets index to the colour taken as parameter

#### Screen::draw_column

takes in a x cord and the y axis start position and its end position and the colour
then loops from y start to end calling draw_pixel which then creates a column very useful for raycasting 

## world_file

### World struct

the World struct has a width, height and tiles fields the width and height differ from the screen as they are the measurements of the map aka tiles named pretty badly but whatever
tiles is just a vector of u8s as they are only 1 or 0s so no need for anything else
1s are for walls and 0s are for nothing 
also the layout of tiles is bcs of my configs in nvim and how it formats rust code its good most of the time but not perfect
as its 10 by 10 but represented weirdly 

#### World::Defualt

just a Defualt init method like i said 10 by 10 and then the tiles map is jsut a cage basically a opening in the middle and walls all around

#### World::get_tiles

its returns the tile bases on and index of coordinates
agian as i used a 1D array i have to convert it so the same forumale is used y * width + x is used to index the array and return the tile at that location

#### World::is_wall

this is a collision dectection method all it does is call the get_tile method and if that returns 1 then the is_wall method returns true else false

## player_file

### Player struct

x_cord, y_cords, angle and fov fields 
x and y cords used for positioning
players facing direction in radians
fov the field of viewin radians
fov the field of view

#### Player::Defualt








