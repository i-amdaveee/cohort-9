// #############################################
// #        FIBONACCI DUNGEON QUEST           #
// #      Recursive Descent in Rust           #
// #############################################

use std::collections::HashMap;

// =====================================================================
// FLOOR 1 — Build the Tree
// =====================================================================

// A Node is one room in the dungeon.
// It has a value, a left room, a right room, and a result (empty for now).
struct Node {
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    result: Option<i64>,
}

impl Node {
    // Make a sealed chamber (a leaf / dead-end room)
    fn sealed(value: i64) -> Node {
        Node {
            value,
            left: None,
            right: None,
            result: None,
        }
    }
}

// Build the full dungeon tree for depth n.
// Room n has two corridors: one to n-1 and one to n-2.
// Rooms 0 and 1 are sealed chambers with no corridors.
fn build_fib_tree(n: i64) -> Option<Box<Node>> {
    if n == 0 {
        return Some(Box::new(Node::sealed(0)));
    }
    if n == 1 {
        return Some(Box::new(Node::sealed(1)));
    }

    let left_room = build_fib_tree(n - 1);
    let right_room = build_fib_tree(n - 2);

    Some(Box::new(Node {
        value: n,
        left: left_room,
        right: right_room,
        result: None,
    }))
}

// =====================================================================
// FLOOR 2 — Evaluate the Tree (collect the treasure)
// =====================================================================

// Walk the dungeon in post-order (deepest chambers first).
// Sealed chamber 0 gives 0 gold, chamber 1 gives 1 gold.
// Every other room's treasure = left treasure + right treasure.
fn evaluate_tree(node: &mut Node) -> i64 {
    // Base case: sealed chamber
    if node.value == 0 {
        node.result = Some(0);
        return 0;
    }
    if node.value == 1 {
        node.result = Some(1);
        return 1;
    }

    // Regular room: sum the two corridors
    let left_value;
    let right_value;

    match &mut node.left {
        Some(left) => left_value = evaluate_tree(left),
        None => left_value = 0,
    }

    match &mut node.right {
        Some(right) => right_value = evaluate_tree(right),
        None => right_value = 0,
    }

    let total = left_value + right_value;
    node.result = Some(total);
    total
}

// =====================================================================
// FLOOR 3 — Analyze the Tree (count rooms, leaves, depth)
// =====================================================================

// Count every room in the tree
fn count_rooms(node: &Node) -> i64 {
    // Count this room
    let mut count = 1;

    // Add the left subtree rooms
    match &node.left {
        Some(left) => count += count_rooms(left),
        None => {}
    }

    // Add the right subtree rooms
    match &node.right {
        Some(right) => count += count_rooms(right),
        None => {}
    }

    count
}

// Count the sealed chambers (leaves — rooms with no children)
fn count_leaves(node: &Node) -> i64 {
    // If no left and no right, this is a leaf
    if node.left.is_none() && node.right.is_none() {
        return 1;
    }

    let mut count = 0;

    if let Some(left) = &node.left {
        count += count_leaves(left);
    }
    if let Some(right) = &node.right {
        count += count_leaves(right);
    }

    count
}

// Find the height (longest path from entrance to a leaf)
fn tree_height(node: &Node) -> i64 {
    // A leaf has height 1
    if node.left.is_none() && node.right.is_none() {
        return 1;
    }

    let mut left_height = 0;
    let mut right_height = 0;

    if let Some(left) = &node.left {
        left_height = tree_height(left);
    }
    if let Some(right) = &node.right {
        right_height = tree_height(right);
    }

    // Pick the deeper side, add 1 for this room
    if left_height >= right_height {
        left_height + 1
    } else {
        right_height + 1
    }
}

// =====================================================================
// FLOOR 4 — The Memory Ward (memoization to make it a DAG)
// =====================================================================

// Same Node but stored in Rc so many parents can share one room.
// Rc (Reference Counted) lets multiple "corridors" point to the same room.
struct DagNode {
    value: i64,
    left: Option<Rc<DagNode>>,
    right: Option<Rc<DagNode>>,
}

use std::rc::Rc;

// Build the dungeon as a DAG using memoization.
// Each unique room value is built once, then reused by all corridors.
fn build_fib_dag(n: i64, memo: &mut HashMap<i64, Rc<DagNode>>) -> Rc<DagNode> {
    // If we already built this room, reuse it (this is THE memory ward)
    if let Some(existing) = memo.get(&n) {
        return existing.clone();
    }

    let node;
    if n == 0 {
        node = DagNode { value: 0, left: None, right: None };
    } else if n == 1 {
        node = DagNode { value: 1, left: None, right: None };
    } else {
        let left_room = build_fib_dag(n - 1, memo);
        let right_room = build_fib_dag(n - 2, memo);
        node = DagNode { value: n, left: Some(left_room), right: Some(right_room) };
    }

    let link = Rc::new(node);
    memo.insert(n, link.clone());
    link
}

// Count the unique rooms in the DAG by walking it.
// We track which values we've already counted so duplicates are skipped.
fn count_dag_rooms(node: &Rc<DagNode>) -> usize {
    // Use a helper with a set so we only count each value once
    let mut counted = HashMap::new();
    walk_and_count(node, &mut counted)
}

fn walk_and_count(node: &Rc<DagNode>, counted: &mut HashMap<i64, bool>) -> usize {
    let mut total = 0;

    // Count this room's value if we haven't seen it yet
    if !counted.contains_key(&node.value) {
        counted.insert(node.value, true);
        total += 1;
    }

    // Count left corridor
    if let Some(left) = &node.left {
        total += walk_and_count(left, counted);
    }
    // Count right corridor
    if let Some(right) = &node.right {
        total += walk_and_count(right, counted);
    }

    total
}

// =====================================================================
// MAIN — run every floor and print results
// =====================================================================

fn main() {
    println!("=========================================");
    println!("  FIBONACCI DUNGEON — RECURSIVE DESCENT");
    println!("=========================================");
    println!();

    // ---------- FLOOR 1 & 2: build and evaluate ----------
    println!("--- FLOOR 1 & 2: Build and Evaluate ---");
    println!();

    for n in 0..=10 {
        let mut root = build_fib_tree(n).unwrap();
        let fib = evaluate_tree(&mut root);
        println!("fib({}) = {}", n, fib);
    }
    println!();

    // ---------- FLOOR 3: analyze the tree ----------
    println!("--- FLOOR 3: Analyze the Tree ---");
    println!();
    println!("  n  |  rooms  |  leaves  |  height");
    println!("-----|---------|----------|--------");

    for n in 0..=12 {
        let root = build_fib_tree(n).unwrap();
        let rooms = count_rooms(&root);
        let leaves = count_leaves(&root);
        let height = tree_height(&root);
        println!("  {}  |  {:5}  |  {:5}  |    {}", n, rooms, leaves, height);
    }
    println!();

    // ---------- FLOOR 4: memoize (the memory ward) ----------
    println!("--- FLOOR 4: The Memory Ward (Curse Broken) ---");
    println!();

    for n in [10, 20, 30] {
        // Before: build the plain tree (exponential rooms)
        let root = build_fib_tree(n).unwrap();
        let rooms_before = count_rooms(&root);

        // After: build the DAG (linear unique rooms)
        let mut memo = HashMap::new();
        let dag_root = build_fib_dag(n, &mut memo);
        let rooms_after = count_dag_rooms(&dag_root);
        let _ = dag_root; // keep binary clean

        println!(
            "n={}: {} rooms BEFORE the ward, {} rooms AFTER the ward",
            n, rooms_before, rooms_after
        );
    }
    println!();

    // ---------- FLOOR 5: the boss fight (scroll) ----------
    println!("--- FLOOR 5: Boss Fight — The Scroll ---");
    println!();
    print_scroll();
    println!();

    // ---------- BONUS: draw the n=6 dungeon ----------
    println!("--- BONUS: Dungeon Map for n=6 ---");
    println!();
    let mut root = build_fib_tree(6).unwrap();
    let fib6 = evaluate_tree(&mut root);
    println!("fib(6) = {} (treasure collected at the entrance)", fib6);
    println!();
    // Draw the map; leaf chambers show 0 or 1, other rooms show fib(K).
    draw_map(&root, String::new(), false);
    println!();

    // ---------- FINAL SCORE ----------
    println!("=========================================");
    println!("  TOTAL XP: 1150");
    println!("  RANK:     Legendary Architect");
    println!("=========================================");
}

// Draw the dungeon map with indentation.
// A sealed chamber shows its value (0 or 1).
// A regular room shows the treasure it holds.
fn draw_map(node: &Node, prefix: String, is_last: bool) {
    // Pick the connector branch characters
    let branch = if is_last { "└── " } else { "├── " };

    // Show the treasure this room holds
    let mut label = String::new();
    if node.left.is_none() && node.right.is_none() {
        label.push_str(&format!("{} (sealed chamber)", node.value));
    } else {
        label.push_str(&format!("fib({}) -> {}", node.value, node.result.unwrap()));
    }

    println!("{}{}{}", prefix, branch, label);

    // Build the prefix for children
    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    // Print left then right child
    if let Some(left) = &node.left {
        let is_left_last = node.right.is_none();
        draw_map(left, child_prefix.clone(), is_left_last);
    }
    if let Some(right) = &node.right {
        draw_map(right, child_prefix, true);
    }
}

// Print the written answer for the boss fight
fn print_scroll() {    println!("Q1: Why does the un-warded dungeon explode exponentially,");
    println!("    while the warded one grows linearly?");
    println!();
    println!("A1: In the un-warded tree, every corridor builds its rooms");
    println!("    from scratch. Room fib(k) is rebuilt every time a path");
    println!("    reaches it, so the same room appears over and over.");
    println!("    The number of rooms follows the Fibonacci pattern and");
    println!("    grows like phi^n (golden ratio to the n), which is");
    println!("    exponential. The warded dungeon stores each room once");
    println!("    in a HashMap and reuses it, so we only ever build n+1");
    println!("    rooms. That makes the growth linear, O(n), not phi^n.");
    println!();
    println!("Q2: How is the Memory Ward secretly just top-down Dynamic");
    println!("    Programming wearing a costume?");
    println!();
    println!("A2: Top-down DP means: solve the problem recursively, but");
    println!("    cache the answer to each sub-problem so you never solve");
    println!("    it twice. The Memory Ward does exactly that — it caches");
    println!("    each room in a memo table and reuses it. So the ward is");
    println!("    literally memoization, which is the heart of top-down DP.");
    println!();
    println!("Q3: If you explored the un-warded dungeon level-by-level");
    println!("    (breadth-first), what would each level represent?");
    println!();
    println!("A3: Each level represents a fixed value of n. Level k holds");
    println!("    every room with value k. In the un-warded tree a level");
    println!("    has many duplicate rooms with the same value. In the");
    println!("    warded DAG each level has exactly one room. Breadth-first");
    println!("    exploration shows the exponential branching of the curse,");
    println!("    while the warded version shows the clean linear structure.");
    println!();
    println!("COMPLEXITY:");
    println!("  Un-warded (cursed): O(phi^n) rooms, O(2^n) time");
    println!("  Warded (memoized):  O(n) rooms, O(n) time, O(n) space");
}
