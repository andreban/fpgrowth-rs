use crate::fpgrowth::{FpNode, FpTree};
use rayon::Scope;
use std::sync::mpsc::{channel, Sender};

unsafe impl<'a> Send for FpNode<'a> {}
unsafe impl<'a> Sync for FpNode<'a> {}

unsafe impl<'a> Send for FpTree<'a> {}
unsafe impl<'a> Sync for FpTree<'a> {}

fn paralled_fp_growth_tree<'a>(
    fp_tree: FpTree<'a>,
    path: Vec<&'a str>,
    tx: Sender<(Vec<&'a str>, usize)>,
    s: &Scope<'a>,
) {
    // TODO: this should be from less frequent to most frequent.
    for item in fp_tree.frequencies.keys() {
        match fp_tree.frequencies.get(item) {
            Some(frequency) if *frequency >= fp_tree.min_support => {
                // Collect the data.
                let mut path = path.clone();
                path.push(item);
                tx.send((path.clone(), *frequency)).unwrap();

                // Iterate to children.
                let conditional_tree = fp_tree.build_conditional_tree(item);
                let cloned_path = path.clone();
                let cloned_tx = tx.clone();
                s.spawn(move |s| {
                    paralled_fp_growth_tree(conditional_tree, cloned_path, cloned_tx, s);
                });
            }
            _ => continue,
        }
    }
}

/// Collects frequent item sets from the provided transactions, using a parallel implementation.
///
/// # Arguments
/// * `transactions` - a list of transactions.
/// * `min_support` - the minimum support.
/// * `collect` - a closure that will be invoked when a new item set that matches the minimum
///               support is found.
///
/// # Example
/// ```
/// use fpgrowth_rs::fp_growth;
///
/// let transactions = vec![
///     vec!["E", "A", "D", "B"],
///     vec!["D", "A", "C", "E", "B"],
///     vec!["C", "A", "B", "E"],
///     vec!["B", "A", "D"],
///     vec!["D"],
///     vec!["D", "B"],
///     vec!["A", "D", "E"],
///     vec!["B", "C"],
/// ];
///
/// fp_growth(transactions.as_slice(), 3, |item_set, occurences| {
///     println!("{:?}: {}", item_set, occurences)
/// });
///
/// ```
pub fn parallel_fp_growth<'a, F>(transactions: &'a [Vec<&str>], min_support: usize, mut collect: F)
where
    F: FnMut(&[&str], usize) + Send,
{
    rayon::scope(|s| {
        let (tx, rx) = channel::<(Vec<&str>, usize)>();
        let fp_tree = FpTree::new(transactions, min_support);
        paralled_fp_growth_tree(fp_tree, vec![], tx, s);
        while let Ok((a, b)) = rx.recv() {
            collect(&a, b);
        }
    });
}
