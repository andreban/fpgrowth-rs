use std::collections::HashMap;

#[derive(Debug)]
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
    for set in frequent_item_sets.keys() {
        if set.len() == 1 {
            continue;
        }
        let combined_support =
            *frequent_item_sets.get(set).unwrap() as f32 / num_transactions as f32;
        for item in set {
            let antecedent_set = set
                .iter()
                .filter(|i| *i != item)
                .map(|i| i.clone())
                .collect::<Vec<_>>();
            let consequent_set = vec![item.clone()];
            let antecedent_support =
                *frequent_item_sets.get(&antecedent_set).unwrap() as f32 / num_transactions as f32;
            let consequent_support =
                *frequent_item_sets.get(&consequent_set).unwrap() as f32 / num_transactions as f32;

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
