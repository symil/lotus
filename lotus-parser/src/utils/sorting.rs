use std::collections::HashSet;

// Depth-first search - https://en.wikipedia.org/wiki/Topological_sorting
pub fn sort_dependancy_graph(links: Vec<Vec<usize>>) -> Result<Vec<usize>, Vec<Vec<usize>>> {
    let mut result = vec![];
    let mut node_stack = vec![];
    let mut temporary_marks = HashSet::new();
    let mut permanent_marks = HashSet::new();
    let mut cycles = vec![];

    while let Some(node_index) = select_unmarked_nodes(links.len(), &permanent_marks) {
        node_stack.push(node_index);
        visit(&mut node_stack, &links, &mut result, &mut cycles, &mut temporary_marks, &mut permanent_marks);
        node_stack.pop();
    }

    match cycles.is_empty() {
        true => Ok(result),
        false => Err(cycles)
    }
}

fn select_unmarked_nodes(node_count: usize, permanent_marks: &HashSet<usize>) -> Option<usize> {
    for i in 0..node_count {
        if !permanent_marks.contains(&i) {
            return Some(i);
        }
    }

    None
}

fn visit(node_stack: &mut Vec<usize>, links: &Vec<Vec<usize>>, result: &mut Vec<usize>, cycles: &mut Vec<Vec<usize>>, temporary_marks: &mut HashSet<usize>, permanent_marks: &mut HashSet<usize>) {
    let node_index = node_stack.last().unwrap().clone();

    if permanent_marks.contains(&node_index) {
        return;
    }

    if temporary_marks.contains(&node_index) {
        let first_index = node_stack.iter().position(|i| i == &node_index).unwrap();
        let cycle = node_stack[first_index..].to_vec();

        cycles.push(cycle);

        return;
    }

    temporary_marks.insert(node_index);

    for neighbor_index in &links[node_index] {
        node_stack.push(*neighbor_index);
        visit(node_stack, links, result, cycles, temporary_marks, permanent_marks);
        node_stack.pop();
    }

    temporary_marks.remove(&node_index);
    permanent_marks.insert(node_index);
    result.push(node_index);
}