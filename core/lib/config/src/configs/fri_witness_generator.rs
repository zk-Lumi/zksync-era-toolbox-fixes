use std::time::Duration;

// Built-in uses
// External uses
use serde::Deserialize;

/// Configuration for the fri witness generation
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct FriWitnessGeneratorConfig {
    /// Max time for witness to be generated
    pub generation_timeout_in_secs: u16,
    pub basic_generation_timeout_in_secs: Option<u16>,
    pub leaf_generation_timeout_in_secs: Option<u16>,
    pub scheduler_generation_timeout_in_secs: Option<u16>,
    pub node_generation_timeout_in_secs: Option<u16>,
    /// Max attempts for generating witness
    pub max_attempts: u32,
    // Percentage of the blocks that gets proven in the range [0.0, 1.0]
    // when 0.0 implies all blocks are skipped and 1.0 implies all blocks are proven.
    pub blocks_proving_percentage: Option<u8>,
    pub dump_arguments_for_blocks: Vec<u32>,
    // Optional l1 batch number to process block until(inclusive).
    // This parameter is used in case of performing circuit upgrades(VK/Setup keys),
    // to not let witness-generator pick new job and finish all the existing jobs with old circuit.
    pub last_l1_batch_to_process: Option<u32>,
    // Force process block with specified number when sampling is enabled.
    pub force_process_block: Option<u32>,

    // whether to write to public GCS bucket for https://github.com/matter-labs/era-boojum-validator-cli
    pub shall_save_to_public_bucket: bool,
}

#[derive(Debug)]
pub struct WitnessGenerationTimeouts {
    basic: Duration,
    leaf: Duration,
    node: Duration,
    scheduler: Duration,
}

impl WitnessGenerationTimeouts {
    pub fn basic(&self) -> Duration {
        self.basic
    }

    pub fn leaf(&self) -> Duration {
        self.leaf
    }

    pub fn node(&self) -> Duration {
        self.node
    }

    pub fn scheduler(&self) -> Duration {
        self.scheduler
    }

    pub fn new(basic: u16, leaf: u16, node: u16, scheduler: u16) -> Self {
        Self {
            basic: Duration::from_secs(basic as u64),
            leaf: Duration::from_secs(leaf as u64),
            node: Duration::from_secs(node as u64),
            scheduler: Duration::from_secs(scheduler as u64),
        }
    }
}

impl FriWitnessGeneratorConfig {
    pub fn witness_generation_timeouts(&self) -> WitnessGenerationTimeouts {
        WitnessGenerationTimeouts::new(
            self.basic_generation_timeout_in_secs
                .unwrap_or(self.generation_timeout_in_secs),
            self.leaf_generation_timeout_in_secs
                .unwrap_or(self.generation_timeout_in_secs),
            self.node_generation_timeout_in_secs
                .unwrap_or(self.generation_timeout_in_secs),
            self.scheduler_generation_timeout_in_secs
                .unwrap_or(self.generation_timeout_in_secs),
        )
    }

    pub fn last_l1_batch_to_process(&self) -> u32 {
        self.last_l1_batch_to_process.unwrap_or(u32::MAX)
    }
}
