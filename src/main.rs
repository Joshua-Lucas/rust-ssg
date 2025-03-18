use std::ops::Index;

pub mod nodes;

fn main() {
    // Use this to spring borad to split a value bas
    let test = vec!["Hello **world** this __is__ a test"];

    let delm = ["**", "__"];

    let split = delm.into_iter().fold(test, |acc, d| {
        println!("{:?}", acc);
        acc.iter()
            .flat_map(|&x| x.split(d).collect::<Vec<_>>())
            .collect()
    });

    // JOSH use this to expand with the text node
    let manual_test = vec!["Hello **world** this __is__ a **test**"];

    let another_test = manual_test
        .iter()
        .map(|x| {
            let delm = ["**", "__"];

            delm.iter()
                .map(|y| {
                    let indx = &x.match_indices(y).collect::<Vec<_>>();

                    let mut indx_groups = Vec::new();
                    for i in (0..indx.len()).step_by(2) {
                        indx_groups.push((indx[i], indx[i + 1]))
                    }

                    let test = indx_groups
                        .iter()
                        .map(|z| {
                            let start = z.0 .0;
                            let end = z.1 .0;
                            let split = &x[start..end];
                            let new_node = split.split(z.0 .1).collect::<Vec<_>>();

                            let remaining_nodes = x.split(split).collect::<Vec<_>>();
                            (new_node[1], remaining_nodes)
                        })
                        .collect::<Vec<_>>();

                    println!("{:?}", test);
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // println!("{:?}", split);
    // println!("{:?}", another_test);
}
