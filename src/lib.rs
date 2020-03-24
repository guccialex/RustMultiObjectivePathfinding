
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub fn test(){
    
    println!("HEYfuck");
}

pub fn testme(){

    println!("i am being tested, working");
}




use std::collections::HashMap;
use image::GenericImageView;

pub fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("cat.jpeg").unwrap();
    
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
    
    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    
    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
}


pub fn createimage(){
    
    
    let buffer: &[u8] = &[240,240,0,128,128,0,0,129,129,0,240,240]; // Generate the image data
    
    // Save the buffer as "image.png"
    image::save_buffer("testimage.png", buffer, 2, 2, image::ColorType::Rgb8).unwrap()
    
    
}


//the struct inside a unit that has the classes and their corresponding value
struct UnitClass{
    
    //these shouldnt be negative
    movementvalue: f32,
    damagevalue: f32,
    opponentvalue: f32,
    
}


impl UnitClass{
    
    fn new() -> UnitClass{
        
        UnitClass{movementvalue: 1.0, damagevalue: 0.2, opponentvalue: 1.0}
        
    }
    
    //get the value given a class evulation correspondance
    fn getunitvalue(&self, classvalues: &HashMap<u32, f32>) -> f32{
        
        
        //class values are
        /*
        0 is TESTING and unused
        1 is how much it values movement
        2 is how much it values damage
        3 is how much it values going to an opponent
        */
        
        let mut totalvalue = 0.0;
        
        for (curkey, curvalue) in classvalues.iter(){
            
            if (curkey == &1){
                
                totalvalue += self.movementvalue;
                
            }
            if (curkey == &2){
                
                totalvalue += self.damagevalue;
                
            }
            if (curkey == &3){
                
                totalvalue += self.opponentvalue;
                
            }
            
        }
        
        
        totalvalue
        
        
        
    }
    
    
}

pub fn TESTTHIS(){

    print!("IM BEING TESTEd");



}






pub struct VectorPathfinding{
    
    //its a vector (the y) of vectors (the x) of colours (a vector of 3 u8's)
    themap: Vec<Vec<UnitClass>>,
    
    flatmap: Vec<u8>,
    
    xsize: u32,
    
    ysize: u32,
    
    scale: f32,
    
    
}

impl VectorPathfinding{
    
    pub fn new( thexsize: u32, theysize: u32, thescale: f32) -> VectorPathfinding{
        
        //the vector
        let mut mapvector = Vec::new();
        
        let mut flatmapvector: Vec<u8> = Vec::new();
        
        for curcolumn in 0..theysize{
            
            //the vector of colours for this row
            let mut rowvec = Vec::new();
            
            
            for currow in 0..thexsize{
                
                //create the colour value for this point
                let mut unitvalue = UnitClass::new();
                
                
                //for the flatmap image thats gonna be exported
                flatmapvector.push(0);
                flatmapvector.push(0);
                flatmapvector.push(0);
                
                
                
                //for the row at the current column, push a colour vector into it
                rowvec.push(unitvalue);
            }
            
            
            //push that row vector into the vector
            mapvector.push(rowvec);
        }
        
        VectorPathfinding{xsize: thexsize, ysize: theysize, scale: thescale, themap: mapvector, flatmap: flatmapvector}
        
        
    }
    
    pub fn addobject(&mut self, isometry: Isometry2<f32>, shape: ConvexPolygon<f32>, colour: (u8,u8,u8)){
        
        //check if each of the points intersects with the object
        //if it does, add its colours to that point
        
        
        for cury in 0..self.ysize{
            
            for curx in 0..self.xsize{
                
                
                let curpoint = Point2::new((curx as f32*1.0)/self.scale, (cury as f32*1.0)/self.scale);
                
                if ( shape.contains_point(&isometry, &curpoint)){
                    
                    //if it contains this point
                    //colour it in
                    
                    
                    //R
                    self.flatmap[(((cury*self.ysize) + curx)*3) as usize] =  colour.0;
                    
                    //G
                    self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 1] = colour.1;
                    
                    //B
                    self.flatmap[(((cury*self.ysize) + curx)*3) as usize +2] = colour.2;
                    
                }
                
                
            }
            
        }
        
        
    }
    
    
    //draw this object and make a jpg with the name passed in
    pub fn drawthis(&self, imagename: &str){
        
        //let colour
        
        //image::save_buffer("imagedir/".to_owned()+&imagename.to_owned()+&".png".to_owned(), &myvec, xdim, ydim, image::ColorType::Rgb8).unwrap()
        
        image::save_buffer("exporting/".to_owned()+&imagename.to_owned()+&".png".to_owned(), &self.flatmap, self.xsize, self.ysize, image::ColorType::Rgb8).unwrap()
        
        
    }
    
    //get the next move the object passed in should make
    pub fn getdirection(&self, xandypos: (u32,u32), classvalues: HashMap<u32, f32>){
        
        //get the position of something and how much it values each object by class
        
        
        /*
        
        an object wants to move towards a goal if the path to the goal
        with its class weights is positive
        
        so, start off on every positive goal in the plane
        
        then, get each path to that goal
        
        and keep going, until that path becomes zero or negative
        
        (because if zero or negative, an object in that position with that cost weightings wouldnt bother moving)
        
        
        so start at each positive goal value
        
        recursively, for each directions you can move away from that object, subtract the cost of moving away
        and do that for that goal until you have a negative or zero value
        
        and then do that for all the goals
        
        //get a list of all the nodes visited
        //go away from that node, all the positions you can move to from that node
        //
        
        
        */
        
        
        
        
        //if the node has been visited (if not instantiated, means it hasnt been visited)
        let listofnodesvisited: HashMap<(u32, u32), bool> = HashMap::new();
        
        
        //get the class values
        //get every positive value in the class values, those are going to be the goals
        let mut listofgoals: HashMap<u32, f32> = HashMap::new();
        
        let mut listofantigoals: HashMap<u32, f32> = HashMap::new();
        
        for (currentgoal, currentvalue) in classvalues.iter(){
            
            if (currentvalue > &0.0)
            {
                listofgoals.insert(*currentgoal, *currentvalue);
                
                
            }
            //if its a zero or anti value
            else
            {
                listofantigoals.insert(*currentgoal, *currentvalue);
                
            }
            
        }
        
        
        //a map of the combined values given the objects inside and the goal values
        let mut thenodemap: HashMap<(u32, u32), PathNode> = HashMap::new();
        

        //for every node in the map
        for cury in 0..self.ysize{
            
            for curx in 0..self.xsize{

                let theunitvalue = self.themap[cury as usize][curx as usize].getunitvalue(&classvalues);

                let thecurrentnode = PathNode::new(theunitvalue);

                thenodemap.insert( (cury, curx) , thecurrentnode );
                
            }
            
            
        }
        
        
        
        //get the ideal path to the goals

        let thestartnode = (1,1);
        
        let thegoalnode = (10,10);

        let mypath = getpath (&mut thenodemap, thestartnode, thegoalnode);
        
        
        
        
        //doing it recursively
        //pass in
        //1: a mutable reference to nodes visited
        //2: the node i want to check
        //3: the current value of the things leading to it
        //4: then call this function on all the other ones
        
    }
}


#[derive(Clone, Debug, Default)]
struct PathNode{
    
    //the HIGHEST sum of the selfvalues from the goal node to this node through connected nodes
    //inclusive
    pathvalue: f32,
    
    //the valued objects that this object already visited
    //means dont GAIN the value that this pathvalue has when you visit it again (this should really be "dont increase its value from the value increasign thing again")
    //("but still have it decreased by the decreasing value things about that tile, but i cant actually do that")
    
    //the value that this node imparts
    selfvalue: f32,
    
    
    //if this, with its highest path acheived has already run on all its adjacent nodes
    adjacentnodesrun: bool,
}

impl PathNode{
    
    fn new(selfvalue: f32) -> PathNode{
        
        PathNode{pathvalue: 0.0, selfvalue: 0.0, adjacentnodesrun: false}
        
    }
    
    fn getselfvalue(&self) -> f32{
        
        self.selfvalue
    } 
    
    fn getpathvalue(&self) -> f32{
        
        self.pathvalue
    }
    
    fn updatepathvalue(&mut self, newpathvalue: f32){
        
        self.pathvalue = newpathvalue;
        
    }
    
    
}





fn getpath(nodemap: &mut HashMap<(u32,u32),PathNode> , startnode: (u32, u32), goalnode: (u32, u32) ) -> PathNode{
    
    
    //TODO: stop running if the values got negative
    
    
    //the list nodes in order of lowest value node to highest value node
    //add the current node and its value to the stackofnodes list
    
    //pop the highest value node
    //the highest value node SHOULD always be the highest POSSIBLE value that that node can have (it SHOULD be IF there are no ways to INCREASE value, instead of only decrease)
    //well maybe this is the way to have it work with the "highest possible value" thing, because if it increases value, it shouldnt increase it twice
    //
    //so it should then be pushed onto the "highestvalueacheived" node
    //how the fuck do i do this when values can increase?
    
    
    //values passed in
    /*
    startnode
    goalnode
    nodemap
    
    */
    
    
    let mut nodewithpath: PathNode;
    
    
    
    
    for curiter in 0..1000
    {
        
        //get the node with the highest value
        let mut highestpathvalue = 0.0;
        let mut highestnode: &mut PathNode = &mut Default::default();
        let mut highestnodekey: & (u32, u32) = &(10000,10000);
        
        for (curkey, curnode) in nodemap.iter_mut(){
            
            //if the current node hasnt already been run on its adjacent nodes with its highest value
            if (curnode.adjacentnodesrun == false){
                
                if (curnode.getpathvalue() > highestpathvalue){
                    
                    //set the highest value to the highest value seen so far
                    highestpathvalue = curnode.getpathvalue();
                    
                    //set the node to the node with the highest value seen so far
                    highestnode = curnode;
                    
                    highestnodekey = curkey;
                    
                }
                
            }
            
        }


        //if these values are uninitialized, which they should be, panic, so that the compiler works
        
        
        //set this nodes "adjacentnodesrun" as true
        highestnode.adjacentnodesrun = true;
        
        //get a copy of the highest node so i can iterate through the nodemap again and still have this value
        let highestnodecopy = highestnode.clone();
        


        //if this is the start node, it means that this is the highest value for the start node acheived
        //and so, break and return the node that has the path
        if  (highestnodekey == &startnode){
            
            //return that start node with the highest path value
            //(*highestnode).clone()

        }
        
        
        
        
        //get the nodes this node is attached to
        let mut connectednodes = Vec::new();
        connectednodes.push((highestnodekey.0 - 1 , highestnodekey.1));
        connectednodes.push((highestnodekey.0 , highestnodekey.1 + 1));
        connectednodes.push((highestnodekey.0 + 1 , highestnodekey.1));
        connectednodes.push((highestnodekey.0 , highestnodekey.1 - 1));
        
        
        //check if these nodes are valid
        for connectednode in connectednodes{
            
            
            //if its in the node map
            if let Some(existingnode) = nodemap.get_mut(&connectednode){
                
                //get the pathvalue that this node leading to it gives
                let newpathvalue = existingnode.selfvalue + highestnodecopy.pathvalue;
                
                //if this pathvalue is higher than its current pathvalue
                //update its pathvalue
                if (newpathvalue > existingnode.pathvalue){
                    existingnode.updatepathvalue(newpathvalue);
                }
                
            }
            
        }


        
    }

    let myreturn = PathNode::new(10.0);


    myreturn
    
    
    
    
}




use nalgebra::Isometry2;
use ncollide2d::shape::ConvexPolygon;
use ncollide2d::query::PointQuery;

use nalgebra::Point2;

//given an isometry and a convex polygon and colour
//and a size  of the grid
//create the image
pub fn shapeimage(isometry: Isometry2<f32>, shape: ConvexPolygon<f32>, colour: (u8,u8,u8), xdim: u32, ydim: u32, scale:f32, imagename: &str){
    
    let mut myvec = Vec::new();
    
    for y in 0..ydim{
        
        for x in 0..xdim{
            
            
            //create a new point
            //the scale is
            //how many times larger the plane is from the pixels
            //if one unit is one pixel
            //then with a scale of 2.0
            //one unit is 2*2 pixels
            let curpoint = Point2::new((x as f32*1.0)/scale, (y as f32*1.0)/scale);
            
            if ( shape.contains_point(&isometry, &curpoint)){
                
                /*
                print!("{} ",x);   
                print!("{}",y);
                println!();
                */
                
                
                myvec.push(colour.0);
                myvec.push(colour.1);
                myvec.push(colour.2);
            }
            else{
                myvec.push(0);
                myvec.push(0);
                myvec.push(0);
                
            }
            
        }
        
    }
    
    image::save_buffer("imagedir/".to_owned()+&imagename.to_owned()+&".png".to_owned(), &myvec, xdim, ydim, image::ColorType::Rgb8).unwrap()
    
}


use pathfinding::prelude::{absdiff, astar};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }
    
    fn successors(&self) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
        Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
        .into_iter().map(|p| (p, 1)).collect()
    }
}


pub fn pathfindingtest(){
    
    
    
    static GOAL: Pos = Pos(4, 6);
    let result = astar(&Pos(1, 1), |g| g.successors(), |g| g.distance(&GOAL) / 3,|g| *g == GOAL);
    
    print!("{:?}",result);
    
    
    
}

