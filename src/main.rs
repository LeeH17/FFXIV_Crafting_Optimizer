//Temporary warning disables for development
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]


//use std::ptr;
use trees::{tr, Node};

//Struct representing a node
#[derive(Copy, Clone)]
struct CraftingNode{
	test_id: i16, //Temporary ID for testing known graphs
	is_goal: bool,
	progress: i16, //Simplified values
	quality: i16,
	durability: i16,
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

impl PartialEq for CraftingNode {
	fn eq(&self, other: &Self) -> bool {
		return self.test_id == other.test_id;
	}
}

const MAX_DEPTH: i16 = 300;

// parent.push_back(tr(new_node from this))
fn make_node(traits: [i16; 4]) -> trees::Tree<CraftingNode> {

	//Calculate if goal or not
	let progress = traits[1];
	let mut item_complete = false;
	if progress >= 100 {
		item_complete = true;
	}

	let new_node = CraftingNode {
		test_id: traits[0],
		is_goal: item_complete,
		progress: traits[1],
		quality: traits[2],
		durability: traits[3],
	};

	return tr(new_node);
}


//Replace with struct/according to struct or something?
fn synthesis(old: &CraftingNode) -> trees::Tree<CraftingNode> {
	return make_node([old.test_id*2,
		old.progress + 20,
		old.quality,
		old.durability - 10]);
}

fn touch(old: &CraftingNode) -> trees::Tree<CraftingNode> {
	return make_node([old.test_id*2,
		old.progress,
		old.quality + 30,
		old.durability - 10]);
}


fn generate_children(root: &mut Node<CraftingNode>) {

	for mut current in root.iter_mut() {
    
		let current_data = current.data().clone();

		let current_id = current_data.test_id;
    	println!("\n{}", current_id);

    	//children nodes according to action types
    	if current_data.durability > 10 {
	    	current.push_back(touch(&current_data));
			current.push_back(synthesis(&current_data));	
    	}

    	let child_iter = current.iter_mut();
    	//Once tree is built, this is equivalent of going down in depth, move to functions
    }
}


fn main() {
    println!("\n-------------");
    println!("Run Start");
    println!("-------------\n");

    //First, build up the tree
    //Set up the root
    let mut root = tr(CraftingNode {
    	test_id: 0,
    	is_goal: false,
    	progress: 0,
    	quality: 0,
    	durability: 40,
    });

    generate_children(&mut root);
    

    //Now can do IDDFS over the tree


    println!("\nEND\n");
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
