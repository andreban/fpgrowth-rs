use std::collections::HashMap;

use fpgrowth_rs::{fp_growth, generate_association_rules, AssociationRule};

#[test]
fn test_association_rules() {
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

    let num_transactions = transactions.len();
    let mut results = HashMap::new();
    fp_growth(transactions.as_slice(), 7, |item_set, occurences| {
        let mut items = item_set.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        items.sort();
        results.insert(items, occurences);
    });

    let mut association_rules = vec![];
    generate_association_rules(&results, num_transactions, &mut |association_rule| {
        association_rules.push(association_rule);
    });

    assert_eq!(association_rules.len(), 4);

    let potato_chips = association_rules
        .iter()
        .find(|r| r.antecedent_set == vec!["potato chips".to_string()])
        .unwrap();

    assert_eq!(
        potato_chips,
        &AssociationRule::new(
            vec!["potato chips".to_string()],
            vec!["beer".to_string()],
            0.5,
            0.55,
            0.45,
            1.6363635,
            0.9,
        )
    );
}
