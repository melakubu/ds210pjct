pub struct Statistics {
    data: Vec<u32>,
}

impl Statistics {
    // Constructor
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // Adding a path length 
    pub fn add_length(&mut self, length: u32) {
        self.data.push(length);
    }

    // Calculating the mean, median, and standard deviation of shortest lengths between pairs of nodes 
    pub fn compute(&self) -> (f64, u32, f64) {
        let mean = self.data.iter().map(|&val| val as f64).sum::<f64>() / self.data.len() as f64; // Dividing the sum of the shortest lengths by the num of nodes
        let mut sorted = self.data.clone(); // Sorting for the median calc
        sorted.sort_unstable();
        let median = if sorted.len() % 2 == 0 { // If the num of points is even, we get the avg of the two middle nums 
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2
        } else {
            sorted[sorted.len() / 2] // Otherwise we get the middle num
        };
        let variance = sorted.iter().map(|&val| { // Calc the variance
            let diff = val as f64 - mean;
            diff * diff
        }).sum::<f64>() / sorted.len() as f64;
        let std_deviation = variance.sqrt(); // Then square is for the SD
        (mean, median, std_deviation) // Then return back all three values
    }
}
