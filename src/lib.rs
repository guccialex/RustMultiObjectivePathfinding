
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
use std::collections::BTreeMap;

use ncollide2d::shape::Polyline;
use ncollide2d::shape::Segment;
use nalgebra::geometry::Point2;



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







//takes in a reference to a list of lines
//returns a list of nodes

fn getlineintersections<'a>(segmentlist: HashMap<u32, (&Segment<f32>,&Isometry2<f32>, &'a ShapeShape)>) -> Vec<(Point2<f32>, &'a ShapeShape)>{
    
    use ncollide2d::partitioning::BVT;
    use ncollide2d::bounding_volume;
    use ncollide2d::bounding_volume::AABB;
    use ncollide2d::query::visitors::AABBSetsInterferencesCollector;
    use nalgebra::base::Matrix2;
    use ncollide2d::partitioning::BVH;
    use ncollide2d::query;
    
    
    let mut returnvec: Vec<(Point2<f32>, &ShapeShape)> = Vec::new();
    
    
    
    //the leaves
    let mut class1leaves:  Vec<(u32, AABB<f32>)> = Vec::new();
    
    
    //for every object
    for (itsid, (segment, isometry, shapeshape)) in &segmentlist{
        
        
        let thebounding = bounding_volume::aabb(*segment, isometry);
        
        class1leaves.push((*itsid, thebounding));
        
    }
    
    let class1bvt = BVT::new_balanced(class1leaves);
    
    
    
    
    
    //the vector all intersections are collected to
    let mut collectedall: Vec<(u32, u32)> = Vec::new();
    
    
    let curiso = Isometry2::<f32>::identity();
    
    let thematrix = Matrix2::identity();
    
    let mut class1inter1 = AABBSetsInterferencesCollector::new(0.0, &curiso, &thematrix , &mut collectedall);	
    
    
    //get class 1's intersections with itself
    class1bvt.visit_bvtt(&class1bvt, &mut class1inter1);
    
    //for all the intersecting bounding boxes, check if they actually intersect, and where
    for (lineid1, lineid2) in &collectedall{
        
        
        //if its not intersecting with itself
        if (lineid1 == lineid2){
        }
        else
        {
            
            if let Some((segment1, iso1, shapeshape1)) = segmentlist.get(lineid1){
                
                if let Some((segment2, iso2, shapeshape2)) = segmentlist.get(lineid2){
                    
                    
                    //let contact = query::contact(iso1, *segment1, iso2, *segment2, 10.0);
                    
                    //println!("{:?}, and {:?}", segment1, segment2);
                    
                    
                    //turn those two lines into infinitely stretching lines and get where they intersect
                    
                    
                    //println!();
                    //println!("{:?}", contact);
                    
                    //get the point/line intersections for this line and that line
                    
                    //if they are closer than the agents radius, they are connected nodes
                    
                    //if both ends are closer to the other line, then the entire line is closer than the radius
                    //and this means that the agent cant be
                    
                    
                    println!("these are the lines {:?}, {:?}", segment1, segment2);
                    
                    
                    
                    let line1point1;
                    let line1point2;
                    let line2point1;
                    let line2point2;
                    
                    unsafe {
                        line1point1 = (*segment1.a().get_unchecked(0), *segment1.a().get_unchecked(1)) ;
                        
                        line1point2 =  (*segment1.b().get_unchecked(0), *segment1.b().get_unchecked(1));
                        
                        line2point1 = (*segment2.a().get_unchecked(0), *segment1.a().get_unchecked(1));
                        
                        line2point2 = (*segment2.b().get_unchecked(0), *segment2.b().get_unchecked(1));
                    }
                    
                    //check if the two line segments intersect
                    
                    
                    //if they dont
                    //check if 
                    
                    
                    //linepointintersection(line1point1, (line2point1, line2point2));
                    //linepointintersection(line1point2, (line2point1, line2point2));
                    
                    
                    
                    //create a node on all the lines of the
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                    
                }
            }
            
        }
        
        
        
        
        
        
    }
    
    
    
    
    
    
    returnvec
    
    
}



//given two lines
/*
return if they intersect, and then where that intersection is

and if they dont intersect, the two closest points on both lines



*/


/*

returns if the 

takes in a point and a line
and returns the 

*/
fn linepointintersection(point: (f32,f32), theline: ((f32,f32),(f32,f32)) ) {//-> Option<(f32, (f32,f32))> {
    
    let x3 = point.0;
    let y3 = point.1;
    
    let x1 = (theline.0).0;
    let y1 = (theline.0).1;
    
    let x2 = (theline.1).0;
    let y2 = (theline.1).1;
    
    let toppart = ((x3 - x1)*(x2 - x1)) + ((y3 - y1)*(y2-y1));
    
    let mut botpart = ((((y2 - y1) + (x2 - x1)).powf(2.0)).sqrt()).powf(2.0);
    
    if (botpart == 0.0){
        
        botpart = 0.001;
    }
    
    let theuvalue = (toppart / botpart);
    
    
    //these are the x and y values of the closest point on the line to the point passed in
    
    let xclosest = x1 + theuvalue * (x2-x1);
    
    let yclosest = y1 + theuvalue * (y2 - y1);
    
    println!("{:?} and {:?} ", xclosest, yclosest);
    
    
    
}






#[derive(Debug)]
struct ShapeNode{

    //this nodes position
    point: Point2<f32>,

    //the id of the shape that the edge this node is on belongs to
    shapeid: u32,

    //a list of ShapenodeIds
    associatednodesothershapes: Vec<u32>,

    //a list of ShapeNode id's
    associatednodesameshape: Vec<u32>,  
    

    //the id of this shapenode
    id: u32,

    //the normal of this node, move in the negative direction to go in the shape
    //the positive direction to go out of the shape
    
    
}

impl ShapeNode{

    fn new(thepoint: Point2<f32>, theshape: u32, theid: u32) -> ShapeNode{

        ShapeNode{point: thepoint, shapeid: theshape, id: theid, associatednodesameshape: Vec::new(), associatednodesothershapes: Vec::new()}
    }
    
    
    
}








#[derive(Debug)]
struct ShapeShape{
    
    //polylines: Vec
    
    //the class values for this shape
    classvalues: HashMap<u32, f32>,
    
    //the line segments that make this shape
    //shapelines: Vec<Segment<f32>>,
    
    shape: ConvexPolygon<f32>,
    
    isometry: Isometry2<f32>,
    
    //the lines
    linesegments: Vec<Segment<f32>>,


    //the id of this shape
    id: u32,
    
    
    
}

impl ShapeShape{
    
    //new method takes a convex shape and returns a shapeshape
    fn new(theshape: ConvexPolygon<f32>, theiso: Isometry2<f32>, newclassvalues: HashMap<u32, f32>)-> ShapeShape{
        
        let mut thelinesegments: Vec<Segment<f32>> = Vec::new();
        
        let listofpoints = theshape.points();
        
        //println!("{:?}", listofpoints);
        
        for iternumb in 0..listofpoints.len(){
            
            let point1: Point2<f32>;
            
            if (iternumb == 0)
            {
                point1 = listofpoints[listofpoints.len()-1];
            }
            else
            {
                point1 = listofpoints[iternumb -1];
            }
            
            let point2 = listofpoints[iternumb];
            
            let thislinesegment: Segment<f32> = Segment::new(point1, point2);
            
            thelinesegments.push(thislinesegment);

            //
            
        }
        //println!("{:?}", thelinesegments);
        
        ShapeShape{classvalues: newclassvalues, shape:theshape, isometry: theiso, linesegments: thelinesegments, id:0}
    }
    
    
    
}

//takes in a list of nodes and sets the associations for each of them
fn setassociations(thenodes: &mut Vec<ShapeNode>, agentdiameter: f32){

    /*
    //puts the nodes into a bvt




    //the ones close to each other, if theyre from different lines and different shapes, become associated
    //so that a line is drawn between the two, for when checking what nodes can be walked between happens

    //THE NODE IS ON THE VERTEX AND LINE OF THE SHAPE, NOT ABOVE IT RN, the agents width hasnt been considered for that yet, it has a normal for that


    //a node should have two lists
    //associatednodesothershapes: a list of nodes that are 

    //associatednodesameshape: a list of nodes that are from a different line from the same shape



    use ncollide2d::partitioning::BVT;
    use ncollide2d::bounding_volume;
    use ncollide2d::bounding_volume::AABB;
    use ncollide2d::query::visitors::AABBSetsInterferencesCollector;
    use nalgebra::base::Matrix2;
    use ncollide2d::partitioning::BVH;
    use ncollide2d::query;
    use ncollide2d::shape::Ball;
    use ncollide2d::bounding_volume::BoundingSphere;
    use ncollide2d::query::visitors::BoundingVolumeInterferencesCollector;
    
    //the list of balls and isometries
    
    //the leaves
    let mut class1leaves:  Vec<(u32, BoundingSphere<f32>)> = Vec::new();
    
    
    //for every object
    for curnode in thenodes{
        
        
        let thebounding = BoundingSphere::new(curnode.point, agentdiameter);
        
        class1leaves.push((curnode.id, thebounding));
        
    }
    
    let class1bvt = BVT::new_balanced(class1leaves);
    
    
    //the vector all intersections are collected to
    let mut collectedall: Vec<(u32, u32)> = Vec::new();
    
    

    let curiso = Isometry2::<f32>::identity();
    
    let thematrix = Matrix2::identity();
    
    let mut class1inter1 = BoundingVolumeInterferencesCollector::new( &mut collectedall);	
    
    
    //get class 1's intersections with itself
    class1bvt.visit_bvtt(&class1bvt, &mut class1inter1);


    
    //for all the intersecting bounding boxes, check if they actually intersect, and where
    for (point1, point2) in &collectedall{
        
        
        
        
        
        
        
        
    }
    */
    
    



}




pub struct ShapePathfinding{
    
    shapes: Vec<ShapeShape>,

    //goes up whenever a shape is added, is not the correct number of shapes in this struct
    //if any shapes are removed
    shapenumber: u32,
    
    
    
}

impl ShapePathfinding{
    
    pub fn new() -> ShapePathfinding{
        
        ShapePathfinding{shapes: Vec::new(), shapenumber: 0}
        
        
    }
    
    pub fn addobject(&mut self, iso: Isometry2<f32>, shape: ConvexPolygon<f32>, classvalues: HashMap<u32, f32>){
        
        let newshape = ShapeShape::new(shape, iso, classvalues);
        
        self.shapes.push(newshape);
        
        
    }
    
    
    pub fn getpath(&self, agentdiameter: f32) { //-> Vec<f32>{


        //used to give the nodes an id
        let mut totalnumbernodes: u32 = 0;

        let mut listofnodes: Vec<ShapeNode> = Vec::new();

        let nodesperunit = 2.0;

        
        //get every line
        
        //for every shapeshape object

        for curshape in &self.shapes{
            for curpoint in curshape.shape.points(){

                //create a node at each shapes verteces

                //TODO
                //DO I NEED TO APPLY THE ISOMETRY TO THIS POINT FIRST
                listofnodes.push(ShapeNode::new(*curpoint, curshape.id, totalnumbernodes));

                totalnumbernodes += 1;
                //println!("{:?}", curpoint);
                
                
            }

        }


        //TODO, implement this later
        /*  
        //now pass in the list of nodes
        //and the list of lines plus shape ids

        //add lines and nodes to a bvt
        //expand point/line bounding box by the size of the agent
        //check the nodes in a lines bounding box
        //get the distance between every node and every line
        //if they're closer than the agents width, create a node on the point on the line its closest to
        //and if it extends over the line, dont create anything, because the points vertex will connect to each other in the next step

        //they will pass back a list of nodes

        println!("{:?}", listofnodes);
        */



        //create a node on lines that have x units of space without a node
        for curshape in &self.shapes{

            for cursegment in &curshape.linesegments{

                //get the length of the line
                //get how many times the nodesperunit divides this

                let numberofnodes = cursegment.length() / nodesperunit ;

                //round up
                //subract by two for the two ends of the line that already have nodes
                let intofnodes = (numberofnodes.ceil() as u32) - 2;


                //for the number of nodes that should be made
                for curiter in 0..intofnodes{

                    //get the start position of the segment
                    let startpoint = cursegment.a();

                    //the vector representing the length and direction of the segment 
                    let segmentvector = cursegment.scaled_direction();

                    //the position of the new node
                    let newpos = startpoint + segmentvector * (curiter as f32 + 1.0) / (intofnodes as f32 + 1.0);

                    //the shapeid this node belongs to
                    let shapeid = curshape.id;


                    let newnode = ShapeNode::new(newpos, shapeid, totalnumbernodes);
                    totalnumbernodes += 1;

                    listofnodes.push(newnode);



                }

            }
        }





        //takes in a reference to a mutable list of nodes
        //makes each node associated with every one thats close by
        setassociations(&mut listofnodes, 1.0);


        
        
        //two nodes can go to eachother without touching the shape IF they're from the same line of the same shape
        //or if they're on the same edge

        //connects with every node that it can see that isnt blocked by a line

        //the cost is equal to the shapes that they are both in
        //THEY SHOULD BE IN AN UNCHANGING GROUP OF SHAPES  BETWEEN THE NODES IF THEY"RE NOT PASSING THROUGH LINES

        //setconnections(listofnodes);






        //create a node at every vertices
        
        //then create a node at every line intersection

        //then create a node wherever a line wherever a  line comes close to the vertex of another shape


        //then if one node is closer than the agent width to another node, make those nodes "connected"
        //and so, if an object wants to pass along the outside of that node, it has to intersect with the other shape


        
        
        /*
        let mut listoflines: HashMap<u32, (&Segment<f32>, &Isometry2<f32>, &ShapeShape)> = HashMap::new();
        
        let mut linesegmentcounter = 0;
        
        //for every shapeshape object
        for curshape in &self.shapes{
            for curline in &curshape.linesegments{
                
                listoflines.insert(linesegmentcounter, (curline, &curshape.isometry, curshape));
                
                linesegmentcounter += 1;
                
            }
        }
        
        let myintersections = getlineintersections(listoflines);
        
        */
        
        
        
        
        //get the intersection between every line
        //let mylineintersections: Vec<((f32,f32), HashMap<u32,f32>)> = getlineintersections();
        
        
        //let mut mylistofnodes = Vec::new();
        
        
        //create a node at every intersection
        
        //for curitem in mylineintersections{
            //    mylistofnodes.push(ShapeNode::new());
            //}
            
            
            //for every node
            //for curnode in listofnodes{
                
                //get every node on its side (every other node that is in the same shape)
                //and connect it with that node if they have line of sight
                
                
                //then connect every node with every other node that is close to it and from another shape's line
                
                //}
                
                
                
                
                
            }
            
        }
        
        
        
        /*
        
        
        
        //i actually cant think of a better name, but this is
        //basically a shape in the shapepathfinding
        struct ShapeShape{
            
            //its class values
            classvalues: HashMap<u32, f32>,
            
            //the lines that make up this shape
            //first value is start of the line
            //the second is the end
            //this is going to be important for what nodes connect to what at a t junction
            shapelines:  Vec<(f32,f32), (f32,f32)>,
            
        }
        
        impl ShapeShape{
            
            
            
        }
        
        
        
        
        
        //a node for the shapefinding struct
        //is on a line
        struct ShapeNode{
            
            //it has a list of connected nodes
            
            
            //the shapes this object is inside of
            shapesinside: Vec<&ShapeShape>,
            
            
            //and the shapes that this node is on the line of
            shapesline: 
            
            
            connectednode:
            
            
            
        }
        
        impl ShapeNode{
            
            
        }
        
        
        pub struct ShapePathfinding{
            
            //every node is connected to every other node on the same line
            //or every other node that is in 
            
            //a nodes properties
            //it is a part of all shapes it is inside
            //and it can either be part of the shape it is on the side of or not a part of it
            //an agent can move along the inside wall of it to another node on the same line, or the outside wall
            //and the cost of movement is the cost depending on whether its the inside or outside wall
            
            //cost of operation is close to O(n) but not above, where n is the total number of nodes
            
            
            //a list of the shapes and their isometry in this pathfinding object
            shapelist: Vec<ShapeShape>,
            
            //a list of the lines in this object
            linelist: Vec<ShapeLine>,
            
            //a list of nodes in this object, this should be created at the time when
            //the path needs to be found
            
            
            
        }
        
        
        impl ShapePathfinding{
            
            
            pub fn new() -> ShapePathfinding{
                
                ShapePathfinding
                
            }
            
            //given the position of some shape, and its classevaluation return the path if there is one, (and probably what object its to)
            pub fn findpath(&self, startpos: (f32, f32), agentdiameter: f32, classevaluation: HashMap<u32, f32>) -> Option<Vec<(f32, f32)>>{
                
                
                
                
                //for every shape in this object
                
                //get its lines
                
                //for each line
                
                //make a node at the end and every x units
                //and everywhere that two lines intersect
                
                //then for every node
                
                //check every other node it can see that a line is not in the way of
                
                
                //so a node has
                //the line(s) it is on
                //the 
                
                
                
                
                
                /*
                
                nodes:
                
                
                
                
                
                */
                
                
                /*
                
                when a shape is added to this object
                
                get all of its lines
                
                get all intersections of its line with other objects
                
                create a node at each corner, and at each intersection with another object
                
                
                check all nodes that this new shapes lines intersect  with
                
                
                
                
                if two nodes are at the same position as another node (if they are a reasonably close distance, because size of the thing)
                
                then combine those two nodes
                
                
                //if two nodes are closer than the agent's diameter, they're can be combined to be the same node
                
                
                //what does it mean to be a node shared by multiple lines and multiple shapes
                
                
                ok, being on the same line does not mean necessarily at all other nodes on that line can reach it
                
                god damn, combined nodes complicate things an annoying amount
                
                
                
                so for every line this node is attached to, they have that many ways they can be approached from
                
                a node on a line can be approached from its left side or right side
                
                
                if there are four quadrants which all intersect at a single node
                if you are in the bottom left
                you MUST go through either the top left or bottom right quadrant to end up at the top right quadrant
                
                
                so how about, if you are at a node, you can move to any node on the same side that you've approaced it from
                and you can pass through to a different side of the node if you can
                and the cost for that is (the degree of side you must pass through/ 360 * the agents diameter)
                
                
                
                if the other node is on the same line as this node, you can pass through either the inside, through the shape
                the line is on, or the ouside,
                
                if the other node is on a line from the same shape as this one, but not on the same line, you must pass through the shape
                
                
                
                OK
                because changing your "side" on a node incurs a cost, the whole highest cost calculation doesnt work anymore
                so, there should be nodes with the same position called "overlapping" nodes
                where the nodes each have a different "side" to them
                and they are all connected to each other, and can go from one to the other, but of course, at the appropriate cost
                of passing through sides to get to them
                and they each have their own "highest cost" value thing for them
                
                
                
                
                
                get the lines of all the shapes
                
                for each line, create a node at its intersection with every other line
                and anywhere it comes closer to a line than the width of the agent create a node on both lines (each node should have different poses but be connected)
                
                then all those nodes created by the proximity or intersections of lines
                make sure there is one for each line that passed through it
                and give those nodes references to each other, and consider them "overlapping nodes"
                
                then for every sections longer than x make sure there is a node on each line
                and nodes on a line should also have 2 nodes, one for each side
                
                and the cost to go from one side of the node to the other is
                (shape cost of this node + shapecost of that node + shapecost of all the nodes inbetween)*size of this agent / number of nodes passed through
                + if they arent exactly overlapping and have some amount of distance apart, then add the cost of the difference in distance
                (you can either get to it by going counterclockwise around, or clockwise)
                
                and each node can only see the other nodes on its same side and have those connected
                
                so each node is connected to all other nodes it can see that are on the same side as this one
                +
                the "overlapping nodes"
                
                and the cost to go from one node to another is, if these nodes are on the  same side, the cost of the side they're on * distance
                
                
                (and when returning the path to take, it should )
                
                
                
                
                First:
                get the intersections between every line
                create a node on each line where it intersects with another line
                or where it closest comes near another line if they're close enough to be correlated
                
                make all the nodes correlated that should be correlated
                
                then on every line, if there is a space larger than X where there is no node, create a node evenly spaced to fill it
                
                (creating nodes, they should have a reference to the shape that they are a part of)
                
                
                
                
                
                
                
                */
                
                
                
                
                
                
                //create the list of nodes and the nodes they're attached to
                
                
                //for each line
                
                //add a node on it at the beginning, the end, and every x units
                
                //then each node is connected to every other node on the line, and connect it to every other node
                //it has direct line of sight with
                
                //(what do i do at a t junction, when some nodes arent connected to other nodes on the same line
                    //on a certain side of it, because theres something passing through the middle)
                    
                    
                }
                
                
            }
            
            */
            
            
            
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
            
            
            
            struct f32wrapped{
                
                
            }
            
            
            
            fn getpath(nodemap: &mut HashMap<(u32,u32),PathNode> , startnode: (u32, u32), goalnode: (u32, u32) ) -> Option<PathNode>{
                
                
                //TODO: stop running that path if the values get negative
                
                
                //a hashmap of 
                //another map that sorts the keys by the pathvalue of the object
                //let mut sortedkeytree: PartialOrd<BTreeMap<f32, (u32,u32)>>;
                
                
                
                
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
                                    
                                    //and add it the tree of nodes by pathvalue
                                    //sortedkeytree.insert(newpathvalue, connectednode);
                                    
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
            
            