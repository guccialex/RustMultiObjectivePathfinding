
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



pub fn createimage(){
    
    
    let buffer: &[u8] = &[240,240,0,128,128,0,0,129,129,0,240,240]; // Generate the image data
    
    // Save the buffer as "image.png"
    image::save_buffer("testimage.png", buffer, 2, 2, image::ColorType::Rgb8).unwrap()
    
    
}


//the struct inside a unit that has the classes and their corresponding value
struct UnitClass{
    
    //these shouldnt be negative
    classvalues: HashMap<u32, f32>,
    
}


impl UnitClass{
    
    fn new() -> UnitClass{
        
        //initialize the values 0 -> 100 with 0.0
        
        let mut theclassvalues = HashMap::new();
        
        for x in 0..100{
            
            theclassvalues.insert(x, 0.0);
            
        }
        
        UnitClass{classvalues: theclassvalues}
        
    }
    
    //add values to this unit class
    fn addunitvalue(&mut self, classvalues: &HashMap<u32, f32>){
        
        //for each value passed in
        for (curkey, curvalue) in classvalues.iter(){
            
            //get the corresponding classvalue for this object
            if let Some(thisclassvalue) = self.classvalues.get_mut(curkey){
                
                //add the value passed in to this object
                *thisclassvalue += curvalue;
                
            }
            //if its not in this obejct list, panic
            else{
                panic!("why cant i find you in thsi objects hashmap");
            }
            
        }
        
        
    }
    
    //get the value given a class evulation correspondance
    fn getunitvalue(&self, classevaluation: &HashMap<u32, f32>) -> f32{
        
        //the total value to be returned
        let mut totalvalue = 0.0;
        
        //for each value passed in
        for (curkey, curevaluation) in classevaluation.iter(){
            
            //get the corresponding classvalue for this object
            if let Some(thisclassvalue) = self.classvalues.get(curkey){
                
                //multiply it by the evaluation passed in and add it to the total
                totalvalue += (*thisclassvalue) * (*curevaluation);
                
            }
            //if its not in this obejct list, panic
            else{
                panic!("why cant i find you in thsi objects hashmap");
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
    
    
    
    pub fn addobject(&mut self, isometry: Isometry2<f32>, shape: ConvexPolygon<f32>, shapeclassvalues: HashMap<u32, f32>){
        
        //check if each of the points intersects with the object
        //if it does, add its classvalues to that point
        for cury in 0..self.ysize{
            
            for curx in 0..self.xsize{
                
                
                let curpoint = Point2::new((curx as f32*1.0)/self.scale, (cury as f32*1.0)/self.scale);
                
                if ( shape.contains_point(&isometry, &curpoint)){
                    
                    
                    self.themap[cury as usize][curx as usize].addunitvalue(&shapeclassvalues);
                    
                    
                    //if it contains 1 2 or 3, use those to update the flatmap
                    
                    //with 0.0 being 0 to 10 being 255
                    
                    if let Some(redcolour) = shapeclassvalues.get(&0){
                        
                        //checked addition
                        if let Some(newvalue) = self.flatmap[(((cury*self.ysize) + curx)*3) as usize].checked_add((redcolour*80.0) as u8){
                            
                            self.flatmap[(((cury*self.ysize) + curx)*3) as usize] = newvalue;
                        }
                        else{
                            
                            self.flatmap[(((cury*self.ysize) + curx)*3) as usize] = 255;
                        }
                        
                        
                    }
                    
                    if let Some(bluecolour) = shapeclassvalues.get(&1){
                        
                        
                        //checked addition
                        if let Some(newvalue) = self.flatmap[(((cury*self.ysize) + curx)*3) as usize  + 1].checked_add((bluecolour*80.0) as u8){
                            
                            self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 1] = newvalue;
                        }
                        else{
                            
                            self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 1] = 255;
                        }
                        
                        
                        
                    }
                    
                    
                    if let Some(greencolour) = shapeclassvalues.get(&2){
                        
                        //checked addition
                        if let Some(newvalue) = self.flatmap[(((cury*self.ysize) + curx)*3) as usize  + 2].checked_add((greencolour*80.0) as u8){
                            
                            self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 2] = newvalue;
                        }
                        else{
                            
                            self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 2] = 255;
                        }
                        
                    }
                    
                    
                }
                
                
            }
            
        }
        
        
    }
    
    //add a single point to this
    pub fn addpoint(&mut self, position: (u32, u32), shapeclassvalues: HashMap<u32, f32>){
        
        let cury = position.0;
        let curx = position.1;
        
        self.themap[cury as usize][curx as usize].addunitvalue(&shapeclassvalues);
        
        
        //if it contains 1 2 or 3, use those to update the flatmap
        
        //with 0.0 being 0 to 10 being 255
        
        if let Some(redcolour) = shapeclassvalues.get(&0){
            
            //checked addition
            if let Some(newvalue) = self.flatmap[(((cury*self.ysize) + curx)*3) as usize].checked_add((redcolour*80.0) as u8){
                
                self.flatmap[(((cury*self.ysize) + curx)*3) as usize] = newvalue;
            }
            else{
                
                self.flatmap[(((cury*self.ysize) + curx)*3) as usize] = 255;
            }
            
        }
        
        
        if let Some(bluecolour) = shapeclassvalues.get(&1){
            
            //checked addition
            if let Some(newvalue) = self.flatmap[(((cury*self.ysize) + curx)*3) as usize  + 1].checked_add((bluecolour*80.0) as u8){
                
                self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 1] = newvalue;
            }
            else{
                
                self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 1] = 255;
            }
            
        }
        
        
        
        if let Some(greencolour) = shapeclassvalues.get(&2){
            
            //checked addition
            if let Some(newvalue) = self.flatmap[(((cury*self.ysize) + curx)*3) as usize  + 2].checked_add((greencolour*80.0) as u8){
                
                self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 2] = newvalue;
            }
            else{
                
                self.flatmap[(((cury*self.ysize) + curx)*3) as usize + 2] = 255;
            }
            
        }
        
        
        
        
    }
    
    
    //draw this object and make a jpg with the name passed in
    pub fn drawthis(&self, imagename: &str){
        
        //let colour
        
        //image::save_buffer("imagedir/".to_owned()+&imagename.to_owned()+&".png".to_owned(), &myvec, xdim, ydim, image::ColorType::Rgb8).unwrap()
        
        image::save_buffer("exporting/".to_owned()+&imagename.to_owned()+&".png".to_owned(), &self.flatmap, self.xsize, self.ysize, image::ColorType::Rgb8).unwrap()
        
        
    }
    
    pub fn drawpath(&self, pathlist: Vec<(u32, u32)>, imagename: &str){
        
        //get a copy of the current flatmap buffer
        
        let mut flatmapcopy =  self.flatmap.clone();

        //add on top of it, all the nodes that are in the path
        
        for currentpathnode in pathlist{
            
            let cury = currentpathnode.0;

            let curx = currentpathnode.1;
            
            
            flatmapcopy[(((cury*self.ysize) + curx)*3) as usize + 0] = 255;
            flatmapcopy[(((cury*self.ysize) + curx)*3) as usize + 1] = 255;
            flatmapcopy[(((cury*self.ysize) + curx)*3) as usize + 2] = 255;
            
            
            
        }

        image::save_buffer("exporting/".to_owned()+&imagename.to_owned()+&".png".to_owned(), &flatmapcopy, self.xsize, self.ysize, image::ColorType::Rgb8).unwrap()
        
        
        
    }
    
    //get the next move the object passed in should make
    pub fn getdirection(&self, xandypos: (u32,u32), classvalues: HashMap<u32, f32>) -> Option<Vec<(u32,u32)>>{
        
        
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
        
        let mut listofgoalnodepos = Vec::new();
        
        
        //build a nodemap, a map that summarizes how much this object values each point
        let mut thenodemap: HashMap<(u32, u32), PathNode> = HashMap::new();
        
        //for every node in the map
        for cury in 0..self.ysize{
            
            for curx in 0..self.xsize{
                
                
                let mut theunitvalue = self.themap[cury as usize][curx as usize].getunitvalue(&classvalues);
                
                //if it is positive, add this node to the list of goals
                if (theunitvalue > 0.1){
                    listofgoalnodepos.push((cury, curx));                
                }
                
                
                //create a new pathnode
                let thecurrentnode = PathNode::new(theunitvalue);
                
                //add it to the map of paths im making
                thenodemap.insert( (cury, curx) , thecurrentnode );
                
                
                
            }
            
        }
        
        
        
        println!("{:?}", self.ysize);
        
        //for each goal node
        for currentgoalpos in listofgoalnodepos{
            
            let thestartnodepos = xandypos;
            
            let thegoalnodepos = currentgoalpos;
            
            //get the node in the position of the goal node
            if let Some(goalnode) = thenodemap.get_mut(&thegoalnodepos){
                
                //update its path value
                goalnode.updatepathvalue( goalnode.getselfvalue(), (10000, 10000));

                //update the path it gets its path from to zero
                goalnode.nodethatgavepath = None;
                
            }
            
            
            //the vector to return
            
            let mut pathtoreturn: Vec<(u32,u32)> = Vec::new();
            
            //run the path finder
            if let Some(mypath) = getpath (&mut thenodemap, thestartnodepos, thegoalnodepos){
                
                
                //for that path
                //try to reconstruct the path
                
                let mut stillrunning = true;
                
                let mut currentnodepos = thestartnodepos;
                
                let mut curcount = 0;
                
                while (stillrunning){
                    
                    curcount += 1;
                    
                    if (curcount >= 1000){
                        stillrunning = false;
                    }
                    
                    //print the position of this node
                    println!("{:?}", currentnodepos);
                    
                    pathtoreturn.push(currentnodepos);
                    
                    
                    
                    //get this node
                    if let Some(curnode) = thenodemap.get(&currentnodepos){
                        
                        //get the position it got its path from
                        if let Some(prevnode) = curnode.nodethatgavepath{
                            
                            print!("{:?}", curnode);
                            
                            //update this node to that one
                            currentnodepos = prevnode;
                            
                            
                        }
                        //else if this is the progenitor node, stop running
                        else{
                            stillrunning = false;
                        }
                        
                    }
                    else{
                        
                        //this means the current node is the nonesense one that the goal points to
                        stillrunning = false;
                        //panic!("if i cant get it, it should be done..");
                    }
                    
                    
                    
                }
                
                return(Some(pathtoreturn));
                
                
                
            }
            
            
        }
        None
        
        /*
        //get the ideal path to the goals
        
        let thestartnode = (0,0);
        let thegoalnode = (20,60);
        
        //TESTING THING
        //make the selfvalue and pathnode at the startnode higher
        
        let mut nodetoadd = PathNode::new(100.0);
        nodetoadd.updatepathvalue(100.0, (100000,100000));
        //nodetoadd.adjacentnodesrun = true;
        
        thenodemap.insert(thegoalnode, nodetoadd);
        
        if let Some(mypath) = getpath (&mut thenodemap, thestartnode, thegoalnode){
            
            
            
            
            //for that path
            //try to reconstruct the path
            
            let mut stillrunning = true;
            
            let mut currentnodepos = thestartnode;
            
            let mut curcount = 0;
            
            while (stillrunning){
                
                curcount += 1;
                
                if (curcount >= 1000){
                    stillrunning = false;
                }
                
                //print the position of this node
                println!("{:?}", currentnodepos);
                
                
                
                //get this node
                if let Some(curnode) = thenodemap.get(&currentnodepos){
                    
                    //get the position it got its path from
                    if let Some(prevnode) = curnode.nodethatgavepath{
                        
                        print!("{:?}", curnode);
                        
                        //update this node to that one
                        currentnodepos = prevnode;
                        
                        
                    }
                    //else if this is the progenitor node, stop running
                    else{
                        stillrunning = false;
                    }
                    
                }
                else{
                    
                    //this means the current node is the nonesense one that the goal points to
                    stillrunning = false;
                    //panic!("if i cant get it, it should be done..");
                }
                
                
                
            }
            
        }
        
        */
        
        
        //return the path taken
        
        
        
        
        
    }
}


#[derive(Clone, Debug, Default)]
struct PathNode{
    
    //the HIGHEST sum of the selfvalues from the goal node to this node through connected nodes
    //inclusive
    pathvalue: f32,
    
    //if this was given a pathvalue, the last node that gave this one a path
    //used so that a traceback of the path taken to reach the start from the goal can be found
    nodethatgavepath: Option<(u32,u32)>,
    
    //the valued objects that this object already visited
    //means dont GAIN the value that this pathvalue has when you visit it again (this should really be "dont increase its value from the value increasign thing again")
    //("but still have it decreased by the decreasing value things about that tile, but i cant actually do that")
    
    //the value that this node imparts
    selfvalue: f32,
    
    //if this, with its highest path acheived has already run on all its adjacent nodes
    adjacentnodesrun: bool,
}

impl PathNode{
    
    fn new(theselfvalue: f32) -> PathNode{
        
        PathNode{pathvalue: 0.0, selfvalue: theselfvalue, adjacentnodesrun: false, nodethatgavepath: None}
        
    }
    
    fn getselfvalue(&self) -> f32{
        
        self.selfvalue
    } 
    
    fn getpathvalue(&self) -> f32{
        
        self.pathvalue
    }
    
    //get the new pathvalue, and the nodepos that this pathvalue came from
    fn updatepathvalue(&mut self, newpathvalue: f32, givernodepos: (u32, u32) ){
        
        self.pathvalue = newpathvalue;
        self.nodethatgavepath = Some(givernodepos);
        
    }
    
    
}





fn getpath(nodemap: &mut HashMap<(u32,u32),PathNode> , startnode: (u32, u32), goalnode: (u32, u32) ) -> Option<PathNode>{
    
    
    //TODO: stop running that path if the values get negative
    
    
    //a hashmap of 
    //another map that sorts objects by ( )
    
    
    //the value to return
    let mut nodewithpath: PathNode;
    
    
    println!("it started at leasst");
    
    for curiter in 0..400000
    {
        
        //get the node with the highest value
        let mut highestpathvalue = 0.0;
        let mut highestnode: &mut PathNode = &mut Default::default();
        let mut highestnodekey: & (u32, u32) = &(10000,10000);
        
        //print!("i am of size{:?}", nodemap.len());
        
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
        
        
        //set this nodes "adjacentnodesrun" as true
        highestnode.adjacentnodesrun = true;
        
        
        //if the highestnodekey hasnt changed, panic
        if (highestnodekey == &(10000 as u32, 10000 as u32)){
            println!("the object has not been reached in {} iterations, before having its value to run to drop to zero",curiter );
            
            return(None)
        }
        
        
        //println!("{:?}", highestnode);
        //println!("{:?}", highestnodekey);
        
        
        //println!("{:?}", highestnode);
        
        //get a copy of the highest node so i can iterate through the nodemap again and still have this value
        let highestnodecopy = highestnode.clone();
        
        let highestnodekey = &highestnodekey.clone();
        
        
        
        //if this is the start node, it means that this is the highest value path to the start node possible
        //and so, break and return the node that has the path
        if  (highestnodekey == &startnode){
            
            println!("i found a path, it took {} iterations",curiter );
            
            //return that start node with the highest path value
            return(Some((*highestnode).clone()))
            
        }
        
        
        
        //get the nodes this node is attached to
        let mut connectednodes = Vec::new();
        connectednodes.push((highestnodekey.0 , highestnodekey.1 + 1));
        connectednodes.push((highestnodekey.0 + 1 , highestnodekey.1));
        
        //check if these are gonna overflow to under zero
        if let Some(newsouthy) = highestnodekey.0.checked_sub(1){
            
            connectednodes.push((newsouthy , highestnodekey.1));
            
        }
        
        if let Some(newwestx) = highestnodekey.1.checked_sub(1){
            
            connectednodes.push((highestnodekey.0 , newwestx));
            
        }
        
        
        
        //check if these nodes are valid
        for connectednode in connectednodes{
            
            
            //if its in the node map
            if let Some(existingnode) = nodemap.get_mut(&connectednode){
                
                //get the pathvalue that this node leading to it gives
                let newpathvalue = existingnode.selfvalue + highestnodecopy.pathvalue;
                
                //if this pathvalue is higher than its current pathvalue
                //update its pathvalue
                //this WILL fuck up and set the goal to point to the second node and the
                //second node to point to the goal
                if (newpathvalue > existingnode.pathvalue){
                    
                    //so if the connected node is equal to the goal, just dont do it
                    if (connectednode == goalnode){
                        
                        
                    }
                    else{
                        //push the node that gave it this value
                        existingnode.updatepathvalue(newpathvalue, *highestnodekey);
                    }
                }
                
            }
            
        }
        
        
        
    }
    
    //this SHOULDNT run
    
    panic!("if no path was found it should be found out before me");
    
    
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

