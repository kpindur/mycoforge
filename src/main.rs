mod syntax_tree;

use crate::syntax_tree::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_traversals<T>(
        tree: &SyntaxTree<T>,
        ground_truths: &(Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>),
    ) where
        T: PartialEq,
    {
        assert_eq!(
            ground_truths.0,
            tree.dfs(0, Order::Prefix).unwrap(),
            "DFS Preorder traversal failed!"
        );
        assert_eq!(
            ground_truths.1,
            tree.dfs(0, Order::Postfix).unwrap(),
            "DFS Postorder traversal failed!"
        );
        assert_eq!(
            ground_truths.2,
            tree.dfs(0, Order::Infix).unwrap(),
            "DFS Inforder traversal failed!"
        );
    }

    #[test]
    fn test1() {
        let mut test_cases: Vec<(Vec<&str>, Vec<Option<Vec<usize>>>)> = Vec::new();
        test_cases.push((vec!["+", "x", "3.0"], vec![Some(vec![1, 2]), None, None]));
        test_cases.push((vec!["ln", "x"], vec![Some(vec![1]), None]));
        test_cases.push((
            vec!["*", "+", "x", "y", "/", "z", "sin", "pi"],
            vec![
                Some(vec![1, 4]),
                Some(vec![2, 3]),
                None,
                None,
                Some(vec![5, 6]),
                None,
                Some(vec![7]),
                None,
            ],
        ));

        let mut ground_truths: Vec<(Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>)> = Vec::new();
        ground_truths.push((vec![0, 1, 2], vec![1, 2, 0], vec![1, 0, 2], vec![0, 1, 2]));
        ground_truths.push((vec![0, 1], vec![1, 0], vec![1, 0], vec![0, 1]));
        ground_truths.push((
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![1, 2, 3, 4, 5, 6, 7, 0],
            vec![2, 1, 3, 0, 5, 4, 7, 6],
            vec![0, 1, 4, 2, 3, 5, 6, 7],
        ));

        let test_trees = test_cases
            .iter()
            .map(|test_case| {
                let labels: &Vec<&str> = &test_case.0;
                let children: &Vec<Option<Vec<usize>>> = &test_case.1;

                let mut tree: SyntaxTree<f32> = SyntaxTree::new();

                for (label, children) in labels.iter().zip(children.iter()) {
                    let node: Node<f32> = Node::new(label.to_string(), 0.0);
                    tree.insert(node, children.clone());
                }

                tree
            })
            .collect::<Vec<SyntaxTree<f32>>>();

        for (test_tree, ground_truth) in test_trees.iter().zip(ground_truths.iter()) {
            test_traversals(&test_tree, &ground_truth);
        }
    }
}
