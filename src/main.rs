use petgraph::Graph;
use petgraph::Undirected;
use nauty_pet::prelude::*;
use nauty_pet::canon::IntoCanonColor;

fn main() {
    // Two different vertex labellings for the tree graph with two edges
    let mut g1 =  Graph::<(), (), Undirected>::new_undirected();
    let mut g2 =  Graph::<(), (), Undirected>::new_undirected();
    let a1 = g1.add_node(());
    let a2 = g1.add_node(());
    let a3 = g1.add_node(());
    let a4 = g1.add_node(());
    let a5 = g1.add_node(());
    let a6 = g1.add_node(());
    let b3 = g2.add_node(());
    let b2 = g2.add_node(());
    let b1 = g2.add_node(());
    let b6 = g2.add_node(());
    let b5 = g2.add_node(());
    let b4 = g2.add_node(());
    g1.add_edge(a1, a4, ());
    g1.add_edge(a1, a5, ());
    g1.add_edge(a1, a6, ());
    g1.add_edge(a2, a5, ());
    g1.add_edge(a3, a6, ());
    g2.add_edge(b1, b4, ());
    g2.add_edge(b1, b5, ());
    g2.add_edge(b1, b6, ());
    g2.add_edge(b2, b5, ());
    g2.add_edge(b3, b6, ());

    // The canonical forms are identical
    let c1 = g1.clone().into_canon();
    let c2 = g2.clone().into_canon();
    assert!(c1.is_identical(&c2));

    // Default coloring
    let c1 = g1.clone().into_canon_color(&mut [1, 1, 1, 1, 1, 1]);
    let c2 = g2.clone().into_canon_color(&mut [1, 1, 1, 1, 1, 1]);
    assert!(c1.is_identical(&c2));

    // "correct" coloring
    let c1 = g1.clone().into_canon_color(&mut [1, 1, 0, 1, 1, 0]);
    let c2 = g2.clone().into_canon_color(&mut [1, 1, 0, 1, 1, 0]);
    assert!(c1.is_identical(&c2));

    // "incorrect" coloring
    let c1 = g1.clone().into_canon_color(&mut [1, 1, 0, 1, 1, 0]);
    let c2 = g2.clone().into_canon_color(&mut [1, 1, 1, 1, 1, 1]);
    assert!(!c1.is_identical(&c2));
}