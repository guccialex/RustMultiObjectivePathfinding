
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}





use std::collections::HashMap;
use image::GenericImageView;
use std::collections::BTreeMap;

use ncollide2d::shape::Polyline;
use ncollide2d::shape::Segment;
use nalgebra::geometry::Point2;
use nalgebra::Isometry2;
use ncollide2d::shape::ConvexPolygon;





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
    
    //the id of the line this is on
    lineid: u32,
    
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
    
    fn new(thepoint: Point2<f32>, theshape: u32, theline: u32, theid: u32) -> ShapeNode{
        
        ShapeNode{point: thepoint, shapeid: theshape, lineid: theline, id: theid, associatednodesameshape: Vec::new(), associatednodesothershapes: Vec::new()}
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
    
    //the linesegment id and the line segment of this node
    linesegments: HashMap<u32, Segment<f32>>,
    
    
    //the id of this shape
    id: u32,
    
    
    
}

impl ShapeShape{
    
    //new method takes a convex shape and returns a shapeshape
    fn new(theshape: ConvexPolygon<f32>, theiso: Isometry2<f32>, newclassvalues: HashMap<u32, f32>, shapenumber: &mut u32, linenumber: &mut u32)-> ShapeShape{
        
        //the list of line segments for this object
        let mut thelinesegments: HashMap<u32, Segment<f32>> = HashMap::new();
        
        //the list of points of this shape
        let listofpoints = theshape.points();
        
        
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
            
            thelinesegments.insert(*linenumber, thislinesegment);
            
            *linenumber += 1;
        }
        
        
        *shapenumber = *shapenumber + 1;
        
        ShapeShape{classvalues: newclassvalues, shape:theshape, isometry: theiso, linesegments: thelinesegments, id: *shapenumber - 1}
    }
    
    
    
}

//takes in a list of nodes and sets the associations for each of them
fn setassociations(thenodes: &mut HashMap<u32, ShapeNode>, agentdiameter: f32){
    
    
    

    
    
    
    //make all the nodes a sphere at their point half the size of agent, add to list
    let mut class1leaves:  Vec<(u32, BoundingSphere<f32>)> = Vec::new();
    
    
    for (curnodeid, curnode) in thenodes.iter(){
        
        let thebounding = BoundingSphere::new(curnode.point, agentdiameter/2.0);
        
        class1leaves.push((curnode.id, thebounding));
        
    }
    
    let class1bvt = BVT::new_balanced(class1leaves);
    
    

    //what to set for the nodes
    //because i cannot iterate through both and mutate them at the same time
    //this list stores the node, and then what i want the associatednodesameshape to be
    //and then what the associatednodeothershape to be

    let mut storedsetmap: HashMap<u32, (Vec<u32>, Vec<u32>)> = HashMap::new();
    
    
    
    
    //getting all the intersections
    for (curnodeid, curnode) in thenodes.iter(){
        
        
        //where the intersections are collected to
        let mut collectedall: Vec<u32> = Vec::new();
        
        
        let thebounding = BoundingSphere::new(curnode.point, agentdiameter/2.0);
        
        //add it to the collector
        let mut class1inter1 = BoundingVolumeInterferencesCollector::new( &thebounding, &mut collectedall);
        
        
        //collect it
        class1bvt.visit(&mut class1inter1);


        let mut associatedsameshape = Vec::new();

        let mut associatedothershape = Vec::new();



        
        //the list of node ids this node intersects with
        for othernodeid in &collectedall{            
            
            if let Some(othernode) = thenodes.get(othernodeid){
                

                //if they are the same node
                //do nothing
                if (othernodeid == curnodeid){
                    
                    //println!("{}", curnodeid);
                    
                }
                //if they are from the same shape but different lines

                else if (othernode.shapeid == curnode.shapeid && othernode.lineid != curnode.lineid){

                    associatedsameshape.push(*othernodeid);                    
                    
                }
                //if they are from different shapes (and obvs diff lines)
                else if (othernode.shapeid != curnode.shapeid){

                    associatedothershape.push(*othernodeid);

                    
                }
                
                
                
            }
            else{
                panic!("why is there no node with this id?");
            }
            
        }




        storedsetmap.insert(*curnodeid, (associatedsameshape, associatedothershape));
        
        
        
    }



    //now set the associated nodes from the storedsetmap

    for (nodeid, (sameshapenodevec, othershapenodevec)) in storedsetmap{


        if let Some(thenode) = thenodes.get_mut(&nodeid){

            thenode.associatednodesothershapes = othershapenodevec;

            thenode.associatednodesameshape = sameshapenodevec;




        }
        else{
            panic!("why is a node with this id not in here");
        }



    }


    
    
    
    
    
    
    
}


//given a map of nodes, and all the lines
//set the nodes to be connected
fn setconnections(thenodes: &mut HashMap<u32, ShapeNode>, agentdiameter: f32){


    //given a list of nodes

    //and a list of lines
    
    //find the line of sight between all the nodes



    //put every line of the shape lines in a bst


    //then for every node and every other node, cast a line between this node and that node



    //every node should cast out rays,
    //and basically start at the base of the rays
    //and basically, with the objects width, calculate if from this distance
    //the lines to the left and right of it should also be pursued and have their intersections checked further

    //(like cast lines out at 1 degree apart, then if line 4 intersects something 1 unit away, check if line 5 and line 3
    //would also have been blocked by that by checking the distance the line intersects from the origin and the width of the agent
    //and how far apart the lines are at that distance (if theyre less than half the agent width, they would intersect))


    //or cast out 4 lines in the cardinal directions
    //if no line intersects with them before the distance between them becomes larger than the agent width
    //then cast out more lines and repeat

    //OR
    //cast out 1 line
    //check if any lines intersect with it
    //at the distance it has its first intersection (if no intersection, distance is the distance of the farthest node from this node)
    //cast out another line to an amount of degrees to the left, so that at that distance a line was created will be one agent width
    //away at the point it intersects with

    //and after the thing comes fully around
    //turn all those individual cones which apply until varying distances into a shape
    //(it will have to be a concave shape)
    //and check all nodes that are fully inside it, they have line of sight with this one


	
	
	
	
	
	//the leaves by class
	let mut class1leaves:  Vec<(u32, AABB<f32>)> = Vec::new();
	let mut class2leaves:  Vec<(u32, AABB<f32>)> = Vec::new();
	
	
    
    //
	let class1bvt = BVT::new_balanced(class1leaves);
	let class2bvt = BVT::new_balanced(class2leaves);
	
	
	use ncollide2d::query::visitors::AABBSetsInterferencesCollector;
	use nalgebra::base::Matrix2;
	use ncollide2d::partitioning::BVH;
	
	
	
	//the vector all non-ground intersections are collected to
	let mut collectedall: Vec<(u32,u32)> = Vec::new();
	
	
	let curiso = Isometry2::<f32>::identity();
	
	let thematrix = Matrix2::identity();
	
	let mut class1inter1 = AABBSetsInterferencesCollector::new(0.0, &curiso, &thematrix , &mut collectedall);	
	//get class 1's intersections with itself
	class1bvt.visit_bvtt(&class1bvt, &mut class1inter1);
	
	
	let mut class1inter3 = AABBSetsInterferencesCollector::new(0.0, &curiso, &thematrix , &mut collectedall);
	//get class 1's intersections with class3
	class1bvt.visit_bvtt(&class3bvt, &mut class1inter3);
	
	
	let mut class1inter4 = AABBSetsInterferencesCollector::new(0.0, &curiso, &thematrix , &mut collectedall);
	//class 2's intersections with class3
	class1bvt.visit_bvtt(&class4bvt, &mut class1inter4);
	
	
	let mut class2inter3 = AABBSetsInterferencesCollector::new(0.0, &curiso, &thematrix , &mut collectedall);
	//class 2's intersections with class3
	class2bvt.visit_bvtt(&class3bvt, &mut class2inter3);


}




pub struct ShapePathfinding{
    
    shapes: Vec<ShapeShape>,
    
    //goes up whenever a shape is added, is not the correct number of shapes in this struct
    //if any shapes are removed
    shapenumber: u32,
    
    //goes up whenever a line is added, is not the correct number of lines in this struct
    //if any shapes are removed
    linenumber: u32,
    
    
    
}

impl ShapePathfinding{
    
    pub fn new() -> ShapePathfinding{
        
        ShapePathfinding{shapes: Vec::new(), shapenumber: 0, linenumber: 0}
        
        
    }
    
    pub fn addobject(&mut self, iso: Isometry2<f32>, shape: ConvexPolygon<f32>, classvalues: HashMap<u32, f32>){
        
        let linenumber = &mut self.linenumber;
        let shapenumber = &mut self.shapenumber;
        
        let newshape = ShapeShape::new(shape, iso, classvalues, shapenumber, linenumber);
        
        self.shapes.push(newshape);
        
        
    }
    
    
    pub fn getpath(&self, agentdiameter: f32) { //-> Vec<f32>{
        
        
        //used to give the nodes an id
        let mut totalnumbernodes: u32 = 0;
        let mut listofnodes: HashMap<u32, ShapeNode> = HashMap::new();
        
        let nodesperunit = 2.0;
        
        
        //for every shape object
        for curshape in &self.shapes{
            
            //for every line of that shape
            for (segmentid, cursegment) in curshape.linesegments.iter(){
                
                
                //TODO
                //DO I NEED TO APPLY THE ISOMETRY TO THIS POINT FIRST
                
                //create a node at the start and end of the segment
                listofnodes.insert(totalnumbernodes, ShapeNode::new(*cursegment.a(), curshape.id, *segmentid, totalnumbernodes));
                totalnumbernodes += 1;
                
                
                listofnodes.insert(totalnumbernodes, ShapeNode::new(*cursegment.b(), curshape.id, *segmentid, totalnumbernodes));
                totalnumbernodes += 1;
                
            }
            
        }
        
        
        
        
        //create a node on lines that have x units of space without a node
        for curshape in &self.shapes{
            
            for (segmentid, cursegment) in &curshape.linesegments{
                
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
                    
                    
                    let newnode = ShapeNode::new(newpos, shapeid, *segmentid,  totalnumbernodes);
                    
                    
                    listofnodes.insert(totalnumbernodes, newnode);
                    
                    totalnumbernodes += 1;
                    
                    
                    
                }
                
            }
        }
        
        
        
        
        
        //takes in a reference to a mutable list of nodes
        //makes each node associated with every one thats close by
        setassociations(&mut listofnodes, agentdiameter);


        //now that associations are set
        //set connections
        setconnections(&mut listofnodes, agentdiameter);
        
        
        
        
        
    }
    
}





