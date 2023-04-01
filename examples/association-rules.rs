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

use std::collections::HashMap;

use fpgrowth_rs::fp_growth;

/// This example uses the data from the following blog post:
///  - https://towardsdatascience.com/the-fp-growth-algorithm-1ffa20e839b8
///
/// Interestingly, the blog post seems to have an inconsistency when calculating the association
/// rules. When generating the FP Tree, support for "potato chips" is 0.50, but in the table with
/// the resulting association rules, the "antecedent support" for "potato chips" shows 0.45. This
/// leads to the incorrect confidence and lifts in that table:
///  - The table shows a lift of 1.81 for "potato chips" -> "beer" and to
///     "beer" -> "potato chips", the correct is 1.63.
///  - The table shows the confidence for "potato chips" -> "beer" to be 1.0, the correct is 0.9.
///
fn main() {
    let transactions = vec![
        vec!["beer", "wine", "cheese"],
        vec!["beer", "potato chips"],
        vec!["eggs", "flower", "butter", "cheese"],
        vec!["eggs", "flower", "butter", "beer", "potato chips"],
        vec!["wine", "cheese"],
        vec!["potato chips"],
        vec!["eggs", "flower", "butter", "wine", "cheese"],
        vec!["eggs", "flower", "butter", "beer", "potato chips"],
        vec!["wine", "beer"],
        vec!["beer", "potato chips"],
        vec!["butter", "eggs"],
        vec!["beer", "potato chips"],
        vec!["flower", "eggs"],
        vec!["beer", "potato chips"],
        vec!["eggs", "flower", "butter", "wine", "cheese"],
        vec!["beer", "wine", "potato chips", "cheese"],
        vec!["wine", "cheese"],
        vec!["beer", "potato chips"],
        vec!["wine", "cheese"],
        vec!["beer", "potato chips"],
    ];

    println!("Common Item Sets: ");
    let num_transactions = transactions.len();
    let mut results = HashMap::new();
    fp_growth(transactions, 7, |item_set, occurences| {
        let mut items = item_set.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        items.sort();
        results.insert(items, occurences);
        println!("{:?}: {}", item_set, occurences)
    });


    println!("\nAssociation Rules: ");
    for set in results.keys() {
        if set.len() == 1 {
            continue;
        }
        let support = *results.get(set).unwrap() as f32 / num_transactions as f32;
        for item in set {
            let antecessor_set = set
                .iter()
                .filter(|i| *i != item)
                .map(|i| i.clone())
                .collect::<Vec<_>>();
            let sucessor_set = vec![item.clone()];
            let antecedent_support =
                *results.get(&antecessor_set).unwrap() as f32 / num_transactions as f32;
            let consequent_support =
                *results.get(&sucessor_set).unwrap() as f32 / num_transactions as f32;

            let lift = support / (antecedent_support * consequent_support);
            let confidence = support / antecedent_support;
            println!(
                "{:?} -> {:?}, antecedent_support: {}, consequent_support: {}, support: {}, lift: {}, confidence: {}",
                antecessor_set,
                sucessor_set,
                antecedent_support,
                consequent_support,
                support,
                lift,
                confidence
            );
        }
    }
}
