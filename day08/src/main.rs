use ndarray::{Array2, Axis};

fn mark_visible<'a, I1, I2>(trees: I1, visible: I2)
where
    I1: Iterator<Item = i8>,
    I2: Iterator<Item = &'a mut bool>,
{
    let mut tallest = -1;
    for (tree, vis) in trees.zip(visible) {
        if tree > tallest {
            *vis = true;
            tallest = tree;
        }
        if tree >= 9 {
            break;
        }
    }
}

fn main() {

    ///////////////////////////////////////////////////////////////////////////
    // Load input data
    ///////////////////////////////////////////////////////////////////////////
    let input = include_str!("input");
    //let input = include_str!("sample");
    let width = input.find('\n').unwrap();
    let trees: Vec<i8> = input
        .as_bytes()
        .iter()
        .filter_map(|x| match x {
            b'0'..=b'9' => Some((x - b'0') as i8),
            _ => None,
        })
        .collect();
    let trees = Array2::from_shape_vec((width, trees.len() / width), trees).unwrap();

    for row in trees.rows() {
        for tree in row.iter() {
            print!("{}", tree);
        }
        println!();
    }

    ///////////////////////////////////////////////////////////////////////////
    // Part 1
    ///////////////////////////////////////////////////////////////////////////

    let mut visible = Array2::from_elem(trees.raw_dim(), false);

    for (row_t, mut row_v) in trees.rows().into_iter()
        .zip(visible.rows_mut().into_iter())
    {
        mark_visible(row_t.iter().copied(), row_v.iter_mut());
    }

    let mut fliplr_trees = trees.view();
    let mut fliplr_visible = visible.view_mut();
    fliplr_trees.invert_axis(Axis(1));
    fliplr_visible.invert_axis(Axis(1));
    for (row_t, mut row_v) in fliplr_trees.rows().into_iter()
        .zip(fliplr_visible.rows_mut().into_iter())
    {
        mark_visible(row_t.iter().copied(), row_v.iter_mut());
    }

    let mut t_trees = trees.view().reversed_axes();
    let mut t_visible = visible.view_mut().reversed_axes();
    for (row_t, mut row_v) in t_trees.rows().into_iter()
        .zip(t_visible.rows_mut().into_iter())
    {
        mark_visible(row_t.iter().copied(), row_v.iter_mut());
    }

    t_trees.invert_axis(Axis(1));
    t_visible.invert_axis(Axis(1));
    for (row_t, mut row_v) in t_trees.rows().into_iter()
        .zip(t_visible.rows_mut().into_iter())
    {
        mark_visible(row_t.iter().copied(), row_v.iter_mut());
    }

    let sum: u32 = visible.iter().map(|x| *x as u32).sum();

    ///////////////////////////////////////////////////////////////////////////
    // Part 2
    ///////////////////////////////////////////////////////////////////////////

    let mut best_val = 0;
    let mut best_loc = (0,0);
    for (row, row_t) in trees.rows().into_iter().enumerate() {
        for (col, tree) in row_t.iter().enumerate() {
            let tree = *tree;
            let row_t = trees.row(row);
            let col_t = trees.column(col);
            let s0 = row_t.iter().skip(col+1).position(|&x| x >= tree).map(|x| x+1).unwrap_or(row_t.len()-col-1);
            let s1 = row_t.iter().take(col).rev().position(|&x| x >= tree).map(|x| x+1).unwrap_or(col);
            let s2 = col_t.iter().skip(row+1).position(|&x| x >= tree).map(|x| x+1).unwrap_or(col_t.len()-row-1);
            let s3 = col_t.iter().take(row).rev().position(|&x| x >= tree).map(|x| x+1).unwrap_or(row);
            let score = s0 * s1 * s2 * s3;
            if score > best_val {
                best_loc = (row, col);
                best_val = score;
            }
        }
    }

    for (row, row_t) in visible.rows().into_iter().enumerate() {
        for (col, tree) in row_t.iter().enumerate() {
            if best_loc == (row, col) {
                print!("\x1b[47m🎄\x1b[49m");
            }
            else if *tree {
                print!("🌲");
            } else {
                print!("  ");
            }
        }
        println!();
    }

    println!("Part 1 solution: {}", sum);
    println!("Part 2 solution: {}", best_val);

}
