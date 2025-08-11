// use cairo_air::{
//     utils::{serialize_proof_to_file, ProofFormat},
//     verifier::verify_cairo,
//     CairoProof, PreProcessedTraceVariant,
// };
// use cairo_lang_runner::Arg;
use cairo_prove::{
    args::{Cli, Commands, ProgramArguments},
    execute::execute,
    prove::{prove, prover_input_from_runner},
};
// // use clap::Parser;
// // use log::{error, info};
// use stwo_cairo_prover::stwo_prover::core::fri::FriConfig;
// use stwo_cairo_prover::stwo_prover::core::{
//     pcs::PcsConfig,
//     vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
// };
