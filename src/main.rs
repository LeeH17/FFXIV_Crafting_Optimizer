use std::io::{stdout, BufWriter};


const MAX_DEPTH: i16 = 300;


fn main() {
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Begin crafting optimization?");
    let width = message.chars().count();

    //let mut writer = BufWriter::new(stdout.lock());
    //say(message.as_bytes(), width, &mut writer).unwrap();
    println!("{}",message);
}


//Struct representing a node
struct Node{
	test_ID: i16, //Temporary ID for testing known graphs
	is_Goal: bool,
	/* FFXIV specific parts, begin with testing IDDFS
	remaining_Durability: i16,
	remaining_CP: i16,
	
	quality_Expected: i16,
	quality_Max: i16,
	quality_Min: i16,

	progress_Expected: i16,
	progress_Max: i16,
	progress_Min: i16,*/

	child_Nodes: Vec<Node>,
}



fn IterativeDepthFirstSearch(root: Node) {
	//Search to depths of increasing size
	for depth in 1...MAX_DEPTH {
		//Start a search from the root to the given depth
		//Get tuple telling us if there are remaining deeper nodes and returning any found nodes.

		(found_Node, nodes_Remaining) = DepthFirstSearch(root, depth);


		if found_Node != null {
			return found_Node;
			/*
			TODO: Update this so instead of returning first success node and stopping, instead
				display the found crafting path and continue searching for better ones
			*/

		} else if nodes_Remaining == false {
			//Searched through entire tree, failed to find successful path
			return NULL;
		}

	}

}

//Depth limited search for directed graphs
fn DepthFirstSearch(current_Node: Node, depth: i16){
	if depth == 0 {
		//We've hit max depth for this iteration, treat as leaf node
		if current_Node.is_Goal {
			return (current_Node, TRUE);
		} else {
			//There are still remaining children potentially, but they
			//  will be visited in the next iteration with higher depth
			return (NULL, TRUE);
		}

	} else if depth > 0 {
		let nodes_Remain = FALSE;

		//Depth remaining, check the child nodes
		for child in current_Node.child_Nodes.iter() {

			//Recurse down to the next depth
			(found_Node, nodes_Remaining) = DepthFirstSearch(child, depth-1);

			if found != NULL {
				//We found the goal node, but there are still nodes we haven't checked
				return (found_Node, TRUE);

			} else if nodes_Remaining {
				//Haven't found the node, but there are still nodes we haven't checked
				nodes_Remain = TRUE;
			}
		}

		//We didn't find the goal node, return whether any nodes remain as well.
		return (NULL, nodes_Remain);

	}


}


/*

IDFFS example from wikipedia
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
