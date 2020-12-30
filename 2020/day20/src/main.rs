use std::io;
use std::collections::HashMap;
use std::option::Option;

//Day 20 2020 for advent of code
//
//
//
type Edge = u32;
type EdgeNumber = u8;
type Tile = usize;
type ImageRow = u128;
type TileRow = u8;
type TileCoordinate = u32;
type FinalAnswer = usize;


#[derive(Clone,Debug)]
struct TileData {
    id: Tile,
    is_matched: bool,   //true if we've stitched this tile into the pic
    is_flipped: bool,   //true if we had to flip this tile to stitch it
    top_edge_no: EdgeNumber,   //did we rotate? 0 is no, 1,2,3 mean rotate those edges to top
    image: Vec<TileRow>,// the image on this tile 
    x: TileCoordinate,              //where we stitched this tile in - x coordinate
    y: TileCoordinate               //                                 y coordinate
}

#[derive(Clone,Copy,Debug)]
struct EdgeInfo {    //information about stored edge information
    tile_id: Tile,  
    edge_no: EdgeNumber,  //top is 0, counting around clockwise to left = 3
    is_reversed: bool, //is this the normal version of the edge or the reversed one?
    matching_edge: (Tile,EdgeNumber)
}

static MAX_PICTURE_SIZE:TileCoordinate = 96;
static MONSTER:[ImageRow; 3] = [0b10,0b10000110000110000111,0b01001001001001001000];

//    test_rules: Vec<Vec<RuleNum>>
//}



fn main() {
    let orig_inp = read_input();  //read input buffer strings (trimmed)
    let (part1_answer, image) = calc_part_1(&orig_inp);
    println!("The part 1 answer is {}",&part1_answer);

    let part2_answer = calc_part_2(&image);
    println!("The part 2 answer is {}",&part2_answer);
}

fn read_input() -> Vec<String> {
    let mut inbuf = String::new();
    let mut outbuf = Vec::new();
    while 0<io::stdin().read_line(&mut inbuf).expect("I/O error on read"){
        inbuf.truncate(inbuf.trim_end().len());
        outbuf.push(inbuf);
        inbuf = String::new();

    }
    outbuf
}
fn calc_part_1(inp:&Vec<String>) -> (Tile, Vec<ImageRow>) {
    let mut normal:HashMap<Edge,Tile> = HashMap::new();
    let mut reverse:HashMap<Edge,EdgeInfo> = HashMap::new();
    let mut tile_info:HashMap<Tile,TileData>= HashMap::new();
    let mut edge_info:HashMap<(Tile,EdgeNumber),EdgeInfo>=HashMap::new();
    let mut line = 0; //counter to current line in input
    while line < inp.len() {
        // read the tile num and set up Tile data struct for it
        let mut this_tile:TileData = TileData {
            id: inp[line][5..9].parse().unwrap(),
            is_matched: false,
            is_flipped: false,
            top_edge_no: 0,
            image: Vec::new(),
            x: 0,
            y: 0
        };
        //read the tile num
        //let tile:Tile = inp[line][5..9].parse().unwrap();
        line += 1;
        //create array of edges
        let mut edge_list:Vec<Edge> = vec![0;4];
        //parse the top line first
        edge_list[0]=inp[line].chars().fold(0,|acc, n| (acc<<1) | char_to_bit(n));
        //for (idx_i,i) in inp[line].chars().enumerate() {
        //    edge_list[0] = edge_list[0] | char_to_bit(i) << 9-idx_i;
        //};
        //get top row of the left/right edges
        edge_list[1] = char_to_bit(inp[line].chars().last().unwrap()); 
        edge_list[3] = char_to_bit(inp[line].chars().next().unwrap()); 
        //now read through the eight rows to get left and right edges (we're counting clockwise so 1
        //is right, 3 is left
        line += 1;
        for i in 1..9 {
            edge_list[1] = (edge_list[1] << 1) | char_to_bit(inp[line].chars().last().unwrap()); 
            edge_list[3] = edge_list[3]  | (char_to_bit(inp[line].chars().next().unwrap())<<i); 
            //building ImageRow goes here.  we start by skipping a byte, and shift the last byte
            //off at the end to get rid of the right edge
            let image_row=((inp[line].chars().skip(1).fold(0,|acc, n| (acc<<1) | char_to_bit(n)) & !(1))>>1) as TileRow;
            this_tile.image.push(image_row);
            line += 1; //skip to the next line
        };

        //now read the bottom edge into edge_list[2]
        edge_list[2]=inp[line].chars().fold(0,|acc, n| (acc<<1) | char_to_bit(n));
        edge_list[2]=bit_reverse(&edge_list[2]);
        //get bottom row of the left/right edges
        edge_list[1] = (edge_list[1] << 1) | char_to_bit(inp[line].chars().last().unwrap()); 
        edge_list[3] = edge_list[3]  | (char_to_bit(inp[line].chars().next().unwrap())<<9); 

        //now insert these edges into the normal and reverse hashmaps (only if other edge not
        //already there)
        //
        //if other edge is there, delete it
        for i in 0..4 {
            normal.insert (edge_list[i],this_tile.id);
            let r = bit_reverse(&edge_list[i]);
            // look for the reverse of this edge and remove it if it's there - we've matched
            // otherwise add this entry to reverse awaiting match with later processed edge
            //match reverse.remove_entry(&r) {
            match reverse.remove_entry(&edge_list[i]) {
                Some(n) => {
                    //reverse.remove_entry(&edge_list[i]);  //remove the other reversed entry for this pair
                    reverse.remove_entry(&r);  //remove the other reversed entry for this pair
                    let (_a,mut b) = n;
                    b.matching_edge = (this_tile.id,i as EdgeNumber);
                    edge_info.insert((b.tile_id,b.edge_no),b);
                    b.matching_edge = (b.tile_id,b.edge_no);
                    b.tile_id = this_tile.id;
                    b.edge_no = i as EdgeNumber;
                    //wrong?b.is_reversed = !b.is_reversed;
                    edge_info.insert((b.tile_id,b.edge_no),b);
                    0
                }
                None => {
                    let mut e = EdgeInfo {
                        tile_id: this_tile.id,
                        edge_no: i as EdgeNumber,
                        is_reversed: true,
                        matching_edge: (0,0)
                    };
                    reverse.insert(r,e);
                    e.is_reversed = false;
                    reverse.insert(edge_list[i],e);
                    0
                }
            };
        };
        line += 2;   //skip the blank line
        tile_info.insert(this_tile.id,this_tile); //store info on this tile
    };
    //Okay... we've processed all tiles and have a list of unmatched edged in reverse - look for
    //tile numbers that show up four times (two edges, each with their reverse)... those are the corners
    let mut pic_edges_tile_numbers:Vec<Tile> = Vec::new();
    for (_idx_i,i) in reverse.iter() {
        pic_edges_tile_numbers.push(i.tile_id);
    };
    pic_edges_tile_numbers.sort();
    let mut prv = 0;
    let mut count = 0;
    let mut corner_list:Vec<Tile> = Vec::new();
    for (idx_i,i) in pic_edges_tile_numbers.iter().enumerate() {
        if pic_edges_tile_numbers[idx_i] == prv {
            count += 1;
        }
        else {
            count = 0;
        };
        if count == 3 {
            corner_list.push(*i);
        }
        prv = *i;
    }
    let out = corner_list.iter().fold(1,|n,&v| n * v);

    //pick one of the corners and declare it as the top left... pick an arbitrary flipped factor
    //too...  we'll pick second corner found, and flipped - this will make the example case work
    let top_left_corner_tile_id=corner_list[1];
    let mut top_left_corner_top:EdgeNumber = 0;
    let mut num_matches=0;
    for i in 0..5  {  //look for the two unmatched edges - first unmatched edge will be top (because we're flipping the tile)
        if edge_info.contains_key(&(top_left_corner_tile_id,i%4)) {
            num_matches += 1;
        }
        else {
            num_matches = 0;
        };
        if num_matches == 2 {
            top_left_corner_top = (i+1)%4;
            break;
        }
    }

    let picture = match_tile (&edge_info,&mut tile_info,top_left_corner_tile_id,0,0,top_left_corner_top,true); //works only for the test data with the hard coded values
    //let picture = match_tile (&edge_info,&mut tile_info,1951,0,0,2,true); //works only for the test data with the hard coded values


    (out,picture)
}

fn calc_part_2(inp:&Vec<ImageRow>) -> FinalAnswer {
    println!("full picture is :");
    print_it(&inp);
    let mut working_image = flip_image(&inp);
    let mut out = 0;
    'outer: for _i in 0..4   {
        for _j in 0..2 {
            out = match find_them_monsters(&working_image) {
                Some(n)=>n,
                None => 0,
                };
            if out > 0 {
                break 'outer;
            };
            working_image = flip_image(&working_image);
        };
        working_image = rotate_image(&working_image);
    };
    out


}

fn bit_reverse(inp:&Edge) -> Edge {
    //reverses the 10 lowest order bits in an Edge structure
    let mut out = 0;
    for i in 0..10 {
        out = out | ((inp & 1<<i)>>i)<<(9-i);
    }
    out
}

fn char_to_bit (inp: char) -> Edge {
    match inp {
        '#' => 1,
        '.' => 0,
        _   => panic!("invalid pic element {}",inp)
    }
}

fn match_tile (inp_edges:&HashMap<(Tile,EdgeNumber),EdgeInfo>,
               mut inp_tiles:&mut HashMap<Tile,TileData>,
               inp_tile_id:Tile,
               inp_x:TileCoordinate,inp_y:TileCoordinate,inp_e:EdgeNumber, inp_flipped:bool) -> Vec<ImageRow> {
//recursive function to builid the image rows... input is the x/y coordinate and flipped status of
//the current row, 
//Function is expecting to be called by tile to the LEFT of this tile (x,y coordinates ascend
//rightward and downward).  or DOWN for the x=0 tile in a row.
//after flipping the tile and creating the Image, calls next tile to the RIGHT with ITS coordinates
//and flipped status.
//
//inputs - the vec of edge matching data (not changed)
//         the tile data (changed here when match is complete)
//         the tile id of THIS tile
//         x/y coordinate of the tile being processed here with the edge number of the match 
//         with the calling routine - (this is the left edge for all tiles except x=0 where its
//         the top edge)
//
//returns -  the vector of image bitmasks
//

let current_top:EdgeNumber;
let rotate_factor = match inp_flipped {  //value to use to move CLOCKWISE one edge - used in calcs later
                          true => 3,
                          false => 1
                          };
if inp_x == 0 { //if we're starting a new row
    current_top = inp_e;  //as x=0 we're coming in from the top
}
else {
    current_top = (rotate_factor + inp_e)%4;   //calculate the current top from the incoming left edge
};
//call to the right, if we're at the right edge, then create the image vec here 
let new_right = (current_top + rotate_factor)%4;
let mut out = match inp_edges.get(&(inp_tile_id,new_right)) {
                    Some(n) => {  //there's more to the right - go get it!
                        let tmpbool:bool;
                        //if (n.is_reversed == inp_flipped) {
                        if n.is_reversed {
                            tmpbool = inp_flipped;
                        }
                        else {
                            tmpbool = !inp_flipped;
                        };
                        match_tile (&inp_edges,&mut inp_tiles, n.matching_edge.0,
                                    inp_x+1,inp_y,n.matching_edge.1,tmpbool)
                    },
                    None => vec![0;8]   //this is the end of the row... create the vec here
                    };



//now update this tile's info (if we find it necessary), adjust tile's pixels (rotate/flip) and add
//to picture
let this_tile=inp_tiles.get(&inp_tile_id).expect("Error... insane condition - could not find tile id");
let adjusted_image=image8_process(&this_tile.image,current_top,inp_flipped);  //rotate the image if necessary
for i in 0usize..8 {
    let tmp:ImageRow = adjusted_image[i].into();
    out[i] = out[i] | (tmp<<((MAX_PICTURE_SIZE/8 - inp_x - 1)*8));
};


// if x == zero also collect row(s) below this one 
if inp_x==0 {
    let new_bottom = (current_top+2)%4;
    out.append(&mut match inp_edges.get(&(inp_tile_id,new_bottom)) {
                    Some(n) => {  //there's more below - go get it!
                        let tmpbool:bool;
                        if n.is_reversed{
                            tmpbool = inp_flipped;
                        }
                        else {
                            tmpbool = !inp_flipped;
                        };
                        match_tile (&inp_edges,&mut inp_tiles, n.matching_edge.0,
                                    inp_x,inp_y+1,n.matching_edge.1,tmpbool)
                    },
                    None => Vec::new()   //this is the last row... return an empty vec
                    });
};

out
}

fn image8_process(inp_v:&Vec<TileRow>,inp_e:EdgeNumber,inp_b:bool) -> Vec<TileRow> {
    let mut out = inp_v.clone();
    let mut tmpv= out.clone();
    if inp_e != 0 {
        for _i in 0..(4-inp_e) {
            //rotate right this number of times
            for j in 0..8 {
                out[j]=0;
                for k in 0..8 {
                    let bit = tmpv[k] & 1<<(7-j);
                    let bit_to_shift = bit>>(7-j);
                    out[j] = out[j] | (bit_to_shift<<k);
                };
            };
            for (idx_j,j) in out.iter().enumerate() {
                tmpv[idx_j] = *j;
            };
        }
    }

    if inp_b {   //reverse the field if appropriate
        for i in 0..8 {
            let tmp = out[i]; 
            out[i]=0;
            for j in 0..8 {
                out[i] = out[i] | ((tmp & 1<<j)>>j)<<(7-j);
            };
        }
    }
    out
}



fn print_it(inp:&Vec<ImageRow>) {
    for i in inp {
        for j in 0..MAX_PICTURE_SIZE {
            print!("{}",match i & 1<<(MAX_PICTURE_SIZE-j-1) {
                0=>".",
                _=>"#"
            });
        }
        println!("");
    }


}

fn _print_it_8(inp:&Vec<TileRow>) {
    for i in inp {
        for j in 0..8 {
            print!("{}",match i & 1<<(8-j-1) {
                0=>".",
                _=>"#"
            });
        }
        println!("");
    }
}


fn find_them_monsters(inp:&Vec<ImageRow>) -> Option<FinalAnswer> {
    // Take the image sent in, and look for the monsters in this image... 
    
    //create an empty image the same size as the input
    let mut tmp_image = inp.clone();
    for i in 0..tmp_image.len() {
        tmp_image[i] = 0;
    };

    //create a shiftable copy of the MONSTER
    let mut tmp_monster = MONSTER.clone();
    let mut monster_count = 0;
    for _i in 0..MAX_PICTURE_SIZE-20 {   //monster is twenty pixels wide 
        for j in 0..(tmp_image.len()-2) {
            if (inp[j] & tmp_monster[0]) == tmp_monster[0] &&
                (inp[j+1] & tmp_monster[1]) == tmp_monster[1] &&
                 (inp[j+2] & tmp_monster[2]) == tmp_monster[2] {  //we found a monster!
                monster_count += 1;
                //copy the monster's image to the other image to show it off
                tmp_image[j]=tmp_image[j] | tmp_monster[0];
                tmp_image[j+1]=tmp_image[j+1] | tmp_monster[1];
                tmp_image[j+2]=tmp_image[j+2] | tmp_monster[2];
            };
        };
        //move the monster image over to look for more
        tmp_monster[0]=tmp_monster[0]<<1;
        tmp_monster[1]=tmp_monster[1]<<1;
        tmp_monster[2]=tmp_monster[2]<<1;
    };
    if monster_count > 0 {
        println!("found {} monster(s)!",monster_count);
        println!("monsters are here:");
        print_it(&tmp_image);
        println!("properly flipped/rotated image is");
        print_it(&inp);
        return Option::Some(pixel_count(&inp) - pixel_count(&tmp_image));
    }

    Option::None
}

fn rotate_image(inp:&Vec<ImageRow>) -> Vec<ImageRow> {
    //rotate right this number of times
    let mut out:Vec<ImageRow> = vec![0;inp.len()];
    let image_size = inp.len() as TileCoordinate;
    for j in 0..image_size {
        //out[j]=0;
        for k in 0..image_size {
            let bit:ImageRow = inp[k as usize] & 1<<((MAX_PICTURE_SIZE-1)-j);
            let bit_to_shift:ImageRow = bit>>((MAX_PICTURE_SIZE-1)-j);
            out[j as usize] = out[j as usize] | (bit_to_shift<<(k+ (MAX_PICTURE_SIZE-image_size)));
        };
    };
    out
}

fn flip_image(inp:&Vec<ImageRow>) -> Vec<ImageRow> {
    let mut out:Vec<ImageRow> = vec![0;inp.len()];
    let image_size = inp.len() as TileCoordinate;
    for i in 0usize..inp.len() {
        let tmp = inp[i]; 
        for j in 0..image_size {
            out[i] = out[i] | ((tmp & 1<<(j+MAX_PICTURE_SIZE-image_size))>>(j+MAX_PICTURE_SIZE-image_size))<<((MAX_PICTURE_SIZE-1)-j);
        };
    };
    out
}

fn pixel_count(inp:&Vec<ImageRow>) -> FinalAnswer {
    let mut out = 0;
    for i in inp {
        let mut tmp = *i;
        for _j in 0..MAX_PICTURE_SIZE {
            if (tmp & 1) == 1 {
                out += 1;
            }
            tmp= tmp & !1;
            tmp= tmp>>1;
        }
    };
    out
}


