// There are 10^9 lamports in one SOL
const LAMPORTS_PER_SOL: u64 = 100_000_000;

// There are 10^6 micro-lamports in one lamport
const MICRO_LAMPORTS_PER_LAMPORT: u64 = 1_000_000;

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_SOL as f64) as u64
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_SOL as f64
}

pub fn lamports_to_micro_lamports(lamports: u64) -> u64 {
    lamports * MICRO_LAMPORTS_PER_LAMPORT
}

pub fn micro_lamports_to_lamports(micro_lamports: u64) -> u64 {
    micro_lamports / MICRO_LAMPORTS_PER_LAMPORT
}

pub fn sol_to_micro_lamports(sol: f64) -> u64 {
    lamports_to_micro_lamports(sol_to_lamports(sol))
}

pub fn micro_lamports_to_sol(micro_lamports: u64) -> f64 {
    lamports_to_sol(micro_lamports_to_lamports(micro_lamports))
}