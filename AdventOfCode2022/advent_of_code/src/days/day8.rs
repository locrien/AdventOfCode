#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 8;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 21;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 1533;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 8;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 345744;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

// ----------------------------

struct Tree
{
    pub visible:bool,
    pub height:u32,
    pub pos:(usize,usize),
}

pub fn run_part_1(input:String) -> u32
{
    let mut trees = parse_input(input);

    look_at(&mut trees);
    
    let visible_trees = trees.iter()
        .flatten()
        .filter(|t| { t.visible});

    let visible_amount = visible_trees.fold(0,|acc,_t| {acc+1});

    println!("visible trees : {}",visible_amount);

    return visible_amount;
}

pub fn run_part_2(input:String) -> u32
{
    let mut trees = parse_input(input);

    look_at(&mut trees);

    let result = trees.iter().flatten().map(|t| {calculate_tree_score(&trees, t.pos)}).max().expect("");

    return result;
}

fn parse_input(input:String) -> Vec::<Vec::<Tree>>
{
    let trees = input.lines().map(
        |l| {
            l.chars().map(
                |c| {
                     let height = c.to_digit(10).unwrap();

                     let tree = Tree {
                        visible : false,
                        height : height,
                        pos : (0,0),
                    };
                     return tree;
                }).collect::<Vec<Tree>>()
        }).collect::<Vec<Vec<Tree>>>();

    return trees;
}

fn look_at(trees:&mut Vec::<Vec::<Tree>>)
{
    // left to right
    for i in 0..trees.len()
    {
        let mut tallest_height:Option<u32> = None;
        for j in 0..trees[i].len()
        {
            let tree = trees[i].get_mut(j).expect("msg");
            tree.pos = (i,j);
            if tallest_height.is_none() || tree.height > tallest_height.unwrap()
            {
                tree.visible = true;
                tallest_height = Some(tree.height);
            }
        }
    }

    // right to left
    for i in 0..trees.len()
    {
        let mut tallest_height:Option<u32> = None;
        let row_length = trees[i].len();
        for j in (0..row_length).rev()
        {
            let tree = trees[i].get_mut(j).expect("msg");
            if tallest_height.is_none() || tree.height > tallest_height.unwrap()
            {
                tree.visible = true;
                tallest_height = Some(tree.height);
            }
        }
    }

    // up to down
    for i in 0..trees.len()
    {
        let mut tallest_height:Option<u32> = None;
        for j in 0..trees[i].len()
        {
            let tree = trees[j].get_mut(i).expect("msg");
            if tallest_height.is_none() || tree.height > tallest_height.unwrap()
            {
                tree.visible = true;
                tallest_height = Some(tree.height);
            }
        }
    }

    // down to up
    for i in 0..trees.len()
    {
        let mut tallest_height:Option<u32> = None;
        let column_length = trees[i].len();
        for j in (0..column_length).rev()
        {
            let tree = trees[j].get_mut(i).expect("msg");
            if tallest_height.is_none() || tree.height > tallest_height.unwrap()
            {
                tree.visible = true;
                tallest_height = Some(tree.height);
            }
        }
    }
}

fn calculate_tree_score(trees:&Vec::<Vec::<Tree>>, pos:(usize,usize)) -> u32
{
    let current_tree = &trees[pos.0][pos.1];
    let current_tree_height = current_tree.height;
    let mut score = 1;

    // up
    let mut dir_score = 0;
    for i in (0..pos.0).rev()
    {
        let other_tree_height = trees[i][pos.1].height;
        if other_tree_height >= current_tree_height
        {
            dir_score = dir_score + 1;
            break;
        }
        dir_score = dir_score + 1;
    }
    score = score * dir_score;

    // down
    let mut dir_score = 0;
    for i in (pos.0+1)..trees.len()
    {
        if i >= trees.len()
        {
            break;
        }

        let other_tree_height = trees[i][pos.1].height ;
        if other_tree_height >= current_tree_height
        {
            dir_score = dir_score + 1;
            break;
        }
        dir_score = dir_score + 1;
    }
    score = score * dir_score;

    // left
    let mut dir_score = 0;
    for i in (0..pos.1).rev()
    {
        let other_tree_height = trees[pos.0][i].height ;
        if other_tree_height >= current_tree_height
        {
            dir_score = dir_score + 1;
            break;
        }
        dir_score = dir_score + 1;
    }
    score = score * dir_score;

    // right
    let mut dir_score = 0;
    for i in (pos.1+1)..trees[pos.0].len()
    {
        if i >= trees[pos.0].len()
        {
            break;
        }

        let other_tree_height = trees[pos.0][i].height ;
        if other_tree_height >= current_tree_height
        {
            dir_score = dir_score + 1;
            break;
        }

        dir_score = dir_score + 1;
    }
    score = score * dir_score;

    return score;
}