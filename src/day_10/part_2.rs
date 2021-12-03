use crate::utils::read_file;

fn calculate_permutations(_input: &[u32]) -> u64 {
    // let mut copy: Vec<u32> = vec![0];
    // copy.append(&mut input.iter().copied().collect());

    // copy.sort_unstable();

    // copy.push(input.iter().max().unwrap() + 3);

    // copy.reverse();

    // let mut _index_edges: HashMap<usize, u32> = HashMap::new();

    // for slice in copy.windows(3) {
    //     // slice has a length of 3

    //     match slice.get(0).unwrap() {
    //         1 => 1,
    //         3 =>
    //     }
    // }

    0
}

// https://adventofcode.com/2020/day/9
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_10/input.txt".into())?;
    let _input: Vec<u32> = split.iter().map(|s| s.parse::<u32>().unwrap()).collect();

    let _permutations = calculate_permutations(&Vec::new());
    Err("We need to find a better way to loop through it all".into())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn outcome() {
        // assert_eq!( 1820), find_solution().unwrap());
    }

    #[test]
    fn test_example_0() {
        // let input: Vec<u32> = vec!["1", "2", "3", "5", "6"]
        //     .iter()
        //     .map(|s| s.parse::<u32>().unwrap())
        //     .collect();

        // let permutations = calculate_permutations(&input);

        // assert_eq!(8, permutations);
    }
    #[test]
    fn test_example_1() {
        // let input: Vec<u32> = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"]
        //     .iter()
        //     .map(|s| s.parse::<u32>().unwrap())
        //     .collect();

        // let permutations = calculate_permutations(&input);

        // assert_eq!(8, permutations);
    }

    #[test]
    fn test_example_2() {
        // let input: Vec<u32> = vec![
        //     "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
        //     "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
        //     "10", "3",
        // ]
        // .iter()
        // .map(|s| s.parse::<u32>().unwrap())
        // .collect();

        // let permutations = calculate_permutations(&input);

        // assert_eq!(19208, permutations);
    }
}
