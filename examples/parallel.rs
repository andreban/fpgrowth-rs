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

use fpgrowth_rs::parallel_fp_growth;

fn main() {
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

    parallel_fp_growth(transactions.as_slice(), 3, |item_set, occurences| {
        println!("{:?}: {}", item_set, occurences)
    });
}
