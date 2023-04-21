// Copyright 2023 Andre Cipriani Bandarra
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::fpgrowth::{FpNode, FpTree};
use rayon::Scope;
use std::sync::{
    mpsc::{channel, Sender},
    Arc,
};

unsafe impl<'a> Send for FpNode<'a> {}
unsafe impl<'a> Sync for FpNode<'a> {}

unsafe impl<'a> Send for FpTree<'a> {}
unsafe impl<'a> Sync for FpTree<'a> {}

fn handle_item<'a>(
    fp_tree: Arc<FpTree<'a>>,
    path: Vec<&'a str>,
    tx: Sender<(Vec<&'a str>, usize)>,
    s: &Scope<'a>,
    item: &'a str,
) {
    s.spawn(move |s| {
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
                paralled_fp_growth_tree(conditional_tree, cloned_path, cloned_tx, s);
            }
            _ => return,
        }
    });
}
fn paralled_fp_growth_tree<'a>(
    fp_tree: FpTree<'a>,
    path: Vec<&'a str>,
    tx: Sender<(Vec<&'a str>, usize)>,
    s: &Scope<'a>,
) {
    // TODO: this should be from less frequent to most frequent.
    let fp_tree = Arc::new(fp_tree);
    for item in fp_tree.frequencies.keys() {
        handle_item(fp_tree.clone(), path.clone(), tx.clone(), s, item);
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
