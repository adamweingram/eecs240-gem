use std::process::Command;
use std::path::Path;
use regex::Regex;

/// Prints the command to be run in a console-friendly format
/// 
/// # Arguments
/// * `cmd` - Command to print as a `Command` struct
macro_rules! print_command {
    ($cmd:expr) => {{
        let mut cmd_str = format!("{}", $cmd.get_program().to_string_lossy());
        for arg in $cmd.get_args() {
            cmd_str.push_str(&format!(" {}", arg.to_string_lossy()));
        }
        println!("[DEBUG] Command: {}", cmd_str);
    }};
}

/// Runs the 505.mcf_r benchmark
/// 
/// # Arguments
/// * `bench_root_path` - Path to the root of the benchmarks directory
/// * `gem5_root_path` - Path to the root of the gem5 directory
/// * `gem5_exe_path` - Path to the gem5 executable
/// * `bench_specific_dir` - Path to the 505.mcf_r benchmark directory
/// * `se_path` - Path to the gem5 se.py script
/// * `output_dir_path` - Path to the output directory
/// * `options_path` - Path to the benchmark options file
/// * `num_instructions` - Number of instructions to run the benchmark for
/// * `cmd_path` - Path to the benchmark executable
/// * `l1d_size` - Size of the L1 data cache in kilobytes (default: 32)
/// * `l1i_size` - Size of the L1 instruction cache in kilobytes (default: 32)
/// * `l1d_assoc` - Associativity of the L1 data cache (default: 4)
/// * `l1i_assoc` - Associativity of the L1 instruction cache (default: 4)
/// * `l2_assoc` - Associativity of the L2 cache (default: 1)
pub fn run_505_mcf_r_benchmark(bench_root_path: &Path, gem5_root_path: &Path, gem5_exe_path: &Path, bench_specific_dir: &Path, se_path: &Path, 
    output_dir_path: &Path, options_path: &Path, num_instructions: u64, cmd_path: &Path, l1d_size: u64, l1i_size: u64, l1d_assoc: u64, 
    l1i_assoc: u64, l2_assoc: u64) {

    // [DEBUG] Print args
    println!("[DEBUG] Will use args: output_dir_path={}, num_instructions={}, cmd_path={}", 
        output_dir_path.to_str().unwrap(), num_instructions, cmd_path.to_str().unwrap());

    // Run benchmark
    let mut command = Command::new(gem5_exe_path.to_str().unwrap());

    command
        .args(["-d", output_dir_path.to_str().unwrap(),
            se_path.to_str().unwrap(),
                "--cmd", cmd_path.to_str().unwrap(),
                "--options", options_path.to_str().unwrap(),
                "--maxinsts", num_instructions.to_string().as_str(),
                "--cpu-type=O3CPU",
                "--caches",
                    "--l2cache",
                    format!("--l1d_size={}kB", l1d_size).as_str(),
                    format!("--l1i_size={}kB", l1i_size).as_str(),
                    "--l2_size=512kB",
                    format!("--l1d_assoc={}", l1d_assoc).as_str(),
                    format!("--l1i_assoc={}", l1i_assoc).as_str(),
                    format!("--l2_assoc={}", l2_assoc).as_str(),
                    "--cacheline_size=64"]);

    // Print command
    print_command!(command);

    // Spawn/run the command, get the handle
    let mut command_handle = command.spawn().expect("Could not run 505.mc_r benchmarks!");

    // Await command's completion
    command_handle.wait().unwrap();
}

pub fn run_519_lbm_r_benchmark(
    bench_root_path: &Path, 
    gem5_root_path: &Path, 
    gem5_exe_path: &Path, 
    bench_specific_dir: &Path, 
    se_path: &Path,
    output_dir_path: &Path, 
    num_instructions: u64, 
    cmd_path: &Path, 
    l1d_size: u64, 
    l1i_size: u64, 
    l1d_assoc: u64, 
    l1i_assoc: u64, 
    l2_assoc: u64) {

    // [DEBUG] Print args
    println!("[DEBUG] Will use args: output_dir_path={}, num_instructions={}, cmd_path={}", 
    output_dir_path.to_str().unwrap(), num_instructions, cmd_path.to_str().unwrap());

    // Run benchmark
    let mut command = Command::new(gem5_exe_path.to_str().unwrap());

    command
        .args(["-d", output_dir_path.to_str().unwrap(), 
            se_path.to_str().unwrap(),
                // "--cmd", "../run/lbm_r_base.eecs240-m64", 
                "--cmd", cmd_path.to_str().unwrap(),
                "--options", "3000 reference.dat 0 0 100_100_130_ldc.of", 
                "--maxinsts", num_instructions.to_string().as_str(),
                "--cpu-type=O3CPU",
                "--caches",
                    "--l2cache",
                    format!("--l1d_size={}kB", l1d_size).as_str(),
                    format!("--l1i_size={}kB", l1i_size).as_str(),
                    "--l2_size=512kB",
                    format!("--l1d_assoc={}", l1d_assoc).as_str(),
                    format!("--l1i_assoc={}", l1i_assoc).as_str(),
                    format!("--l2_assoc={}", l2_assoc).as_str(),
                    "--cacheline_size=64"])
        .spawn()
        .expect("Could not run 519.lbm_r benchmarks!");

    // Print command
    print_command!(command);

    // Spawn/run the command, get the handle
    let mut command_handle = command.spawn().expect("Could not run 519_lbm_r benchmarks!");

    // Await command's completion
    command_handle.wait().unwrap();
}

#[derive(Debug)]
pub struct StatsData {
    pub dcache_cpu_data_misses: u64,
    pub dcache_overall_misses: u64,
    pub dcache_replacements: u64,
    pub dcache_compulsory_misses: u64,
    pub dcache_capacity_misses: u64,
    pub dcache_conflict_misses: u64,
    pub ipc: f64,
}

/// Parses the stats.txt file generated by gem5
/// 
/// # Arguments
/// * `stats_string` - String containing the contents of the stats.txt file
/// 
/// # Returns
/// * `StatsData` - Struct containing the parsed data
pub fn parse_stats(stats_string: &str) -> StatsData {
    /*
     * In the gem5 stats.txt file, compulsory cache misses are not directly reported. However, you can calculate them by subtracting 
     * the sum of capacity and conflict misses from the total number of misses. The specific entries for these values may vary 
     * depending on the configuration of the simulation, but they might be something like:
     * 
     * - system.cpu.dcache.overall_misses::total (total number of misses)
     * - system.cpu.dcache.overall_mshr_misses::total (capacity misses)
     * - system.cpu.dcache.overall_conflicts::total (conflict misses)
     */

    // Create regular expressions
    // let re_every_line = Regex::new(r"^([A-z0-9\.\:]+) +([0-9]+) +(.*)").unwrap();
    let re_cpu_data_misses = Regex::new(r"(?m)^(system\.cpu\.dcache\.overallMisses::cpu\.data) +([0-9]+) +# (.*)$").unwrap();
    let re_overall_misses = Regex::new(r"(?m)^(system\.cpu\.dcache\.overallMisses::total) +([0-9]+) +# (.*)$").unwrap();
    let re_replacements = Regex::new(r"(?m)^(system\.cpu\.dcache\.replacements) +([0-9]+) +# (.*)$").unwrap();
    let re_ipc = Regex::new(r"(?m)^(system\.cpu\.ipc) +([0-9\.]+) +# (.*)$").unwrap();

    // Find line with dcache cpu data misses
    let mut cpu_data_misses_list = vec![];
    for (_, [key, value, explanation]) in re_cpu_data_misses.captures_iter(stats_string).map(|c| c.extract()) {
        // println!("[DEBUG] Got match: key={}, value={}, explanation={}", key, value, explanation);
        cpu_data_misses_list.push((key.to_string(), value.parse::<u64>().unwrap(), explanation.to_string()));
    }

    // Find line with dcache overall misses
    let mut overall_misses_list = vec![];
    for (_, [key, value, explanation]) in re_overall_misses.captures_iter(stats_string).map(|c| c.extract()) {
        // println!("[DEBUG] Got match: key={}, value={}, explanation={}", key, value, explanation);
        overall_misses_list.push((key.to_string(), value.parse::<u64>().unwrap(), explanation.to_string()));
    }

    // Find line with dcache replacements
    let mut replacements_list = vec![];
    for (_, [key, value, explanation]) in re_replacements.captures_iter(stats_string).map(|c| c.extract()) {
        println!("[DEBUG] Got match: key={}, value={}, explanation={}", key, value, explanation);
        replacements_list.push((key.to_string(), value.parse::<u64>().unwrap(), explanation.to_string()));
    }

    // Find line with ipc 
    let mut ipc_list = vec![];
    for (_, [key, value, explanation]) in re_ipc.captures_iter(stats_string).map(|c| c.extract()) {
        println!("[DEBUG] Got match: key={}, value={}, explanation={}", key, value, explanation);
        ipc_list.push((key.to_string(), value.parse::<f64>().unwrap(), explanation.to_string()));
    }

    // Get values from the lists
    // Note: We'll only use the first of each hit (THERE SHOULD BE EXACTLY ONE OF EACH)
    assert!(cpu_data_misses_list.len() == 1);
    assert!(overall_misses_list.len() == 1);
    assert!(replacements_list.len() == 1);
    assert!(ipc_list.len() == 1);
    let cpu_data_misses = cpu_data_misses_list.get(0).unwrap().1;
    let overall_misses = overall_misses_list.get(0).unwrap().1;
    let replacements = replacements_list.get(0).unwrap().1;
    let ipc = ipc_list.get(0).unwrap().1;

    // Calculate derivative cache stats
    let compulsory_misses: u64 = overall_misses - replacements;
    let capacity_misses: u64 = 999999999999999999;  // TODO: Calculate this
    let conflict_misses: u64 = 999999999999999999;  // TODO: Calculate this

    // Build the output `StatsData` struct
    let output_struct = StatsData {
        dcache_cpu_data_misses: cpu_data_misses,
        dcache_overall_misses: overall_misses,
        dcache_replacements: replacements,
        dcache_compulsory_misses: compulsory_misses,
        dcache_capacity_misses: capacity_misses,
        dcache_conflict_misses: conflict_misses,
        ipc: ipc,
    };

    return output_struct;
}
