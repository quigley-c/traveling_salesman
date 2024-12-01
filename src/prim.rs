// Carson Quigley - CS 575 Prim's Algorithm
use crate::binary_heap;

pub fn prim(mut heap: binary_heap::Heap, edges: Vec<binary_heap::Edge>, start_v: usize) -> Vec<binary_heap::Vert> {
    /* For Prim, as opposed to Dijkstra, we keep track of the total distance
     * traveled ove the whole graph instead of the shortest path between 2 nodes.
     * To this end, we modify the loop to compare neighboring edge weights rather
     * than cumulative weights. */
    binary_heap::decrease_key(&mut heap, start_v, 0.0);
    let mut path: Vec<binary_heap::Vert> = vec![];
    while heap.data.len() > 0 {
        // we set the start vertex as having distance 0 in advance (hopefully)
        let v = binary_heap::heap_extract(&mut heap).expect("");
        binary_heap::sift_down(&mut heap, 0);

        // we popped the next shortest distance so we know it's in the minimum tree.
        path.push(v.clone());

        if v.label == f32::MAX { break; } // useful for unconnected pairs

        for e in &v.edges {
            if heap.indicies[edges[*e].to] >= heap.data.len() {
                // we've visited the node already,
                // and it's already been popped from the heap
                continue;
            }
            let new_dist = edges[*e].val as f32;
            if new_dist < heap.data[heap.indicies[edges[*e].to]].label {
                let vi = &mut heap.data[heap.indicies[edges[*e].to]].clone();
                heap.data[heap.indicies[edges[*e].to]].rev = Some(v.name);
                binary_heap::decrease_key(&mut heap, vi.name, new_dist);
            }
        }
    }

    return path;
}
