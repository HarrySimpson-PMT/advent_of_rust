pub trait DaySolver {
    async fn solve_a(&self, lines: &Vec<String>) -> std::io::Result<()>;
    async fn solve_b(&self, lines: &Vec<String>) -> std::io::Result<()>;
    fn get_day(&self) -> u8;
    fn get_year(&self) -> u16;
}