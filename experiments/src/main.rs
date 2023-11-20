use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;
use std::fs;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use rusqlite::{Connection, params};

mod schemas;
mod experiment_util;

fn main() {
    println!("[INFO] Starting experiments...");

    // // Test benchmark command
    // {
    //     // Paths
    //     // let experiments_root_path = Path::new("/root/experiments");
    //     let bench_root_path = Path::new("/root/bench");
    //     let gem5_root_path = Path::new("/root/gem5");
    //     let gem5_exe_path = gem5_root_path.join("build/X86/gem5.opt");
    //     let bench_specific_dir = bench_root_path.join("505.mcf_r");
    //     let se_path = gem5_root_path.join("configs/example/se.py");
    //     let output_dir_path = bench_specific_dir.join("m5out-temp-testrun");
    //     let options_path = bench_specific_dir.join("data/inp.in");

    //     // Benchmark args
    //     let num_instructions = 100000000;
    //     let cmd_path = bench_specific_dir.join(Path::new("run/mcf_r_base.eecs240-m64"));
    //     let l1d_size = 32;  // In kilobytes
    //     let l1i_size = 32;  // in kilobytes

    //     // [DEBUG] Print args
    //     println!("[DEBUG] Will use args: output_dir_path={}, num_instructions={}, cmd_path={}", 
    //         output_dir_path.to_str().unwrap(), num_instructions, cmd_path.to_str().unwrap());

    //     // Run benchmark
    //     experiment_util::run_505_mcf_r_benchmark(&bench_root_path, &gem5_root_path, &gem5_exe_path, &bench_specific_dir, 
    //         &se_path, &output_dir_path, &options_path, num_instructions, &cmd_path, l1d_size, l1i_size);
    // }
    
    /// Experiment type enum
    /// 
    /// # Fields
    /// * `MCF` - 505.mcf_r
    /// * `LBM` - 519.lbm_r
    #[derive(Debug, Clone)]
    enum ExperimentType {
        MCF,
        LBM,
    }

    /// Experiment rider struct that represents an experiment instance
    /// 
    /// # Fields
    /// * `iteration_id` - ID of the iteration of the experiment
    /// * `complete` - Whether or not the experiment is complete
    #[derive(Debug, Clone)]
    struct ExperimentRider {
        // Experiment information
        iteration_id: u64,
        experiment_type: ExperimentType,
        setting_tag: Option<String>,
        complete: bool,
        output_dir_path: Option<PathBuf>,

        // Benchmark args
        num_instructions: u64,
        l1d_size: u64,
        l1i_size: u64,
        l1d_assoc: u64,
        l1i_assoc: u64,
        l2_assoc: u64,
    }

    // Create a list of experiment riders that will correspond to each experiment instance
    // Note: This is where we generate the parameters for the experiments as well
    let mut experiment_riders: Vec<ExperimentRider> = Vec::new();  // Vector of experiment "riders"
    // let size_max_iter: u64 = 10;
    // let assoc_max_iter: u64 = 10;
    // for i in 0..size_max_iter {
    //     for j in 0..assoc_max_iter {
    //         // Calculate parameters
    //         let l1d_size = 2u64.pow(i as u32 + 1);
    //         let l1i_size = 2u64.pow(i as u32 + 1);
    //         let l1d_sets = 2u64.pow(j as u32);
    //         let l1i_sets = 2u64.pow(j as u32);

    //         // Generate a rider with required parameters
    //         experiment_riders.push(ExperimentRider {
    //             iteration_id: i*assoc_max_iter + j,
    //             setting_tag: Some("optimize_ipc".to_string()),
    //             complete: false,
    //             output_dir_path: None,
    //             // num_instructions: 100_000_000,
    //             num_instructions: 1_000_000,
    //             l1d_size: l1d_size,
    //             l1i_size: l1i_size,
    //             l1d_assoc: l1d_sets,
    //             l1i_assoc: l1i_sets,
    //             l2_assoc: 1u64,
    //         });
    //         println!("[DEBUG] Generated rider with params: {:?}", experiment_riders.last().unwrap());
    //     }
    // }
    {
        let mut overall_iter_counter = 0;  // Track iteration across all types of experiments

        let this_experiment_type = ExperimentType::MCF;

        // Add default for comparison
        for i in vec![1] {
            // Generate a rider with required parameters
            experiment_riders.push(ExperimentRider {
                iteration_id: overall_iter_counter,
                experiment_type: this_experiment_type.clone(),
                setting_tag: Some("default".to_string()),
                complete: false,
                output_dir_path: None,
                num_instructions: 1_000_000,
                l1d_size: 32,
                l1i_size: 32,
                l1d_assoc: 4,
                l1i_assoc: 4,
                l2_assoc: 1,
            });
            overall_iter_counter += 1;
            println!("[DEBUG] Generated rider with params: {:?}", experiment_riders.last().unwrap());
        }

        // // Add Compulsory Miss experiments
        // // Note: We try multiple sizes to figure out the minimum size that will cause _ONLY_ compulsory misses
        // for i in vec![2, 32, 64, 128, 256, 512, 1024] {
        //     // Generate a rider with required parameters
        //     experiment_riders.push(ExperimentRider {
        //         iteration_id: overall_iter_counter,
        //         experiment_type: this_experiment_type.clone(),
        //         setting_tag: Some("compulsory".to_string()),
        //         complete: false,
        //         output_dir_path: None,
        //         num_instructions: 1_000_000,
        //         l1d_size: i,
        //         l1i_size: 256,
        //         l1d_assoc: 4,
        //         l1i_assoc: 4,
        //         l2_assoc: 1,
        //     });
        //     overall_iter_counter += 1;
        //     println!("[DEBUG] Generated rider with params: {:?}", experiment_riders.last().unwrap());
        // }

        // // Add Capacity Miss experiments
        // for i in vec![1] {
        //     // Generate a rider with required parameters
        //     experiment_riders.push(ExperimentRider {
        //         iteration_id: overall_iter_counter,
        //         experiment_type: this_experiment_type.clone(),
        //         setting_tag: Some("capacity".to_string()),
        //         complete: false,
        //         output_dir_path: None,
        //         num_instructions: 1_000_000,
        //         l1d_size: 2,    // Appears to be the max size we can use without a crash
        //         l1i_size: 32,
        //         l1d_assoc: 32,  // Appears to be the max associativity we can use without a crash
        //         l1i_assoc: 4,
        //         l2_assoc: 1,
        //     });
        //     overall_iter_counter += 1;
        //     println!("[DEBUG] Generated rider with params: {:?}", experiment_riders.last().unwrap());
        // }

        // // Add Conflict Miss experiments
        // for i in vec![1] {
        //     // Generate a rider with required parameters
        //     experiment_riders.push(ExperimentRider {
        //         iteration_id: overall_iter_counter,
        //         experiment_type: this_experiment_type.clone(),
        //         setting_tag: Some("conflict".to_string()),
        //         complete: false,
        //         output_dir_path: None,
        //         num_instructions: 1_000_000,
        //         l1d_size: 32,
        //         l1i_size: 32,
        //         l1d_assoc: 1,
        //         l1i_assoc: 4,
        //         l2_assoc: 1,
        //     });
        //     overall_iter_counter += 1;
        //     println!("[DEBUG] Generated rider with params: {:?}", experiment_riders.last().unwrap());
        // }

        // Experiment to get best IPC
        for i in vec![1, 2, 4, 8, 16, 32] {
            // Generate a rider with required parameters
            experiment_riders.push(ExperimentRider {
                iteration_id: overall_iter_counter,
                experiment_type: this_experiment_type.clone(),
                setting_tag: Some("optimize_ipc".to_string()),
                complete: false,
                output_dir_path: None,
                num_instructions: 1_000_000,
                l1d_size: 16,
                l1i_size: 32,
                l1d_assoc: i,
                l1i_assoc: 4,
                l2_assoc: 1,
            });
            overall_iter_counter += 1;
            println!("[DEBUG] Generated rider with params: {:?}", experiment_riders.last().unwrap());
        }
    }

    // Try running experiments
    experiment_riders = experiment_riders.into_par_iter().map(|rider| {
        // Paths
        // let experiments_root_path = Path::new("/root/experiments");
        let bench_root_path = Path::new("/root/bench");
        let gem5_root_path = Path::new("/root/gem5");
        let gem5_exe_path = gem5_root_path.join("build/X86/gem5.opt");
        let bench_specific_dir = bench_root_path.join(match rider.experiment_type {
            ExperimentType::MCF => "505.mcf_r",
            ExperimentType::LBM => "519.lbm_r",
        });
        let se_path = gem5_root_path.join("configs/example/se.py");
        let output_dir_path = bench_specific_dir.join(format!("m5out-temp-{}", rider.iteration_id).as_str());
        let options_path = bench_specific_dir.join("data/inp.in");

        // Benchmark args
        let num_instructions = rider.num_instructions;
        let cmd_path = match rider.experiment_type {
            ExperimentType::MCF => bench_specific_dir.join(Path::new("run/mcf_r_base.eecs240-m64")),
            ExperimentType::LBM => bench_specific_dir.join(Path::new("run/lbm_r_base.eecs240-m64")),
        };

        // [DEBUG] Print args
        println!("[DEBUG] Will use args: output_dir_path={}, num_instructions={}, cmd_path={}", 
        output_dir_path.to_str().unwrap(), num_instructions, cmd_path.to_str().unwrap());

        // First check if the experiment has already been run
        if output_dir_path.exists() {
            println!("[INFO] Experiment iteration {} has already been run! Skipping... (See: {:?})", rider.iteration_id, output_dir_path);
        } 
        
        else {
            match rider.experiment_type {
                ExperimentType::MCF => {
                    // Run benchmark
                    experiment_util::run_505_mcf_r_benchmark(
                        &bench_root_path, 
                        &gem5_root_path, 
                        &gem5_exe_path, 
                        &bench_specific_dir, 
                        &se_path, 
                        &output_dir_path, 
                        &options_path, 
                        num_instructions, 
                        &cmd_path, 
                        rider.l1d_size, 
                        rider.l1i_size, 
                        rider.l1d_assoc, 
                        rider.l1i_assoc, 
                        rider.l2_assoc
                    );
                },
                ExperimentType::LBM => {
                    // Run benchmark
                    experiment_util::run_519_lbm_r_benchmark(
                        &bench_root_path, 
                        &gem5_root_path, 
                        &gem5_exe_path, 
                        &bench_specific_dir, 
                        &se_path, 
                        &output_dir_path, 
                        num_instructions,
                        &cmd_path,
                        rider.l1d_size, 
                        rider.l1i_size, 
                        rider.l1d_assoc, 
                        rider.l1i_assoc, 
                        rider.l2_assoc
                    );
                }
            }
        }

        // "Mark" experiment as complete
        let mut output_rider = rider.clone();
        output_rider.complete = true;
        output_rider.output_dir_path = Some(output_dir_path);
        return output_rider;
    }).collect();

    // [DEBUG] Log finished actually running experiments
    println!("[INFO] Finished running experiments, will now attempt to store...");

    // Open connection to the db
    let db_path = Path::new("/root/experiments.db");
    // let conn = Connection::open_in_memory().unwrap();
    let conn = Connection::open(db_path).unwrap();

    // Create schema, table
    #[derive(Debug)]
    struct Experiment {
        id: i32,
        exp_type: String,
        setting_tag: Option<String>,
        num_instructions: u64,
        l1d_size: u64,
        l1i_size: u64,
        l1d_assoc: u64,
        l1i_assoc: u64,
        l2_assoc: u64,
        dcache_mshrs: i64,
        dcache_max_miss_count: u64,
        dtbwc_mshrs: i64,
        dtbwc_max_miss_count: u64,
        icache_mshrs: i64,
        icache_max_miss_count: u64,
        itbwc_msdhrs: i64,
        itbwc_max_miss_count: u64,
        l2_mshrs: i64,
        l2_max_miss_count: u64,
        dcache_overall_misses: u64,
        dcache_replacements: u64,
        ipc: f64,
    }
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS experiments (
            id                     INTEGER PRIMARY KEY,
            exp_type               TEXT NOT NULL,
            setting_tag            TEXT,
            num_instructions       INTEGER NOT NULL,
            l1d_size               INTEGER NOT NULL,
            l1i_size               INTEGER NOT NULL,
            l1d_assoc              INTEGER NOT NULL,
            l1i_assoc              INTEGER NOT NULL,
            l2_assoc               INTEGER NOT NULL,
            dcache_mshrs           INTEGER NOT NULL,
            dcache_max_miss_count  INTEGER NOT NULL,
            dtbwc_mshrs            INTEGER NOT NULL,
            dtbwc_max_miss_count   INTEGER NOT NULL,
            icache_mshrs           INTEGER NOT NULL,
            icache_max_miss_count  INTEGER NOT NULL,
            itbwc_msdhrs           INTEGER NOT NULL,
            itbwc_max_miss_count   INTEGER NOT NULL,
            l2_mshrs               INTEGER NOT NULL,
            l2_max_miss_count      INTEGER NOT NULL,
            dcache_overall_misses  INTEGER NOT NULL,
            dcache_replacements    INTEGER NOT NULL,
            ipc                    REAL NOT NULL,
            full_data              TEXT
        )",
        (), // empty list of parameters.
    ).unwrap();

    // Parse output
    for rider in experiment_riders {
        // Verify experiment is complete
        assert!(rider.complete, "[ERROR] Experiment iteration {} is not complete!", rider.iteration_id);

        // Get paths of output
        // let output_path = Path::new("/root/bench/505.mcf_r/m5out-temp-0/config.json");
        let config_output_path = match rider.output_dir_path.clone() {
            Some(val) => val.join("config.json"),
            None => panic!("Could not get config output path!"),
        };
        let stats_output_path = match rider.output_dir_path.clone() {
            Some(val) => val.join("stats.txt"),
            None => panic!("Could not get stats output path!"),
        };

        // Read output
        let config_file_contents = fs::read_to_string(config_output_path.clone()).unwrap();
        let stats_file_contents = fs::read_to_string(stats_output_path.clone()).unwrap();

        // Parse the string of config data into serde_json::Value.
        // println!("[DEBUG] Parsing stats file: {:?}", stats_output_path);
        let config_parsed: schemas::Root = serde_json::from_str(config_file_contents.as_str()).unwrap();

        // Parse the string of stats data into a struct
        let stats_parsed = experiment_util::parse_stats(stats_file_contents.as_str());

        // Get configuration data, cache config information
        let num_instructions = config_parsed.system.cpu[0].max_insts_any_thread;
        let l1d_size = config_parsed.system.cpu[0].dcache.size;
        let l1i_size = config_parsed.system.cpu[0].icache.size;
        let l1d_assoc = config_parsed.system.cpu[0].dcache.assoc;
        let l1i_assoc = config_parsed.system.cpu[0].icache.assoc;
        let l2_assoc = config_parsed.system.l2.assoc;

        let dcache_mshrs = config_parsed.system.cpu[0].dcache.mshrs;
        let dcache_max_miss_count = config_parsed.system.cpu[0].dcache.max_miss_count;
        let dtbwc_mshrs = config_parsed.system.cpu[0].dtb_walker_cache.mshrs;
        let dtbwc_max_miss_count = config_parsed.system.cpu[0].dtb_walker_cache.max_miss_count;
        let icache_mshrs = config_parsed.system.cpu[0].icache.mshrs;
        let icache_max_miss_count = config_parsed.system.cpu[0].icache.max_miss_count;
        let itbwc_msdhrs = config_parsed.system.cpu[0].itb_walker_cache.mshrs;
        let itbwc_max_miss_count = config_parsed.system.cpu[0].itb_walker_cache.max_miss_count;
        let l2_mshrs = config_parsed.system.l2.mshrs;
        let l2_max_miss_count = config_parsed.system.l2.max_miss_count;

        // Get stats data
        let dcache_cpu_misses = stats_parsed.dcache_cpu_data_misses;
        let dcache_overall_misses = stats_parsed.dcache_overall_misses;
        let dcache_replacements = stats_parsed.dcache_replacements;
        let ipc = stats_parsed.ipc;

        // // [DEBUG] Print out the data from the experiment
        // println!("[DEBUG] dcache_mshrs 1: {}, dtbwc_mshrs 2: {}, icache_mshrs: {}, itbwc_msdhrs: {}, l2_mshrs: {}", 
        //     dcache_mshrs, dtbwc_mshrs, icache_mshrs, itbwc_msdhrs, l2_mshrs);

        // Insert data into the database
        conn.execute(
            r#"
                INSERT INTO 
                    experiments 
                    (
                        exp_type, 
                        setting_tag,
                        num_instructions, 
                        l1d_size, 
                        l1i_size, 
                        l1d_assoc, 
                        l1i_assoc, 
                        l2_assoc, 
                        dcache_mshrs, 
                        dcache_max_miss_count, 
                        dtbwc_mshrs, 
                        dtbwc_max_miss_count, 
                        icache_mshrs, 
                        icache_max_miss_count, 
                        itbwc_msdhrs, 
                        itbwc_max_miss_count, 
                        l2_mshrs, 
                        l2_max_miss_count, 
                        dcache_overall_misses,
                        dcache_replacements,
                        ipc,
                        full_data
                    ) 
                VALUES 
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)
                "#,
            params![
                match rider.experiment_type {
                    ExperimentType::MCF => "505.mcf_r",
                    ExperimentType::LBM => "519.lbm_r",
                },
                match rider.setting_tag {
                    Some(val) => val,
                    None => "none".to_string(),
                },
                &num_instructions,
                &l1d_size,
                &l1i_size,
                &l1d_assoc,
                &l1i_assoc,
                &l2_assoc,
                &dcache_mshrs,
                &dcache_max_miss_count,
                &dtbwc_mshrs,
                &dtbwc_max_miss_count,
                &icache_mshrs,
                &icache_max_miss_count,
                &itbwc_msdhrs,
                &itbwc_max_miss_count,
                &l2_mshrs,
                &l2_max_miss_count, 
                &dcache_overall_misses,
                &dcache_replacements,
                &ipc,
                &config_file_contents,
            ]
        ).unwrap();
    }

    // Print out data from the experiments that were inserted into the database
    let mut stmt = conn.prepare(r#"
        SELECT 
            id, 
            exp_type, 
            setting_tag,
            num_instructions, 
            l1d_size, 
            l1i_size, 
            l1d_assoc, 
            l1i_assoc, 
            l2_assoc, 
            dcache_mshrs, 
            dcache_max_miss_count,
            dtbwc_mshrs, 
            dtbwc_max_miss_count,
            icache_mshrs, 
            icache_max_miss_count,
            itbwc_msdhrs, 
            itbwc_max_miss_count,
            l2_mshrs,
            l2_max_miss_count,
            dcache_overall_misses,
            dcache_replacements,
            ipc
        FROM 
            experiments
    "#).unwrap();
    let experiments_iter = stmt.query_map([], |row| {
        Ok(Experiment {
            id: row.get(0).unwrap(),
            exp_type: row.get(1).unwrap(),
            setting_tag: row.get(2).unwrap(),
            num_instructions: row.get(3).unwrap(),
            l1d_size: row.get(4).unwrap(),
            l1i_size: row.get(5).unwrap(),
            l1d_assoc: row.get(6).unwrap(),
            l1i_assoc: row.get(7).unwrap(),
            l2_assoc: row.get(8).unwrap(),
            dcache_mshrs: row.get(9).unwrap(),
            dcache_max_miss_count: row.get(10).unwrap(),
            dtbwc_mshrs: row.get(11).unwrap(),
            dtbwc_max_miss_count: row.get(12).unwrap(),
            icache_mshrs: row.get(13).unwrap(),
            icache_max_miss_count: row.get(14).unwrap(),
            itbwc_msdhrs: row.get(15).unwrap(),
            itbwc_max_miss_count: row.get(16).unwrap(),
            l2_mshrs: row.get(17).unwrap(),
            l2_max_miss_count: row.get(18).unwrap(),
            dcache_overall_misses: row.get(19).unwrap(),
            dcache_replacements: row.get(20).unwrap(),
            ipc: row.get(21).unwrap(),
        })
    }).unwrap();

    for experiment in experiments_iter {
        println!("[DEBUG] Found experiment: {:?}", experiment.unwrap());
    }

    println!("[INFO] Finished experiments!");
}
