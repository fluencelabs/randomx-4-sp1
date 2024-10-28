//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::{sol_data::Bytes, SolType};
use clap::Parser;
// use fibonacci_lib::PublicValuesStruct;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const RANDOMX_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "20")]
    n: u32,
}

const BLOCK_TEMPLATE: &[u8] = &[
    0x07, 0x07, 0xf7, 0xa4, 0xf0, 0xd6, 0x05, 0xb3, 0x03, 0x26, 0x08, 0x16, 0xba, 0x3f, 0x10, 0x90,
    0x2e, 0x1a, 0x14, 0x5a, 0xc5, 0xfa, 0xd3, 0xaa, 0x3a, 0xf6, 0xea, 0x44, 0xc1, 0x18, 0x69, 0xdc,
    0x4f, 0x85, 0x3f, 0x00, 0x2b, 0x2e, 0xea, 0x00, 0x00, 0x00, 0x00, 0x77, 0xb2, 0x06, 0xa0, 0x2c,
    0xa5, 0xb1, 0xd4, 0xce, 0x6b, 0xbf, 0xdf, 0x0a, 0xca, 0xc3, 0x8b, 0xde, 0xd3, 0x4d, 0x2d, 0xcd,
    0xee, 0xf9, 0x5c, 0xd2, 0x0c, 0xef, 0xc1, 0x2f, 0x61, 0xd5, 0x61, 0x09,
];

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    let micro_cache = vec![42u8; 131072];

    // println!("n: {}", da[2]);
    // da[0] = 5;
    let n = args.n;

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();

    // stdin.write(&n);
    stdin.write_vec(micro_cache);
    stdin.write_slice(BLOCK_TEMPLATE);

    if args.execute {
        // Execute the program
        let start = std::time::Instant::now();

        let mut res = client.execute(RANDOMX_ELF, stdin).run();
        match res {
            Ok((mut output, report)) => {
                let end = std::time::Instant::now();
                println!("Execution time: {:?}", (end - start).as_secs());

                println!("Program executed successfully.");

                let hash = Bytes::abi_decode(output.as_slice(), true).unwrap();
                println!("hash: {:?}", hash);
                // let number: u32 = output.read();
                // println!("Number: {}", number);
                println!("Number of cycles: {}", report.total_instruction_count());
                println!(
                    "Number of addresses touched: {}",
                    report.touched_memory_addresses
                );
            }
            Err(e) => {
                println!("Execution failed: {:?}", e);
            }
        }

        // Read the output.
        // let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        // let PublicValuesStruct { n, a, b } = decoded;
        // println!("n: {}", n);
        // println!("a: {}", a);
        // println!("b: {}", b);

        // let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
        // assert_eq!(a, expected_a);
        // assert_eq!(b, expected_b);
        // println!("Values are correct!");
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(RANDOMX_ELF);

        let start = std::time::Instant::now();
        // Generate the proof\
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");
        let end = std::time::Instant::now();
        println!("Proof generation time: {:?}", (end - start).as_secs());

        println!("Successfully generated proof!");

        // Verify the proof.
        // client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
