use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Begin crafting optimization?");
    let width = message.chars().count();

    //let mut writer = BufWriter::new(stdout.lock());
    //say(message.as_bytes(), width, &mut writer).unwrap();
    println!("{}",message);
}




/*


IDFFS example from wikipedia
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
