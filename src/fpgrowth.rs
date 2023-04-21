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

use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct FpNode<'a> {
    key: &'a str,
    occurences: usize,
    children: BTreeMap<&'a str, FpNode<'a>>,
}

impl<'a> FpNode<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            key,
            occurences: 0,
            children: Default::default(),
        }
    }

    pub fn add(&mut self, transaction: &[&'a str], count: usize) {
        if transaction[0] != self.key {
            panic!()
        }
        self.occurences += count;

        if transaction.len() == 1 {
            return;
        }

        self.children
            .entry(transaction[1])
            .or_insert_with(|| FpNode::new(transaction[1]))
            .add(&transaction[1..], count);
    }

    pub fn build_conditional_tree(
        &self,
        item: &str,
        mut path: Vec<&'a str>,
    ) -> Vec<(Vec<&'a str>, usize)> {
        if self.key == item {
            return vec![(path, self.occurences)];
        }

        let mut lists = vec![];
        path.push(self.key);
        for child in self.children.values() {
            lists.append(&mut child.build_conditional_tree(item, path.clone()))
        }
        lists
    }
}

#[derive(Debug)]
pub(crate) struct FpTree<'a> {
    pub(crate) root: BTreeMap<&'a str, FpNode<'a>>,
    pub(crate) frequencies: BTreeMap<&'a str, usize>,
    pub(crate) min_support: usize,
}

impl<'a> FpTree<'a> {
    pub(crate) fn new(transactions: &[Vec<&'a str>], min_support: usize) -> Self {
        let transactions = transactions
            .iter()
            .map(|t| (t.to_vec(), 1_usize))
            .collect::<Vec<_>>();
        FpTree::build_fp_tree(transactions.as_slice(), min_support)
    }

    fn build_fp_tree(transactions: &[(Vec<&'a str>, usize)], min_support: usize) -> FpTree<'a> {
        // Build Frequency Lists (F-List)
        let mut frequencies = BTreeMap::new();
        for (items, occurance) in transactions {
            for item in items {
                frequencies
                    .entry(*item)
                    .and_modify(|count| *count += occurance)
                    .or_insert(*occurance);
            }
        }

        // Build the FP-Tree
        let mut root = BTreeMap::new();
        for (transaction, count) in transactions {
            // Filter out items which frequency is below min_support.
            let mut transaction = transaction
                .iter()
                .filter(|item| *frequencies.get(*item).unwrap() >= min_support)
                .copied()
                .collect::<Vec<_>>();
            if transaction.is_empty() {
                continue;
            }

            // Sort transaction by item frequency.
            transaction.sort_by(|a, b| {
                let a_freq = frequencies.get(a).unwrap();
                let b_freq = frequencies.get(b).unwrap();
                let cmp = b_freq.cmp(a_freq);
                if std::cmp::Ordering::Equal == cmp {
                    return a.cmp(b);
                }
                cmp
            });

            // Append transaction to the tree root.
            root.entry(transaction[0])
                .or_insert_with(|| FpNode::new(transaction[0]))
                .add(transaction.as_slice(), *count);
        }

        FpTree {
            root,
            frequencies,
            min_support,
        }
    }

    pub(crate) fn build_conditional_tree(&self, item: &str) -> FpTree<'a> {
        // Build transactions...
        let mut lists = vec![];
        let path = Vec::new();
        for node in self.root.values() {
            lists.extend(node.build_conditional_tree(item, path.clone()));
        }

        FpTree::build_fp_tree(lists.as_slice(), self.min_support)
    }
}

fn fp_growth_tree<F>(fp_tree: &FpTree, collect: &mut F, path: Vec<&str>)
where
    F: FnMut(&[&str], usize),
{
    // TODO: this should be from less frequent to most frequent.
    for item in fp_tree.frequencies.keys() {
        match fp_tree.frequencies.get(item) {
            Some(frequency) if *frequency >= fp_tree.min_support => {
                let mut path = path.clone();
                path.push(item);
                collect(path.as_slice(), *frequency);
                let conditional_tree = fp_tree.build_conditional_tree(item);
                fp_growth_tree(&conditional_tree, collect, path);
            }
            _ => continue,
        }
    }
}

/// Collects frequent item sets from the provided transactions.
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
pub fn fp_growth<F>(transactions: &[Vec<&str>], min_support: usize, mut collect: F)
where
    F: FnMut(&[&str], usize),
{
    let fp_tree = FpTree::new(transactions, min_support);
    fp_growth_tree(&fp_tree, &mut collect, vec![]);
}
