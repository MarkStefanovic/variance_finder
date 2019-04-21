extern crate itertools;
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use itertools::Itertools;
use rayon::prelude::*;


#[pymodule]
/// Module containing utility function for finding accounting variances
fn variance_finder(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(find_matches))?;
    module.add_wrapped(wrap_pyfunction!(find_multiple_matches))?;
    Ok(())
}

#[pyfunction]
/// Given a list of floats, find combinations of items that comprise a total, and return their indices.
///
/// # Arguments
///
/// * `items` - List of floats to match against
/// * `total` - Total to match against
/// * `fuzz` - Consider matches +/- this amount
/// * `max_iterations` - int value indicating the max iterations to try when searching for matches
/// * `max_matches`- int value indicating that the search should cease once this number of matches are found
fn find_matches(
    items: Vec<f32>,
    total: f32,
    fuzz: f32,
    max_iterations: u32,
    max_matches: usize,
) -> PyResult<Vec<Vec<usize>>> {
    let true_fuzz: f32;
    if fuzz.abs() < 0.0001 {
        true_fuzz = 0.0001;
    } else {
        true_fuzz = fuzz.abs();
    }
    let clean_items =
        items
            .iter()
            .enumerate()
            .map(|(ix, f)| (ix, *f))
            .filter(|(_ix, f)| *f != 0.0)
            .collect_vec();
    let mut matches: Vec<Vec<(usize, f32)>> = vec![];
    let mut iterations: u32 = 0;
    for n in 1..clean_items.len() {
        for combo in clean_items.iter().combinations(n) {
            iterations += 1;
            if iterations >= max_iterations {
                return Ok(extract_indices(matches));
            }
            let c =
                combo
                    .iter()
                    .map(|(ix, f)| (*ix, *f))
                    .collect_vec();
            let combo_total =
                c.iter().map(|(_ix, f)| *f).sum();
            if total - true_fuzz <= combo_total && combo_total <= total + true_fuzz {
                matches.push(c);
            }
            if matches.len() >= max_matches {
                return Ok(extract_indices(matches));
            }
        }
    }
    Ok(extract_indices(matches))
}

#[pyfunction]
/// Given a list of floats, and a list of totals to find, find combinations of items that match each total, and return their indices.
///
/// This function runs in parallel for each total, so it is significantly faster than running
/// find_matches multiple times for multiple totals.
///
/// # Arguments
///
/// * `items` - List of floats to match against
/// * `totals` - List of totals to match against
/// * `fuzz` - Consider matches +/- this amount
/// * `max_iterations` - int value indicating the max iterations to try when searching for matches
/// * `max_matches`- int value indicating that the search should cease once this number of matches are found
fn find_multiple_matches(
    items: Vec<f32>,
    totals: Vec<f32>,
    fuzz: f32,
    max_iterations: u32,
    max_matches: usize,
) -> PyResult<Vec<Vec<Vec<usize>>>> {
    let results: Vec<Vec<Vec<usize>>> =
        totals.par_iter()
            .map(|&total| find_matches(items.clone(), total, fuzz, max_iterations, max_matches).unwrap())
            .collect();
    Ok(results)
}


fn extract_indices(matches: Vec<Vec<(usize, f32)>>) -> Vec<Vec<usize>> {
    matches
        .iter()
        .map(|v| v.iter().map(|(ix, _f)| *ix).collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let items: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let actual = find_matches(items, 42.0, 0.1, 1_000_000, 5);
        let expected: Vec<Vec<usize>> = vec![
            vec![42], vec![1, 41], vec![2, 40], vec![3, 39], vec![4, 38]
        ];
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_find_multiple_matches() {
        let items: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let totals: Vec<f32> = vec![42.1, -5.2, 1.3];
        let actual = find_multiple_matches(items, totals, 1.4, 1_000_000, 5);
        let expected: Vec<Vec<Vec<usize>>> = vec![
            vec![vec![41], vec![42], vec![43], vec![1, 40], vec![1, 41]],
            vec![],
            vec![vec![1], vec![2]]
        ];
        assert_eq!(expected, actual.unwrap());
    }
}