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

use crate::fpgrowth::FpTree;
use rayon::Scope;
use std::sync::{
    mpsc::{channel, Sender},
    Arc,
};

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
                parallel_fp_growth_tree(conditional_tree, cloned_path, cloned_tx, s);
            }
            _ => (),
        }
    });
}

fn parallel_fp_growth_tree<'a>(
    fp_tree: FpTree<'a>,
    path: Vec<&'a str>,
    tx: Sender<(Vec<&'a str>, usize)>,
    s: &Scope<'a>,
) {
    let fp_tree = Arc::new(fp_tree);
    let mut items: Vec<_> = fp_tree
        .frequencies
        .iter()
        .filter(|(_, &freq)| freq >= fp_tree.min_support)
        .map(|(&item, &freq)| (item, freq))
        .collect();

    items.sort_by(|(item_a, freq_a), (item_b, freq_b)| {
        let cmp = freq_a.cmp(freq_b);
        if std::cmp::Ordering::Equal == cmp {
            item_b.cmp(item_a)
        } else {
            cmp
        }
    });

    for (item, _) in items {
        handle_item(fp_tree.clone(), path.clone(), tx.clone(), s, item);
    }
}

/// Collects frequent item sets from the provided transactions, using a parallel implementation.
///
/// # Arguments
/// * `transactions` - a list of transactions.
/// * `min_support` - the minimum support.
/// * `collect` - a closure that will be invoked when a new item set that matches the minimum
///   support is found.
///
/// # Example
/// ```
/// use fpgrowth_rs::parallel_fp_growth;
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
/// parallel_fp_growth(transactions.as_slice(), 3, |item_set, occurrences| {
///     println!("{:?}: {}", item_set, occurrences)
/// });
/// ```
pub fn parallel_fp_growth<'a, F>(transactions: &[Vec<&'a str>], min_support: usize, mut collect: F)
where
    F: FnMut(&[&'a str], usize) + Send,
{
    let fp_tree = FpTree::new(transactions, min_support);
    let (tx, rx) = channel::<(Vec<&'a str>, usize)>();

    std::thread::scope(|thread_scope| {
        let handle = thread_scope.spawn(move || {
            while let Ok((item_set, occurrences)) = rx.recv() {
                collect(&item_set, occurrences);
            }
        });

        rayon::scope(|s| {
            parallel_fp_growth_tree(fp_tree, vec![], tx, s);
        });

        handle.join().unwrap();
    });
}
