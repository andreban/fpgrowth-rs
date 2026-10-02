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

#[derive(Debug, PartialEq)]
pub struct AssociationRule {
    pub antecedent_set: Vec<String>,
    pub consequent_set: Vec<String>,
    pub antecedent_support: f32,
    pub consequent_support: f32,
    pub combined_support: f32,
    pub lift: f32,
    pub confidence: f32,
}

impl AssociationRule {
    pub fn new(
        antecedent_set: Vec<String>,
        consequent_set: Vec<String>,
        antecedent_support: f32,
        consequent_support: f32,
        combined_support: f32,
        lift: f32,
        confidence: f32,
    ) -> Self {
        Self {
            antecedent_set,
            consequent_set,
            antecedent_support,
            consequent_support,
            combined_support,
            lift,
            confidence,
        }
    }
}

/// Generates association rules from frequent item sets.
pub fn generate_association_rules<F>(
    frequent_item_sets: &HashMap<Vec<String>, usize>,
    num_transactions: usize,
    on_association_rule: &mut F,
) where
    F: FnMut(AssociationRule),
{
    if num_transactions == 0 {
        return;
    }

    for (set, &count) in frequent_item_sets {
        if set.len() <= 1 {
            continue;
        }
        let combined_support = count as f32 / num_transactions as f32;
        for item in set {
            let mut antecedent_set = set
                .iter()
                .filter(|i| *i != item)
                .cloned()
                .collect::<Vec<_>>();
            let consequent_set = vec![item.clone()];

            let antecedent_count = frequent_item_sets
                .get(&antecedent_set)
                .copied()
                .or_else(|| {
                    antecedent_set.sort();
                    frequent_item_sets.get(&antecedent_set).copied()
                });

            let Some(antecedent_count) = antecedent_count else {
                continue;
            };

            let Some(&consequent_count) = frequent_item_sets.get(&consequent_set) else {
                continue;
            };

            let antecedent_support = antecedent_count as f32 / num_transactions as f32;
            let consequent_support = consequent_count as f32 / num_transactions as f32;

            if antecedent_support <= 0.0 || consequent_support <= 0.0 {
                continue;
            }

            let lift = combined_support / (antecedent_support * consequent_support);
            let confidence = combined_support / antecedent_support;
            on_association_rule(AssociationRule::new(
                antecedent_set,
                consequent_set,
                antecedent_support,
                consequent_support,
                combined_support,
                lift,
                confidence,
            ));
        }
    }
}
