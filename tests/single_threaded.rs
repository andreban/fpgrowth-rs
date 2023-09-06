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
    fp_growth(transactions.as_slice(), 3, |item_set, occurences| {
        let item_set = item_set.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        frequent_item_sets.push((item_set, occurences));
    });

    assert_eq!(frequent_item_sets.len(), 15);
}
