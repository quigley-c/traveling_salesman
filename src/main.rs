// Carson Quigley - Traveling Salesman

use std::io;
use std::collections::VecDeque;
mod binary_heap;
mod prim;

fn square(x: f32) -> f32 { return x*x; }
fn main() {
    // read stdin
    let lines: Vec<String> = io::stdin()
        .lines()
        .collect::<Result<_,_>>()
        .unwrap();
    let parts: Vec<usize> = lines[0]
        .split(" ")
        .filter_map(|w| w.parse().ok())
        .collect();

    let len_v: usize = parts[0];
    let mut nums: Vec<Vec<f32>> = vec![];
    for l in lines {
        let n: Vec<f32> = l.split(" ")
            .filter_map(|w| w.parse::<f32>().ok())
            .collect();
        if n.len() < 2 {
            // we already grabbed the first line
            continue;
        }
        nums.push(n);
    }

    // we have a vec of nums representing positions in a graph. We need to
    // calculate the weights and represent the edges for the heap init.
    let mut edges: Vec<Vec<f32>> = vec![];
    for n in 0..nums.len() {
        for to in 0..nums.len() {
            if n == to { continue; }
            let d = (
                (
                    square(nums[n][0] - nums[to][0]) +
                    square(nums[n][1] - nums[to][1]))
                    as f32)
                .sqrt();
            edges.push(vec![n as f32, to as f32, d]);
        }
    }

    // get prims minimum spanning tree, then
    // use the length for bounded search through all possible paths
    let tuple = binary_heap::init_heap(edges, len_v as i32);
    let start_v = 0; // might make this a random int < num verts
    let min_tree = prim::prim(tuple.0.clone(), tuple.1.clone(), start_v);

    // add the edge back to the start
    let path = dfs(&min_tree, min_tree[0].clone(), &mut vec![]);
    print_path(&path, &tuple.1);
}

fn print_path(list: &Vec<binary_heap::Vert>, edges: &Vec<binary_heap::Edge>) {
    let mut dist: f32 = 0.0;
    let mut last = list[0].clone();
    let mut c = 0;
    for i in list {
        dist = dist + i.label;
        last = i.clone();
        c = c + 1;
    }

    for e in last.edges {
        if edges[e].to == 0 && edges[e].from == last.name ||
            edges[e].from == 0 && edges[e].to == last.name {
            dist = dist + edges[e].val;
        }
    }

    //println!("{}", dist);
    println!("{}", c);
    for i in list {
        print!("{} ", i.name);
    }
    println!();
}

fn dfs(list: &Vec<binary_heap::Vert>, cur: binary_heap::Vert, out: &mut Vec<binary_heap::Vert>) -> Vec<binary_heap::Vert> {
    out.push(cur.clone());
    let mut children: Vec<binary_heap::Vert> = vec![];
    for i in list {
        if i.rev.is_some() && i.rev.expect("") == cur.name {
            children.push(i.clone());
        }
    }
    for c in children {
        dfs(list, c, out);
    }
    return out.clone();
}

// didn't have time for this part, the min tree should get within a factor of 2.
fn _search(heap: &mut binary_heap::Heap, edges: &Vec<binary_heap::Edge>, start_v: usize, min_cost: f32) -> Vec<binary_heap::Vert> {
    let mut q: VecDeque<usize> = vec![].into();
    q.push_front(start_v);

    let mut list: Vec<Option<binary_heap::Vert>> = vec![];
    for i in 0..heap.data.len() {
        list.push(Some(heap.data[i].clone()));
    }

    let mut last: usize = 0;
    heap.data[heap.indicies[q[0] as usize]].label = 0.0;
    while !q.is_empty() {
        let cur = heap.data[q.pop_front().expect("") as usize].clone();
        if cur.label > min_cost { continue; }
        list[heap.indicies[cur.name]] = None;

        for e in cur.edges {
            let edge = &edges[e];
            if !list[heap.indicies[edge.to]].is_some() { continue; }
            let next_v = &mut heap.data[heap.indicies[edge.to]];
            next_v.label = cur.label + edge.val as f32;
            next_v.rev = Some(cur.name);
            q.push_front(edge.to);
        }
        last = cur.name;
    }

    let mut cur: binary_heap::Vert = heap.data[heap.indicies[last]].clone();
    let mut path: Vec<binary_heap::Vert> = vec![];
    path.push(heap.data[heap.indicies[start_v]].clone());
    while cur.name != start_v {
        path.push(cur.clone());
        if cur.rev.is_some() {
            let rev = cur.rev.expect("");
            for e in &cur.edges {
                if edges[*e].to == rev {
                    break;
                }
            }
            cur = heap.data[heap.indicies[rev]].clone();
        }
    }
    for e in &path[path.len()-2].edges {
        if edges[*e].to == start_v {
            break;
        }
    }
    return path;
}

