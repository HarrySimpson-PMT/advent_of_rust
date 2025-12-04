//define solve_a and solve_b functions for day01 as traits
pub trait DaySolver {
    async fn solve_a(&self, lines: &Vec<String>) -> std::io::Result<()>;
    async fn solve_b(&self, lines: &Vec<String>) -> std::io::Result<()>;
    fn get_day(&self) -> u32;
    fn get_year(&self) -> u32;
}