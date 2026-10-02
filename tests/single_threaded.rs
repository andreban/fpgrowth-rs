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

use fpgrowth_rs::fp_growth;

#[test]
fn test_single_threaded() {
    let transactions = vec![
        vec!["E", "A", "D", "B"],
        vec!["D", "A", "C", "E", "B"],
        vec!["C", "A", "B", "E"],
        vec!["B", "A", "D"],
        vec!["D"],
        vec!["D", "B"],
        vec!["A", "D", "E"],
        vec!["B", "C"],
    ];

    let mut frequent_item_sets = vec![];
    fp_growth(transactions.as_slice(), 3, |item_set, occurrences| {
        let item_set = item_set.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        frequent_item_sets.push((item_set, occurrences));
    });

    assert_eq!(frequent_item_sets.len(), 15);
}

#[test]
fn test_duplicate_items_in_transaction() {
    let transactions = vec![vec!["A", "A", "B"], vec!["B", "B", "A"], vec!["A", "B"]];

    let mut frequent_item_sets = vec![];
    fp_growth(transactions.as_slice(), 2, |item_set, occurrences| {
        let item_set = item_set.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        frequent_item_sets.push((item_set, occurrences));
    });

    // Each transaction contains {A, B} once. So A: 3, B: 3, {A, B}: 3.
    for (set, count) in &frequent_item_sets {
        assert_eq!(*count, 3, "Itemset {:?} should have support 3", set);
    }
    assert_eq!(frequent_item_sets.len(), 3);
}

#[test]
fn test_empty_transactions() {
    let transactions: Vec<Vec<&str>> = vec![vec![], vec![]];
    let mut count = 0;
    fp_growth(transactions.as_slice(), 1, |_item_set, _occurrences| {
        count += 1;
    });
    assert_eq!(count, 0);
}
