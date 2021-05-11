//Temporary warning disables for development
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]


//use std::ptr;
use trees::{tr, TreeWalk};

//Struct representing a node
struct Node{
	test_id: i16, //Temporary ID for testing known graphs
	is_goal: bool,
	/* FFXIV specific parts, begin with testing IDDFS
	remaining_durability: i16,
	remaining_cp: i16,
	
	quality_expected: i16,
	quality_max: i16,
	quality_min: i16,

	progress_expected: i16,
	progress_max: i16,
	progress_min: i16,*/

	//child_nodes: Option<Vec<Node>>,
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		return self.test_id == other.test_id;
	}
}

const MAX_DEPTH: i16 = 300;

/*
const NULL_NODE: Node = Node{
	test_id: -1,
	is_goal: false,
	child_nodes: vec![],
};*/

fn make_node(traits: &[i16]) -> Node {

	//Calculate if goal or not
	let progress = traits[8];
	let mut item_complete = false;
	if progress >= 100 {
		item_complete = true;
	}

	let new_node = Node {
		test_id: traits[0],
		is_goal: item_complete,
	};

	return new_node;

}


fn main() {
    println!("\n-------------");
    println!("Run Start");
    println!("-------------\n");

    //Testing tree setup and printing
    let forest = tr(0) /-(tr(1) /tr(2)/tr(3) ) /-( tr(4) /tr(5)/tr(6));
    println!("{}", forest.to_string());


    //Set up a node
    let mut nodes = tr(Node {
    	test_id: 0,
    	is_goal: false,
    	//child_nodes: None,
    });

    nodes.push_back(tr(Node {
    	test_id: 1,
    	is_goal: false,
    	//child_nodes: None,
    }));

    nodes.push_back(tr(Node {
    	test_id: 2,
    	is_goal: false,
    	//child_nodes: None,
    }));

    //println!("{}", nodes.data().test_id);
    /*let mut iter = nodes.iter_mut();
    
    iter.next().unwrap().push_back(tr(Node {
    	test_id: 3,
    	is_goal: false,
    	//child_nodes: None,
    })); */

    //println!("{}", nodes.to_string());

    //let mut walk = TreeWalk::from( nodes );

    let mut iter = nodes.iter_mut().peekable();

    loop {
    	let current = iter.next();
    	if current == None {
    		//Stop when there are no more child nodes
    		break;
    	}
    	//Might be just a worse version of a for loop? TODO check

    	let mut current_data = current.unwrap();

    	let current_id = current_data.data().test_id;
    	println!("\n{}", current_id);

    	current_data.push_back(tr(Node {
    		test_id: current_id + 1,
    		is_goal: false
    	}));

    	let mut child_iter = current_data.iter_mut().peekable();
    	println!("{}", child_iter.next().unwrap().data().test_id);
    	//Once tree is built, this is equivalent of going down in depth, move to functions
    }

//println!("{}", nodes.to_string());

    println!("\nEND\n");

    /*
    println!("{}", iter.peek().unwrap().data().test_id);
    iter.next().unwrap().push_back(tr(Node {
    	test_id: 3,
    	is_goal: false,
    }));

    //let children_iter = iter.peek().unwrap().iter_mut().peekable();
    println!("{}", iter.next().unwrap().data().test_id);
    println!("{}", iter.next().unwrap().data().test_id);
    */

    //Testing depth first tree traversal?

    //TODO: Rewrite methods w/ tree library usage
}



/*fn iterative_depth_first_search(root: &Node) -> Option<&Node> {
	//Search to depths of increasing size
	for depth in 1..MAX_DEPTH {
		//Start a search from the root to the given depth
		//Get tuple telling us if there are remaining deeper nodes and returning any found nodes.

		//Expecting tuple with 
		let results:(Option<&Node>, bool) = depth_first_search(root, depth);


		if results.0 != None {
			return results.0;
			/*
			TODO: Update this so instead of returning first success node and stopping, instead
				display the found crafting path and continue searching for better ones
				Also collect path taken to get here
			*/

		} else if results.1 == false {
			//Searched through entire tree, failed to find successful path
			return None;
		}

	}

	return None; //Failed to find (?), returning null node

}

//Depth limited search for directed graphs
fn depth_first_search(current_node: &Node, depth: i16) -> (Option<&Node>, bool) {
	if depth == 0 {
		//We've hit max depth for this iteration, treat as leaf node
		if current_node.is_goal {
			return (Some(current_node), true);
		} else {
			//There are still remaining children potentially, but they
			//  will be visited in the next iteration with higher depth
			return (None, true);
		}

	} else {//if depth > 0 {
		assert!(depth > 0);
		let nodes_remain = false;

		//Depth remaining, check the child nodes, if any
		for child in /*current_node.as_ref().child_nodes.unwrap_or_else(Vec::new).iter()*/ {

			//Recurse down to the next depth
			let results = depth_first_search(child, depth-1);

			if results.0 != None {
				//We found the goal node, but there are still nodes we haven't checked
				return (results.0, true);

			} else if results.1 {
				//Haven't found the node, but there are still nodes we haven't checked
				let nodes_remain = true;
			}
		}

		//We didn't find the goal node, return whether any nodes remain as well.
		return (None, nodes_remain);

	}


}*/


/*

IDFFS recreted by referencing wikipedia
First papers on this algorithm by Richard E Korf (1985), "Depth-first iterative deepening"
https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search

function IDDFS(root) is
    for depth from 0 to ∞ do
        found, remaining ← DLS(root, depth)
        if found ≠ null then
            return found
        else if not remaining then
            return null

function DLS(node, depth) is
    if depth = 0 then
        if node is a goal then
            return (node, true)
        else
            return (null, true)    (Not found, but may have children)

    else if depth > 0 then
        any_remaining ← false
        foreach child of node do
            found, remaining ← DLS(child, depth−1)
            if found ≠ null then
                return (found, true)   
            if remaining then
                any_remaining ← true    (At least one node found at depth, let IDDFS deepen)
        return (null, any_remaining)

*/
