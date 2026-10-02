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
    occurrences: usize,
    children: BTreeMap<&'a str, FpNode<'a>>,
}

impl<'a> FpNode<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            key,
            occurrences: 0,
            children: Default::default(),
        }
    }

    pub fn add(&mut self, transaction: &[&'a str], count: usize) {
        assert_eq!(
            transaction[0], self.key,
            "First transaction item must match node key"
        );
        self.occurrences += count;

        let mut current = self;
        for &item in &transaction[1..] {
            current = current
                .children
                .entry(item)
                .or_insert_with(|| FpNode::new(item));
            current.occurrences += count;
        }
    }

    pub fn build_conditional_tree(
        &self,
        item: &str,
        mut path: Vec<&'a str>,
    ) -> Vec<(Vec<&'a str>, usize)> {
        if self.key == item {
            return vec![(path, self.occurrences)];
        }

        let mut lists = vec![];
        path.push(self.key);
        for child in self.children.values() {
            lists.append(&mut child.build_conditional_tree(item, path.clone()));
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
        for (items, occurrence) in transactions {
            let mut unique_items = items.clone();
            unique_items.sort_unstable();
            unique_items.dedup();
            for item in unique_items {
                frequencies
                    .entry(item)
                    .and_modify(|count| *count += *occurrence)
                    .or_insert(*occurrence);
            }
        }

        // Build the FP-Tree
        let mut root = BTreeMap::new();
        for (transaction, count) in transactions {
            // Filter out items whose frequency is below min_support, and deduplicate within transaction.
            let mut unique_items = transaction.clone();
            unique_items.sort_unstable();
            unique_items.dedup();
            let mut transaction = unique_items
                .into_iter()
                .filter(|item| frequencies.get(item).copied().unwrap_or(0) >= min_support)
                .collect::<Vec<_>>();
            if transaction.is_empty() {
                continue;
            }

            // Sort transaction by descending item frequency (tie-breaker: lexicographical order).
            transaction.sort_by(|a, b| {
                let a_freq = frequencies.get(a).copied().unwrap_or(0);
                let b_freq = frequencies.get(b).copied().unwrap_or(0);
                let cmp = b_freq.cmp(&a_freq);
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
    // Process items from least frequent to most frequent (standard FP-Growth bottom-up order)
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

    for (item, frequency) in items {
        let mut path = path.clone();
        path.push(item);
        collect(path.as_slice(), frequency);
        let conditional_tree = fp_tree.build_conditional_tree(item);
        fp_growth_tree(&conditional_tree, collect, path);
    }
}

/// Collects frequent item sets from the provided transactions.
///
/// # Arguments
/// * `transactions` - a list of transactions.
/// * `min_support` - the minimum support.
/// * `collect` - a closure that will be invoked when a new item set that matches the minimum
///   support is found.
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
/// fp_growth(transactions.as_slice(), 3, |item_set, occurrences| {
///     println!("{:?}: {}", item_set, occurrences)
/// });
/// ```
pub fn fp_growth<F>(transactions: &[Vec<&str>], min_support: usize, mut collect: F)
where
    F: FnMut(&[&str], usize),
{
    let fp_tree = FpTree::new(transactions, min_support);
    fp_growth_tree(&fp_tree, &mut collect, vec![]);
}
