/// Extract the characters at the given indices from the input string.
///
/// # Examples
///
/// ```
/// use cha_rs::*;
/// let chars = extract("hello", &[1,5]).unwrap();
/// assert_eq!(chars, &['h', 'o']);
/// ```
pub fn extract(input: &str, indices: &[usize]) -> Result<Vec<char>, &'static str> {
    if indices.is_empty() {
        return Err("must provide at least one index to extract");
    }
    if indices.iter().any(|i| i == &0 || i > &input.len()) {
        return Err("all indices must be within the range of the input");
    }

    let c: Vec<char> = input
        .chars()
        .enumerate()
        .filter(|&(i, _)| indices.contains(&(i + 1)))
        .map(|(_, c)| c)
        .collect();
    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        let got = extract("hello", &[1, 3, 5]).unwrap();
        assert_eq!(got, &['h', 'l', 'o']);
    }

    #[test]
    fn test_extract_out_of_range_low() {
        let got = extract("hello", &[0]);
        assert!(got.is_err())
    }

    #[test]
    fn test_extract_out_of_range_high() {
        let got = extract("hello", &[6]);
        assert!(got.is_err())
    }

    #[test]
    fn test_extract_some_inputs_out_of_range() {
        let got = extract("hello", &[3, 5, 6]);
        assert!(got.is_err())
    }

    #[test]
    fn test_extract_empty_indices() {
        let got = extract("hello", &[]);
        assert!(got.is_err())
    }

    #[test]
    fn test_extract_empty_input() {
        let got = extract("", &[1, 3, 5]);
        assert!(got.is_err())
    }
}
