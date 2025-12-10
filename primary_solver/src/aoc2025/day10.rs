use crate::solver::DaySolver;
use std::collections::{HashMap, VecDeque};
use tokio::io;
use z3::{
    Config, Context, Optimize, SatResult,
    ast::{Ast, Int},
};

pub struct Day;

impl DaySolver for Day {
    async fn solve_a(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_a(lines).await
    }

    async fn solve_b(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_b(lines).await
    }

    fn get_day(&self) -> u8 {
        10
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

struct Machine {
    id: u8,
    ind_lights: Vec<bool>,
    buttons: Vec<Vec<u8>>,
    joltage: Vec<u16>,
}
fn print_machine(m: &Machine) {
    println!("Machine ID: {}", m.id);
    println!("Lights: {:?}", m.ind_lights);
    println!("Buttons: {:?}", m.buttons);
    println!("Joltage: {:?}", m.joltage);
}
fn build_machines(input: &Vec<String>) -> Vec<Machine> {
    let mut machines = Vec::new();

    for (i, line) in input.iter().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let light_str = parts[0];
        let ind_lights: Vec<bool> = light_str[1..light_str.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();

        let joltage_idx = parts
            .iter()
            .rposition(|&s| s.contains('{'))
            .expect("joltage set not found");

        let mut buttons = Vec::new();
        for &btn in &parts[1..joltage_idx] {
            if btn.starts_with('(') && btn.ends_with(')') {
                let inner = &btn[1..btn.len() - 1];
                let row: Vec<u8> = inner
                    .split(',')
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect();
                buttons.push(row);
            }
        }

        let joltage_str = parts.last().unwrap();
        let joltage_inner = &joltage_str[1..joltage_str.len() - 1]; // strip { and }
        let joltage: Vec<u16> = joltage_inner
            .split(',')
            .filter_map(|s| s.parse::<u16>().ok())
            .collect();

        machines.push(Machine {
            id: i as u8,
            ind_lights,
            buttons,
            joltage,
        });
    }

    machines
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    let mut result = 0;
    let machines: Vec<Machine> = build_machines(lines);

    fn press_button(state: &mut Vec<bool>, button: &Vec<u8>, _joltage: &Vec<u16>) {
        for &pos in button {
            state[pos as usize] = !state[pos as usize];
        }
    }

    for i in 0..machines.len() {
        let m = &machines[i];
        // print_machine(m);
        let base_state: Vec<bool> = vec![false; m.ind_lights.len()];
        let mut state_map: HashMap<Vec<bool>, u32> = HashMap::new();
        let mut queue: VecDeque<(Vec<bool>, u32)> = VecDeque::new();
        queue.push_back((base_state.clone(), 0));
        while let Some((state, presses)) = queue.pop_front() {
            if let Some(&existing_presses) = state_map.get(&state) {
                if presses >= existing_presses {
                    continue;
                }
            }
            state_map.insert(state.clone(), presses);
            if state == m.ind_lights {
                // println!(
                //     "Machine {} solved in {} presses",
                //     m.id,
                //     presses
                // );
                result += presses;
                break;
            }
            for button in &m.buttons {
                let mut new_state = state.clone();
                press_button(&mut new_state, button, &m.joltage);
                // println!("Pressing button {:?} leads to state {:?}", button, new_state);
                queue.push_back((new_state, presses + 1));
            }
        }
    }

    println!("Result: {}", result);
    Ok(())
}
//1337458128196 too high
//77918 too high
//23062 incorrect
//23552 wrong
//20083
// struct Machine {
//     id: u8,
//     ind_lights: Vec<bool>,
//     buttons: Vec<Vec<u8>>,
//     joltage: Vec<u16>,
// }
pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    let machines = build_machines(lines); // you already have this

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    // One variable per button across ALL machines (we minimise global presses)
    let mut button_vars: HashMap<(u8, usize), Int> = HashMap::new();
    let mut constraints = Vec::new();

    for machine in &machines {
        let n_cols = machine.joltage.len();
        let n_btns = machine.buttons.len();

        // Create fresh Int variables for this machine’s buttons
        let presses: Vec<Int> = (0..n_btns)
            .map(|b| {
                let var = Int::fresh_const(&ctx, &format!("m{}_b{}", machine.id, b));
                button_vars.insert((machine.id, b), var.clone());
                opt.assert(&var.ge(&Int::from_i64(&ctx, 0)));
                var
            })
            .collect();

        // For each column, sum of pressing buttons that affect it == required joltage
        for col in 0..n_cols {
            let mut sum_terms = Vec::new();
            for (b_idx, affected_cols) in machine.buttons.iter().enumerate() {
                if affected_cols.contains(&(col as u8)) {
                    sum_terms.push(&presses[b_idx]);
                }
            }
            let sum = Int::add(&ctx, &sum_terms);
            let target = Int::from_i64(&ctx, machine.joltage[col] as i64);
            constraints.push(sum._eq(&target));
        }
    }

    // Global objective: sum of all button presses
    let global_sum = Int::add(&ctx, &button_vars.values().collect::<Vec<_>>());
    opt.minimize(&global_sum);
    for constr in &constraints {
        opt.assert(constr);
    }
    let result = match opt.check(&[]) {
        SatResult::Sat => opt
            .get_model()
            .and_then(|m| m.eval(&global_sum, true))
            .and_then(|v| v.as_u64())
            .expect("Z3 gave non-u64 result"),
        _ => panic!("UNSAT – no solution exists"),
    };

    let total = result;
    println!("Result: {}", total);
    Ok(())
}

pub async fn solve_b_incorrect_could_work(lines: &Vec<String>) -> io::Result<()> {
    let mut machines = build_machines(lines);
    let mut result: u64 = 0;
    let mut result_list: Vec<u64> = Vec::new();

    fn press_button(state: &mut Vec<u16>, button: &Vec<u8>) -> bool {
        for &pos in button {
            state[pos as usize] += 1;
        }
        true
    }

    for i in 0..machines.len() {
        let m = &mut machines[i];
        println!("joltage for machine {}: {:?}", m.id, m.joltage);
        let n = m.joltage.len();
        println!("n = {}", n);
        let mut optimized_states: Vec<(Vec<u16>, u64)> = Vec::new();
        let mut cycles = 0;

        loop {
            println!("Cycle {}", cycles + 1);
            let mut touches = vec![0u32; n];
            for btn in &m.buttons {
                for &r in btn {
                    let r_idx = r as usize;
                    if r_idx < n {
                        touches[r_idx] += 1;
                    }
                }
            }
            println!("Machine {} touches: {:?}", m.id, touches);

            let reg_buttons_reg_touched: Vec<Vec<bool>> = (0..n)
                .map(|reg| {
                    let mut touched = vec![false; n];
                    for btn in &m.buttons {
                        if btn.contains(&(reg as u8)) {
                            for &r in btn {
                                touched[r as usize] = true;
                            }
                        }
                    }
                    touched
                })
                .collect();

            for reg in 0..n {
                println!(
                    "Machine {} register {} touches registers: {:?}",
                    m.id, reg, reg_buttons_reg_touched[reg]
                );
            }

            let least_button_register_idxs: Vec<u8> = touches
                .iter()
                .enumerate()
                .filter(|&(_i, &count)| count > 0)
                .min_by_key(|&(_i, &count)| count)
                .map(|(i, _count)| i as u8)
                .into_iter()
                .collect();
            println!(
                "Machine {} least button register idxs: {:?}",
                m.id, least_button_register_idxs
            );

            let reg = touches
                .iter()
                .enumerate()
                .filter(|&(_i, &count)| count > 0)
                .min_by_key(|&(_i, &count)| count)
                .map(|(i, _count)| i as u8)
                .unwrap();

            println!("Machine {} chosen register: {}", m.id, reg);
            let relevant_indices: Vec<usize> = m
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(i, btn)| if btn.contains(&reg) { Some(i) } else { None })
                .collect();

            let mut relevant_buttons: Vec<Vec<u8>> = relevant_indices
                .iter()
                .map(|&i| m.buttons[i].clone())
                .collect();

            println!(
                "Machine {} relevant buttons for register {}: {:?}",
                m.id, reg, relevant_buttons
            );

            m.buttons.retain(|btn| !btn.contains(&reg));

            println!(
                "Machine {} remaining buttons after removing relevant: {:?}",
                m.id, m.buttons
            );

            relevant_buttons
                .sort_by_key(|btn| -(btn.iter().filter(|&&r| r == reg).count() as isize));

            let mut touched_registers = vec![false; n];
            for btn in &m.buttons {
                for &r in btn {
                    touched_registers[r as usize] = true;
                }
            }
            println!(
                "Machine {} touched registers after removing buttons: {:?}",
                m.id, touched_registers
            );

            let mut state_map: HashMap<Vec<u16>, u64> = HashMap::new();
            let base_state: Vec<u16> = vec![0; n];

            state_map.insert(base_state.clone(), 0);

            let mut queue: VecDeque<(Vec<u16>, u64)> = VecDeque::new();
            if !optimized_states.is_empty() {
                for (state, count) in &optimized_states {
                    queue.push_back((state.clone(), *count));
                }
                optimized_states.clear();
            } else {
                queue.push_back((base_state.clone(), 0));
            }
            let mut rounds = 0;
            while let Some((state, count)) = queue.pop_front() {
                if state[reg as usize] == m.joltage[reg as usize] as u16 {
                    let mut valid = true;
                    for r in 0..n {
                        if !touched_registers[r] && state[r] != m.joltage[r] {
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        continue;
                    }
                    optimized_states.push((state.clone(), count));
                }
                for button in &relevant_buttons {
                    let mut new_state = state.clone();
                    press_button(&mut new_state, button);
                    let mut exceeds = false;
                    for r in 0..n {
                        if new_state[r] > m.joltage[r] {
                            exceeds = true;
                            break;
                        }
                    }
                    if exceeds {
                        continue;
                    }
                    if let Some(&_existing_count) = state_map.get(&new_state) {
                        continue;
                    }
                    state_map.insert(new_state.clone(), count + 1);
                    let new_count = count + 1;
                    queue.push_back((new_state, new_count));
                }
                rounds += 1;
                if rounds % 1000000 == 0 {
                    println!(
                        "Machine {} rounds processed: {}, queue size: {}, optimized states: {}",
                        m.id,
                        rounds,
                        queue.len(),
                        optimized_states.len()
                    );
                }
            }

            cycles += 1;
            println!(
                "Machine {} optimized states = {}:",
                m.id,
                optimized_states.len()
            );
            if optimized_states.is_empty() {
                println!("No optimized states found, stopping optimization for this machine.");
            }
            if m.buttons.is_empty() {
                break;
            }
        }
        if let Some((_, count)) = optimized_states.iter().min_by_key(|&(_, count)| *count) {
            result += *count as u64;
            result_list.push(*count);
        }
        print!("the follow results are: ");
        for i in 0..result_list.len() {
            let cur = result_list[i];
            print!("i: {} ", cur);
        }
        println!();
        optimized_states.clear();
    }

    println!("Result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Puzzle;
    use crate::get_input_for_puzzle;
    fn get_day_name() -> String {
        let module_path = module_path!();
        let module_name = module_path.split("::").last().unwrap_or("Unknown");
        module_name.to_string().replace("day", "Day")
    }

    fn get_puzzle(part: char) -> Puzzle {
        let day = get_day_name().replace("Day", "").parse::<u8>().unwrap_or(1);
        match part {
            'A' => Puzzle::from_day_part(day, 'A'),
            'B' => Puzzle::from_day_part(day, 'B'),
            _ => panic!("Invalid part"),
        }
    }
}
