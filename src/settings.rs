pub struct Settings {
    pub pop_size: u32,
    pub input_size: u32,
    pub output_size: u32,

    pub conn_mut_rate: f64,
    pub node_mut_rate: f64,
    pub wt_mut_rate: f64,
    pub wt_shift_rate: f64,

    pub off_gene_on_rate: f64,
    pub off_in_both_on_rate: f64,
    pub only_mut_rate: f64,

    pub disjoint_coeff: f64,
    pub excess_coeff: f64,
    pub weight_coeff: f64,
    pub speciation_threshold: f64,
    pub allowed_stag_genes: f64,
}
