use serde::{Serialize, Deserialize};
use serde_json::Value;

// Config.json from 505_mcf_r
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: Value,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "full_system")]
    pub full_system: bool,
    #[serde(rename = "sim_quantum")]
    pub sim_quantum: i64,
    #[serde(rename = "time_sync_enable")]
    pub time_sync_enable: bool,
    #[serde(rename = "time_sync_period")]
    pub time_sync_period: i64,
    #[serde(rename = "time_sync_spin_threshold")]
    pub time_sync_spin_threshold: i64,
    pub system: System,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "auto_unlink_shared_backstore")]
    pub auto_unlink_shared_backstore: bool,
    #[serde(rename = "cache_line_size")]
    pub cache_line_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "exit_on_work_items")]
    pub exit_on_work_items: bool,
    #[serde(rename = "init_param")]
    pub init_param: i64,
    #[serde(rename = "m5ops_base")]
    pub m5ops_base: i64,
    #[serde(rename = "mem_mode")]
    pub mem_mode: String,
    #[serde(rename = "mem_ranges")]
    pub mem_ranges: Vec<String>,
    pub memories: Vec<String>,
    #[serde(rename = "mmap_using_noreserve")]
    pub mmap_using_noreserve: bool,
    #[serde(rename = "multi_thread")]
    pub multi_thread: bool,
    #[serde(rename = "num_work_ids")]
    pub num_work_ids: i64,
    pub readfile: String,
    #[serde(rename = "redirect_paths")]
    pub redirect_paths: Vec<RedirectPath>,
    #[serde(rename = "shadow_rom_ranges")]
    pub shadow_rom_ranges: Vec<Value>,
    #[serde(rename = "shared_backstore")]
    pub shared_backstore: String,
    pub symbolfile: String,
    #[serde(rename = "thermal_components")]
    pub thermal_components: Vec<Value>,
    #[serde(rename = "thermal_model")]
    pub thermal_model: Value,
    #[serde(rename = "work_begin_ckpt_count")]
    pub work_begin_ckpt_count: i64,
    #[serde(rename = "work_begin_cpu_id_exit")]
    pub work_begin_cpu_id_exit: i64,
    #[serde(rename = "work_begin_exit_count")]
    pub work_begin_exit_count: i64,
    #[serde(rename = "work_cpus_ckpt_count")]
    pub work_cpus_ckpt_count: i64,
    #[serde(rename = "work_end_ckpt_count")]
    pub work_end_ckpt_count: i64,
    #[serde(rename = "work_end_exit_count")]
    pub work_end_exit_count: i64,
    #[serde(rename = "work_item_id")]
    pub work_item_id: i64,
    pub workload: Workload,
    #[serde(rename = "clk_domain")]
    pub clk_domain: ClkDomain,
    pub cpu: Vec<Cpu>,
    #[serde(rename = "cpu_clk_domain")]
    pub cpu_clk_domain: CpuClkDomain,
    #[serde(rename = "cpu_voltage_domain")]
    pub cpu_voltage_domain: CpuVoltageDomain,
    #[serde(rename = "dvfs_handler")]
    pub dvfs_handler: DvfsHandler,
    pub l2: L2,
    #[serde(rename = "mem_ctrls")]
    pub mem_ctrls: Vec<MemCtrl>,
    pub membus: Membus,
    pub tol2bus: Tol2bus,
    #[serde(rename = "voltage_domain")]
    pub voltage_domain: VoltageDomain,
    #[serde(rename = "system_port")]
    pub system_port: SystemPort,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedirectPath {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "app_path")]
    pub app_path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "host_paths")]
    pub host_paths: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workload {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "remote_gdb_port")]
    pub remote_gdb_port: i64,
    #[serde(rename = "wait_for_remote_gdb")]
    pub wait_for_remote_gdb: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClkDomain {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub clock: Vec<i64>,
    #[serde(rename = "domain_id")]
    pub domain_id: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "init_perf_level")]
    pub init_perf_level: i64,
    #[serde(rename = "voltage_domain")]
    pub voltage_domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "LFSTSize")]
    pub lfstsize: i64,
    #[serde(rename = "LQEntries")]
    pub lqentries: i64,
    #[serde(rename = "LSQCheckLoads")]
    pub lsqcheck_loads: bool,
    #[serde(rename = "LSQDepCheckShift")]
    pub lsqdep_check_shift: i64,
    #[serde(rename = "SQEntries")]
    pub sqentries: i64,
    #[serde(rename = "SSITSize")]
    pub ssitsize: i64,
    pub activity: i64,
    pub back_com_size: i64,
    pub branch_pred: BranchPred,
    pub cache_load_ports: i64,
    pub cache_store_ports: i64,
    pub checker: Value,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    pub commit_to_decode_delay: i64,
    pub commit_to_fetch_delay: i64,
    #[serde(rename = "commitToIEWDelay")]
    pub commit_to_iewdelay: i64,
    pub commit_to_rename_delay: i64,
    pub commit_width: i64,
    #[serde(rename = "cpu_id")]
    pub cpu_id: i64,
    pub decode_to_fetch_delay: i64,
    pub decode_to_rename_delay: i64,
    pub decode_width: i64,
    pub decoder: Vec<Decoder>,
    pub dispatch_width: i64,
    #[serde(rename = "do_checkpoint_insts")]
    pub do_checkpoint_insts: bool,
    #[serde(rename = "do_statistics_insts")]
    pub do_statistics_insts: bool,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub fetch_buffer_size: i64,
    pub fetch_queue_size: i64,
    pub fetch_to_decode_delay: i64,
    pub fetch_trap_latency: i64,
    pub fetch_width: i64,
    pub forward_com_size: i64,
    pub fu_pool: FuPool,
    #[serde(rename = "function_trace")]
    pub function_trace: bool,
    #[serde(rename = "function_trace_start")]
    pub function_trace_start: i64,
    pub iew_to_commit_delay: i64,
    pub iew_to_decode_delay: i64,
    pub iew_to_fetch_delay: i64,
    pub iew_to_rename_delay: i64,
    pub interrupts: Vec<Interrupt>,
    pub isa: Vec<Isa>,
    pub issue_to_execute_delay: i64,
    pub issue_width: i64,
    #[serde(rename = "max_insts_all_threads")]
    pub max_insts_all_threads: i64,
    #[serde(rename = "max_insts_any_thread")]
    pub max_insts_any_thread: i64,
    pub mmu: Mmu,
    #[serde(rename = "needsTSO")]
    pub needs_tso: bool,
    #[serde(rename = "numIQEntries")]
    pub num_iqentries: i64,
    #[serde(rename = "numPhysCCRegs")]
    pub num_phys_ccregs: i64,
    pub num_phys_float_regs: i64,
    pub num_phys_int_regs: i64,
    pub num_phys_vec_pred_regs: i64,
    pub num_phys_vec_regs: i64,
    #[serde(rename = "numROBEntries")]
    pub num_robentries: i64,
    pub num_robs: i64,
    pub num_threads: i64,
    #[serde(rename = "power_gating_on_idle")]
    pub power_gating_on_idle: bool,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState3,
    #[serde(rename = "progress_interval")]
    pub progress_interval: i64,
    #[serde(rename = "pwr_gating_latency")]
    pub pwr_gating_latency: i64,
    pub rename_to_decode_delay: i64,
    pub rename_to_fetch_delay: i64,
    #[serde(rename = "renameToIEWDelay")]
    pub rename_to_iewdelay: i64,
    #[serde(rename = "renameToROBDelay")]
    pub rename_to_robdelay: i64,
    pub rename_width: i64,
    #[serde(rename = "simpoint_start_insts")]
    pub simpoint_start_insts: Vec<Value>,
    pub smt_commit_policy: String,
    pub smt_fetch_policy: String,
    #[serde(rename = "smtIQPolicy")]
    pub smt_iqpolicy: String,
    #[serde(rename = "smtIQThreshold")]
    pub smt_iqthreshold: i64,
    #[serde(rename = "smtLSQPolicy")]
    pub smt_lsqpolicy: String,
    #[serde(rename = "smtLSQThreshold")]
    pub smt_lsqthreshold: i64,
    pub smt_num_fetching_threads: i64,
    #[serde(rename = "smtROBPolicy")]
    pub smt_robpolicy: String,
    #[serde(rename = "smtROBThreshold")]
    pub smt_robthreshold: i64,
    #[serde(rename = "socket_id")]
    pub socket_id: i64,
    pub squash_width: i64,
    #[serde(rename = "store_set_clear_period")]
    pub store_set_clear_period: i64,
    #[serde(rename = "switched_out")]
    pub switched_out: bool,
    pub syscall_retry_latency: i64,
    pub system: String,
    pub tracer: Tracer,
    pub trap_latency: i64,
    pub wb_width: i64,
    pub workload: Vec<Workload2>,
    pub dcache: Dcache,
    #[serde(rename = "dtb_walker_cache")]
    pub dtb_walker_cache: DtbWalkerCache,
    pub icache: Icache,
    #[serde(rename = "itb_walker_cache")]
    pub itb_walker_cache: ItbWalkerCache,
    #[serde(rename = "dcache_port")]
    pub dcache_port: DcachePort,
    #[serde(rename = "icache_port")]
    pub icache_port: IcachePort,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchPred {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "BTBEntries")]
    pub btbentries: i64,
    #[serde(rename = "BTBTagSize")]
    pub btbtag_size: i64,
    #[serde(rename = "RASSize")]
    pub rassize: i64,
    pub choice_ctr_bits: i64,
    pub choice_predictor_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub global_ctr_bits: i64,
    pub global_predictor_size: i64,
    pub indirect_branch_pred: IndirectBranchPred,
    pub inst_shift_amt: i64,
    pub local_ctr_bits: i64,
    pub local_history_table_size: i64,
    pub local_predictor_size: i64,
    pub num_threads: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectBranchPred {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "indirectGHRBits")]
    pub indirect_ghrbits: i64,
    #[serde(rename = "indirectHashGHR")]
    pub indirect_hash_ghr: bool,
    pub indirect_hash_targets: bool,
    pub indirect_path_length: i64,
    pub indirect_sets: i64,
    pub indirect_tag_size: i64,
    pub indirect_ways: i64,
    pub inst_shift_amt: i64,
    pub num_threads: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decoder {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub isa: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuPool {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "FUList")]
    pub fulist: Vec<Fulist>,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fulist {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub count: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub op_list: Vec<OpList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpList {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub op_class: String,
    pub op_lat: i64,
    pub pipelined: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interrupt {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_domain")]
    pub clk_domain: ClkDomain2,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "int_latency")]
    pub int_latency: i64,
    #[serde(rename = "pio_latency")]
    pub pio_latency: i64,
    pub system: String,
    #[serde(rename = "int_requestor")]
    pub int_requestor: IntRequestor,
    #[serde(rename = "int_responder")]
    pub int_responder: IntResponder,
    pub pio: Pio,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClkDomain2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_divider")]
    pub clk_divider: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntRequestor {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntResponder {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pio {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Isa {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "vendor_string")]
    pub vendor_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mmu {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub dtb: Dtb,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub itb: Itb,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dtb {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "entry_type")]
    pub entry_type: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "next_level")]
    pub next_level: Value,
    pub size: i64,
    pub system: String,
    pub walker: Walker,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Walker {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "num_squash_per_cycle")]
    pub num_squash_per_cycle: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState,
    pub system: String,
    pub port: Port,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Itb {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "entry_type")]
    pub entry_type: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "next_level")]
    pub next_level: Value,
    pub size: i64,
    pub system: String,
    pub walker: Walker2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Walker2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "num_squash_per_cycle")]
    pub num_squash_per_cycle: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState2,
    pub system: String,
    pub port: Port2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port2 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState3 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracer {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workload2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub cmd: Vec<String>,
    pub cwd: String,
    pub drivers: Vec<Value>,
    pub egid: i64,
    pub env: Vec<Value>,
    pub errout: String,
    pub euid: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub executable: String,
    pub gid: i64,
    pub input: String,
    #[serde(rename = "kvmInSE")]
    pub kvm_in_se: bool,
    pub max_stack_size: i64,
    pub output: String,
    pub pgid: i64,
    pub pid: i64,
    pub ppid: i64,
    pub release: String,
    pub simpoint: i64,
    pub system: String,
    pub uid: i64,
    #[serde(rename = "useArchPT")]
    pub use_arch_pt: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dcache {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "addr_ranges")]
    pub addr_ranges: Vec<String>,
    pub assoc: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    pub clusivity: String,
    pub compressor: Value,
    #[serde(rename = "data_latency")]
    pub data_latency: i64,
    #[serde(rename = "demand_mshr_reserve")]
    pub demand_mshr_reserve: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "is_read_only")]
    pub is_read_only: bool,
    #[serde(rename = "max_miss_count")]
    pub max_miss_count: i64,
    #[serde(rename = "move_contractions")]
    pub move_contractions: bool,
    pub mshrs: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState4,
    #[serde(rename = "prefetch_on_access")]
    pub prefetch_on_access: bool,
    #[serde(rename = "prefetch_on_pf_hit")]
    pub prefetch_on_pf_hit: bool,
    pub prefetcher: Value,
    #[serde(rename = "replace_expansions")]
    pub replace_expansions: bool,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: ReplacementPolicy,
    #[serde(rename = "response_latency")]
    pub response_latency: i64,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    pub tags: Tags,
    #[serde(rename = "tgts_per_mshr")]
    pub tgts_per_mshr: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
    #[serde(rename = "write_allocator")]
    pub write_allocator: Value,
    #[serde(rename = "write_buffers")]
    pub write_buffers: i64,
    #[serde(rename = "writeback_clean")]
    pub writeback_clean: bool,
    #[serde(rename = "cpu_side")]
    pub cpu_side: CpuSide,
    #[serde(rename = "mem_side")]
    pub mem_side: MemSide,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState4 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementPolicy {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "block_size")]
    pub block_size: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "indexing_policy")]
    pub indexing_policy: IndexingPolicy,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState5,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: String,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState5 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSide {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemSide {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DtbWalkerCache {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "addr_ranges")]
    pub addr_ranges: Vec<String>,
    pub assoc: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    pub clusivity: String,
    pub compressor: Value,
    #[serde(rename = "data_latency")]
    pub data_latency: i64,
    #[serde(rename = "demand_mshr_reserve")]
    pub demand_mshr_reserve: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "is_read_only")]
    pub is_read_only: bool,
    #[serde(rename = "max_miss_count")]
    pub max_miss_count: i64,
    #[serde(rename = "move_contractions")]
    pub move_contractions: bool,
    pub mshrs: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState6,
    #[serde(rename = "prefetch_on_access")]
    pub prefetch_on_access: bool,
    #[serde(rename = "prefetch_on_pf_hit")]
    pub prefetch_on_pf_hit: bool,
    pub prefetcher: Value,
    #[serde(rename = "replace_expansions")]
    pub replace_expansions: bool,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: ReplacementPolicy2,
    #[serde(rename = "response_latency")]
    pub response_latency: i64,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    pub tags: Tags2,
    #[serde(rename = "tgts_per_mshr")]
    pub tgts_per_mshr: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
    #[serde(rename = "write_allocator")]
    pub write_allocator: Value,
    #[serde(rename = "write_buffers")]
    pub write_buffers: i64,
    #[serde(rename = "writeback_clean")]
    pub writeback_clean: bool,
    #[serde(rename = "cpu_side")]
    pub cpu_side: CpuSide2,
    #[serde(rename = "mem_side")]
    pub mem_side: MemSide2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState6 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementPolicy2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "block_size")]
    pub block_size: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "indexing_policy")]
    pub indexing_policy: IndexingPolicy2,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState7,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: String,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState7 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSide2 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemSide2 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icache {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "addr_ranges")]
    pub addr_ranges: Vec<String>,
    pub assoc: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    pub clusivity: String,
    pub compressor: Value,
    #[serde(rename = "data_latency")]
    pub data_latency: i64,
    #[serde(rename = "demand_mshr_reserve")]
    pub demand_mshr_reserve: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "is_read_only")]
    pub is_read_only: bool,
    #[serde(rename = "max_miss_count")]
    pub max_miss_count: i64,
    #[serde(rename = "move_contractions")]
    pub move_contractions: bool,
    pub mshrs: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState8,
    #[serde(rename = "prefetch_on_access")]
    pub prefetch_on_access: bool,
    #[serde(rename = "prefetch_on_pf_hit")]
    pub prefetch_on_pf_hit: bool,
    pub prefetcher: Value,
    #[serde(rename = "replace_expansions")]
    pub replace_expansions: bool,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: ReplacementPolicy3,
    #[serde(rename = "response_latency")]
    pub response_latency: i64,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    pub tags: Tags3,
    #[serde(rename = "tgts_per_mshr")]
    pub tgts_per_mshr: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
    #[serde(rename = "write_allocator")]
    pub write_allocator: Value,
    #[serde(rename = "write_buffers")]
    pub write_buffers: i64,
    #[serde(rename = "writeback_clean")]
    pub writeback_clean: bool,
    #[serde(rename = "cpu_side")]
    pub cpu_side: CpuSide3,
    #[serde(rename = "mem_side")]
    pub mem_side: MemSide3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState8 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementPolicy3 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags3 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "block_size")]
    pub block_size: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "indexing_policy")]
    pub indexing_policy: IndexingPolicy3,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState9,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: String,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy3 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState9 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSide3 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemSide3 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItbWalkerCache {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "addr_ranges")]
    pub addr_ranges: Vec<String>,
    pub assoc: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    pub clusivity: String,
    pub compressor: Value,
    #[serde(rename = "data_latency")]
    pub data_latency: i64,
    #[serde(rename = "demand_mshr_reserve")]
    pub demand_mshr_reserve: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "is_read_only")]
    pub is_read_only: bool,
    #[serde(rename = "max_miss_count")]
    pub max_miss_count: i64,
    #[serde(rename = "move_contractions")]
    pub move_contractions: bool,
    pub mshrs: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState10,
    #[serde(rename = "prefetch_on_access")]
    pub prefetch_on_access: bool,
    #[serde(rename = "prefetch_on_pf_hit")]
    pub prefetch_on_pf_hit: bool,
    pub prefetcher: Value,
    #[serde(rename = "replace_expansions")]
    pub replace_expansions: bool,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: ReplacementPolicy4,
    #[serde(rename = "response_latency")]
    pub response_latency: i64,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    pub tags: Tags4,
    #[serde(rename = "tgts_per_mshr")]
    pub tgts_per_mshr: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
    #[serde(rename = "write_allocator")]
    pub write_allocator: Value,
    #[serde(rename = "write_buffers")]
    pub write_buffers: i64,
    #[serde(rename = "writeback_clean")]
    pub writeback_clean: bool,
    #[serde(rename = "cpu_side")]
    pub cpu_side: CpuSide4,
    #[serde(rename = "mem_side")]
    pub mem_side: MemSide4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState10 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementPolicy4 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags4 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "block_size")]
    pub block_size: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "indexing_policy")]
    pub indexing_policy: IndexingPolicy4,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState11,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: String,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy4 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState11 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSide4 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemSide4 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcachePort {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IcachePort {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuClkDomain {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub clock: Vec<i64>,
    #[serde(rename = "domain_id")]
    pub domain_id: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "init_perf_level")]
    pub init_perf_level: i64,
    #[serde(rename = "voltage_domain")]
    pub voltage_domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuVoltageDomain {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub voltage: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DvfsHandler {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub domains: Vec<Value>,
    pub enable: bool,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "sys_clk_domain")]
    pub sys_clk_domain: String,
    #[serde(rename = "transition_latency")]
    pub transition_latency: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "addr_ranges")]
    pub addr_ranges: Vec<String>,
    pub assoc: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    pub clusivity: String,
    pub compressor: Value,
    #[serde(rename = "data_latency")]
    pub data_latency: i64,
    #[serde(rename = "demand_mshr_reserve")]
    pub demand_mshr_reserve: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "is_read_only")]
    pub is_read_only: bool,
    #[serde(rename = "max_miss_count")]
    pub max_miss_count: i64,
    #[serde(rename = "move_contractions")]
    pub move_contractions: bool,
    pub mshrs: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState12,
    #[serde(rename = "prefetch_on_access")]
    pub prefetch_on_access: bool,
    #[serde(rename = "prefetch_on_pf_hit")]
    pub prefetch_on_pf_hit: bool,
    pub prefetcher: Value,
    #[serde(rename = "replace_expansions")]
    pub replace_expansions: bool,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: ReplacementPolicy5,
    #[serde(rename = "response_latency")]
    pub response_latency: i64,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    pub tags: Tags5,
    #[serde(rename = "tgts_per_mshr")]
    pub tgts_per_mshr: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
    #[serde(rename = "write_allocator")]
    pub write_allocator: Value,
    #[serde(rename = "write_buffers")]
    pub write_buffers: i64,
    #[serde(rename = "writeback_clean")]
    pub writeback_clean: bool,
    #[serde(rename = "cpu_side")]
    pub cpu_side: CpuSide5,
    #[serde(rename = "mem_side")]
    pub mem_side: MemSide5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState12 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementPolicy5 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags5 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "block_size")]
    pub block_size: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "indexing_policy")]
    pub indexing_policy: IndexingPolicy5,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState13,
    #[serde(rename = "replacement_policy")]
    pub replacement_policy: String,
    #[serde(rename = "sequential_access")]
    pub sequential_access: bool,
    pub size: i64,
    pub system: String,
    #[serde(rename = "tag_latency")]
    pub tag_latency: i64,
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy5 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    pub assoc: i64,
    #[serde(rename = "entry_size")]
    pub entry_size: i64,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState13 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSide5 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemSide5 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemCtrl {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "command_window")]
    pub command_window: i64,
    #[serde(rename = "disable_sanity_check")]
    pub disable_sanity_check: bool,
    pub dram: Dram,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "mem_sched_policy")]
    pub mem_sched_policy: String,
    #[serde(rename = "min_reads_per_switch")]
    pub min_reads_per_switch: i64,
    #[serde(rename = "min_writes_per_switch")]
    pub min_writes_per_switch: i64,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState15,
    #[serde(rename = "qos_policy")]
    pub qos_policy: Value,
    #[serde(rename = "qos_priorities")]
    pub qos_priorities: i64,
    #[serde(rename = "qos_priority_escalation")]
    pub qos_priority_escalation: bool,
    #[serde(rename = "qos_q_policy")]
    pub qos_q_policy: String,
    #[serde(rename = "qos_requestors")]
    pub qos_requestors: Vec<String>,
    #[serde(rename = "qos_syncro_scheduler")]
    pub qos_syncro_scheduler: bool,
    #[serde(rename = "qos_turnaround_policy")]
    pub qos_turnaround_policy: Value,
    #[serde(rename = "static_backend_latency")]
    pub static_backend_latency: i64,
    #[serde(rename = "static_frontend_latency")]
    pub static_frontend_latency: i64,
    pub system: String,
    #[serde(rename = "write_high_thresh_perc")]
    pub write_high_thresh_perc: i64,
    #[serde(rename = "write_low_thresh_perc")]
    pub write_low_thresh_perc: i64,
    pub port: Port3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dram {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "IDD0")]
    pub idd0: f64,
    #[serde(rename = "IDD02")]
    pub idd02: f64,
    #[serde(rename = "IDD2N")]
    pub idd2n: f64,
    #[serde(rename = "IDD2N2")]
    pub idd2n2: f64,
    #[serde(rename = "IDD2P0")]
    pub idd2p0: f64,
    #[serde(rename = "IDD2P02")]
    pub idd2p02: f64,
    #[serde(rename = "IDD2P1")]
    pub idd2p1: f64,
    #[serde(rename = "IDD2P12")]
    pub idd2p12: f64,
    #[serde(rename = "IDD3N")]
    pub idd3n: f64,
    #[serde(rename = "IDD3N2")]
    pub idd3n2: f64,
    #[serde(rename = "IDD3P0")]
    pub idd3p0: f64,
    #[serde(rename = "IDD3P02")]
    pub idd3p02: f64,
    #[serde(rename = "IDD3P1")]
    pub idd3p1: f64,
    #[serde(rename = "IDD3P12")]
    pub idd3p12: f64,
    #[serde(rename = "IDD4R")]
    pub idd4r: f64,
    #[serde(rename = "IDD4R2")]
    pub idd4r2: f64,
    #[serde(rename = "IDD4W")]
    pub idd4w: f64,
    #[serde(rename = "IDD4W2")]
    pub idd4w2: f64,
    #[serde(rename = "IDD5")]
    pub idd5: f64,
    #[serde(rename = "IDD52")]
    pub idd52: f64,
    #[serde(rename = "IDD6")]
    pub idd6: f64,
    #[serde(rename = "IDD62")]
    pub idd62: f64,
    #[serde(rename = "VDD")]
    pub vdd: f64,
    #[serde(rename = "VDD2")]
    pub vdd2: f64,
    #[serde(rename = "activation_limit")]
    pub activation_limit: i64,
    #[serde(rename = "addr_mapping")]
    pub addr_mapping: String,
    #[serde(rename = "bank_groups_per_rank")]
    pub bank_groups_per_rank: i64,
    #[serde(rename = "banks_per_rank")]
    pub banks_per_rank: i64,
    #[serde(rename = "beats_per_clock")]
    pub beats_per_clock: i64,
    #[serde(rename = "burst_length")]
    pub burst_length: i64,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "conf_table_reported")]
    pub conf_table_reported: bool,
    #[serde(rename = "data_clock_sync")]
    pub data_clock_sync: bool,
    #[serde(rename = "device_bus_width")]
    pub device_bus_width: i64,
    #[serde(rename = "device_rowbuffer_size")]
    pub device_rowbuffer_size: i64,
    #[serde(rename = "device_size")]
    pub device_size: i64,
    #[serde(rename = "devices_per_rank")]
    pub devices_per_rank: i64,
    pub dll: bool,
    #[serde(rename = "enable_dram_powerdown")]
    pub enable_dram_powerdown: bool,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "image_file")]
    pub image_file: String,
    #[serde(rename = "in_addr_map")]
    pub in_addr_map: bool,
    #[serde(rename = "kvm_map")]
    pub kvm_map: bool,
    #[serde(rename = "max_accesses_per_row")]
    pub max_accesses_per_row: i64,
    pub null: bool,
    #[serde(rename = "page_policy")]
    pub page_policy: String,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState14,
    pub range: String,
    #[serde(rename = "ranks_per_channel")]
    pub ranks_per_channel: i64,
    #[serde(rename = "read_buffer_size")]
    pub read_buffer_size: i64,
    #[serde(rename = "tAAD")]
    pub t_aad: i64,
    #[serde(rename = "tBURST")]
    pub t_burst: i64,
    #[serde(rename = "tBURST_MAX")]
    pub t_burst_max: i64,
    #[serde(rename = "tBURST_MIN")]
    pub t_burst_min: i64,
    #[serde(rename = "tCCD_L")]
    pub t_ccd_l: i64,
    #[serde(rename = "tCCD_L_WR")]
    pub t_ccd_l_wr: i64,
    #[serde(rename = "tCK")]
    pub t_ck: i64,
    #[serde(rename = "tCL")]
    pub t_cl: i64,
    #[serde(rename = "tCS")]
    pub t_cs: i64,
    #[serde(rename = "tCWL")]
    pub t_cwl: i64,
    #[serde(rename = "tPPD")]
    pub t_ppd: i64,
    #[serde(rename = "tRAS")]
    pub t_ras: i64,
    #[serde(rename = "tRCD")]
    pub t_rcd: i64,
    #[serde(rename = "tRCD_WR")]
    pub t_rcd_wr: i64,
    #[serde(rename = "tREFI")]
    pub t_refi: i64,
    #[serde(rename = "tRFC")]
    pub t_rfc: i64,
    #[serde(rename = "tRP")]
    pub t_rp: i64,
    #[serde(rename = "tRRD")]
    pub t_rrd: i64,
    #[serde(rename = "tRRD_L")]
    pub t_rrd_l: i64,
    #[serde(rename = "tRTP")]
    pub t_rtp: i64,
    #[serde(rename = "tRTW")]
    pub t_rtw: i64,
    #[serde(rename = "tWR")]
    pub t_wr: i64,
    #[serde(rename = "tWTR")]
    pub t_wtr: i64,
    #[serde(rename = "tWTR_L")]
    pub t_wtr_l: i64,
    #[serde(rename = "tXAW")]
    pub t_xaw: i64,
    #[serde(rename = "tXP")]
    pub t_xp: i64,
    #[serde(rename = "tXPDLL")]
    pub t_xpdll: i64,
    #[serde(rename = "tXS")]
    pub t_xs: i64,
    #[serde(rename = "tXSDLL")]
    pub t_xsdll: i64,
    #[serde(rename = "two_cycle_activate")]
    pub two_cycle_activate: bool,
    #[serde(rename = "write_buffer_size")]
    pub write_buffer_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState14 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState15 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port3 {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membus {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "forward_latency")]
    pub forward_latency: i64,
    #[serde(rename = "frontend_latency")]
    pub frontend_latency: i64,
    #[serde(rename = "header_latency")]
    pub header_latency: i64,
    #[serde(rename = "max_outstanding_snoops")]
    pub max_outstanding_snoops: i64,
    #[serde(rename = "max_routing_table_size")]
    pub max_routing_table_size: i64,
    #[serde(rename = "point_of_coherency")]
    pub point_of_coherency: bool,
    #[serde(rename = "point_of_unification")]
    pub point_of_unification: bool,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState16,
    #[serde(rename = "response_latency")]
    pub response_latency: i64,
    #[serde(rename = "snoop_filter")]
    pub snoop_filter: SnoopFilter,
    #[serde(rename = "snoop_response_latency")]
    pub snoop_response_latency: i64,
    pub system: String,
    #[serde(rename = "use_default_range")]
    pub use_default_range: bool,
    pub width: i64,
    #[serde(rename = "cpu_side_ports")]
    pub cpu_side_ports: CpuSidePorts,
    #[serde(rename = "mem_side_ports")]
    pub mem_side_ports: MemSidePorts,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState16 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnoopFilter {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "lookup_latency")]
    pub lookup_latency: i64,
    #[serde(rename = "max_capacity")]
    pub max_capacity: i64,
    pub system: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSidePorts {
    pub role: String,
    pub peer: Vec<String>,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemSidePorts {
    pub role: String,
    pub peer: Vec<String>,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tol2bus {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_domain")]
    pub clk_domain: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "forward_latency")]
    pub forward_latency: i64,
    #[serde(rename = "frontend_latency")]
    pub frontend_latency: i64,
    #[serde(rename = "header_latency")]
    pub header_latency: i64,
    #[serde(rename = "max_outstanding_snoops")]
    pub max_outstanding_snoops: i64,
    #[serde(rename = "max_routing_table_size")]
    pub max_routing_table_size: i64,
    #[serde(rename = "point_of_coherency")]
    pub point_of_coherency: bool,
    #[serde(rename = "point_of_unification")]
    pub point_of_unification: bool,
    #[serde(rename = "power_model")]
    pub power_model: Vec<Value>,
    #[serde(rename = "power_state")]
    pub power_state: PowerState17,
    #[serde(rename = "response_latency")]
    pub response_latency: i64,
    #[serde(rename = "snoop_filter")]
    pub snoop_filter: SnoopFilter2,
    #[serde(rename = "snoop_response_latency")]
    pub snoop_response_latency: i64,
    pub system: String,
    #[serde(rename = "use_default_range")]
    pub use_default_range: bool,
    pub width: i64,
    #[serde(rename = "cpu_side_ports")]
    pub cpu_side_ports: CpuSidePorts2,
    #[serde(rename = "mem_side_ports")]
    pub mem_side_ports: MemSidePorts2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerState17 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "clk_gate_bins")]
    pub clk_gate_bins: i64,
    #[serde(rename = "clk_gate_max")]
    pub clk_gate_max: i64,
    #[serde(rename = "clk_gate_min")]
    pub clk_gate_min: i64,
    #[serde(rename = "default_state")]
    pub default_state: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub leaders: Vec<Value>,
    #[serde(rename = "possible_states")]
    pub possible_states: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnoopFilter2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    #[serde(rename = "lookup_latency")]
    pub lookup_latency: i64,
    #[serde(rename = "max_capacity")]
    pub max_capacity: i64,
    pub system: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSidePorts2 {
    pub role: String,
    pub peer: Vec<String>,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemSidePorts2 {
    pub role: String,
    pub peer: Vec<String>,
    #[serde(rename = "is_source")]
    pub is_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoltageDomain {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "cxx_class")]
    pub cxx_class: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "eventq_index")]
    pub eventq_index: i64,
    pub voltage: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemPort {
    pub role: String,
    pub peer: String,
    #[serde(rename = "is_source")]
    pub is_source: String,
}
