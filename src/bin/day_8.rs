fn main() {
    let is_visible = |x: usize, y: usize, area: &Vec<Vec<u8>>| {
        !area[y][..x]
            .iter()
            .any(|other_tree| *other_tree >= area[y][x])
            || !area[y][x + 1..]
                .iter()
                .any(|other_tree| *other_tree >= area[y][x])
            || !area[..y].iter().any(|r| r[x] >= area[y][x])
            || !area[y + 1..].iter().any(|r| r[x] >= area[y][x])
            || y == area.len() - 1
            || x == area[0].len() - 1
            || x == 0
            || y == 0
    };

    let tree_map: Vec<Vec<u8>> = include_str!("../inputs/day_8.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().map(|c| c as u8 - 0x30).collect::<Vec<u8>>())
        .collect();
    let mut visible_trees: Vec<Vec<bool>> = vec![vec![false; tree_map[0].len()]; tree_map.len()];

    for (x, row) in tree_map.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            visible_trees[y][x] = is_visible(x, y, &tree_map);
        }
    }

    let mut best_scenery = 0;
    for (i, row) in tree_map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let view = distance(tree_map[i][j], tree_map[..i].iter().map(|x| x[j]).collect::<Vec<u8>>().iter().rev()) * // up
            distance(tree_map[i][j], tree_map[i+1..].iter().map(|x| x[j]).collect::<Vec<u8>>().iter()) * // down
            distance(tree_map[i][j], tree_map[i][..j].iter().rev()) * // left
            distance(tree_map[i][j],tree_map[i][j+1..].iter()); // right

            if view > best_scenery {
                best_scenery = view;
            }
        }
    }

    println!(
        "p1: {}",
        visible_trees
            .iter()
            .map(|r| r.iter().filter(|t| **t).count())
            .sum::<usize>()
    );
    println!("p2: {}", best_scenery);
}

fn distance<'a, I>(tree: u8, direction: I) -> u32
where
    I: Iterator<Item = &'a u8>,
{
    let mut distance = 0;
    for other_tree in direction {
        distance += 1;
        if *other_tree >= tree {
            break;
        }
    }
    distance
}
