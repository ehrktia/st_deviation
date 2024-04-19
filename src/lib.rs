// std_deviation calculates std deviation for input data set
// in vector format using the formula sqrt((a-b)^2/N)
// where a - element in data set
// b - mean value of data set
// N - total number of elements in data set
pub fn std_deviation(input: &Vec<f64>) -> Option<f64> {
    let mean = mean(input)?;
    // TODO: expensive operation optimise
    let input_clone = input.clone();
    #[allow(unused_variables)]
    let mean_squared: Vec<f64> = input_clone.iter().map(|a| (a - mean).exp2()).collect();
    let mut total = 0.0;
    for value in mean_squared.iter() {
        total += value;
    }
    let avg_value = total / input_clone.len() as f64;
    Some(avg_value.sqrt())
}
// mean calculates mean value of incoming vector data set
pub fn mean(input: &Vec<f64>) -> Option<f64> {
    let mut total = 0.0;
    for value in input.iter() {
        total += value;
    }
    let input_length: f64 = input.len() as f64;
    Some(total / input_length)
}
// median calculates median for provided data set in vector format
pub fn median(input: &Vec<f64>) -> Option<f64> {
    // TODO: expensive operation optimise
    let mut input_clone = input.clone();
    let input_length = input_clone.len();
    let mid_element = input_length % 2;
    input_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Some(input_clone[mid_element])
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    #[test]
    fn test_std_deviation() {
        let test_input = vec![1.0, 2.0, 3.0];
        let result = match std_deviation(&test_input){
            Some(r) => r,
            None => panic!("error calcuating std deviation"),
        };
        
        assert_eq!(f64::trunc(result * 100f64).floor() / 100f64, 1.08);
    }

    #[test]
    fn test_mean() {
        let test_input = vec![1.0, 2.0, 3.0];
        let result = mean(&test_input);
        assert_eq!(result, Some(2.0));
    }
    #[test]
    fn test_median() {
        let test_input = vec![3.0, 2.0, 1.0];
        let result = median(&test_input);
        assert_eq!(result, Some(2.0));
    }
}
