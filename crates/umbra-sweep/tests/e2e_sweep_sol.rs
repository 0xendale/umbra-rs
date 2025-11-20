use std::{env, path::PathBuf, thread, time::Duration};

use rand_core::OsRng;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash,
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

use umbra_core::ScalarWrapper;
use umbra_sweep::{
    build_and_sign_sweep_sol_transaction, derivation::ScalarSigner, SweepSolParams,
    MIN_REMAINING_LAMPORTS,
};

/// Wait until an account reaches a minimum balance.
fn wait_for_balance(
    rpc: &RpcClient,
    account: &Pubkey,
    min_lamports: u64,
    max_attempts: usize,
) -> u64 {
    for _ in 0..max_attempts {
        if let Ok(bal) = rpc.get_balance(account) {
            if bal >= min_lamports {
                return bal;
            }
        }
        thread::sleep(Duration::from_millis(300));
    }
    rpc.get_balance(account).unwrap_or(0)
}

fn bootstrap_keypair_path() -> PathBuf {
    let home = env::var("HOME").expect("missing HOME");
    PathBuf::from(home)
        .join(".config")
        .join("solana")
        .join("id.json")
}

#[test]
fn e2e_scalar_signer_sweeps_sol() {
    let rpc = RpcClient::new("http://127.0.0.1:8899".to_string());

    // ---------------------------------------------------------------------
    // LOAD BOOTSTRAP PAYER
    // ---------------------------------------------------------------------
    let path = bootstrap_keypair_path();
    println!("Loading bootstrap payer from {:?}", path);

    let bootstrap = read_keypair_file(&path).expect("failed to read bootstrap payer");
    let bootstrap_pk = bootstrap.pubkey();

    println!("Bootstrap payer: {}", bootstrap_pk);

    let bootstrap_balance = rpc.get_balance(&bootstrap_pk).unwrap();
    assert!(
        bootstrap_balance > 100_000_000_000,
        "bootstrap has insufficient funds"
    );
    println!("Bootstrap balance: {}", bootstrap_balance);

    // ---------------------------------------------------------------------
    // CREATE FUNDER ACCOUNT
    // ---------------------------------------------------------------------
    let funder = Keypair::new();
    let funder_pk = funder.pubkey();
    println!("Funder: {}", funder_pk);

    let ix1 =
        solana_system_interface::instruction::transfer(&bootstrap_pk, &funder_pk, 2_000_000_000);
    let bh1 = rpc.get_latest_blockhash().unwrap();

    let msg1 = Message::new(&[ix1], Some(&bootstrap_pk));
    let mut tx1 = Transaction::new_unsigned(msg1);
    tx1.sign(&[&bootstrap], bh1);

    rpc.send_and_confirm_transaction(&tx1)
        .expect("funding failed");

    let funder_balance = wait_for_balance(&rpc, &funder_pk, 1_000_000_000, 40);
    println!("Funder balance OK: {}", funder_balance);

    // ---------------------------------------------------------------------
    // CREATE SCALAR SIGNER (UMBRA CLAIMANT)
    // ---------------------------------------------------------------------
    let mut rng = OsRng;
    let scalar = ScalarWrapper::random(&mut rng);
    let signer = ScalarSigner::new(scalar);
    let p_pk = signer.pubkey();

    println!("One-time P: {}", p_pk);

    // ---------------------------------------------------------------------
    // FUND P (ephemeral account)
    // ---------------------------------------------------------------------
    let ix2 = solana_system_interface::instruction::transfer(&funder_pk, &p_pk, 1_000_000_000);
    let bh2 = rpc.get_latest_blockhash().unwrap();

    let msg2 = Message::new(&[ix2], Some(&funder_pk));
    let mut tx2 = Transaction::new_unsigned(msg2);
    tx2.sign(&[&funder], bh2);

    rpc.send_and_confirm_transaction(&tx2)
        .expect("fund P failed");

    let p_balance = wait_for_balance(&rpc, &p_pk, 500_000_000, 40);
    println!("P funded: {}", p_balance);

    // ---------------------------------------------------------------------
    // SWEEP FROM P → DESTINATION
    // ---------------------------------------------------------------------
    let destination = Keypair::new();
    let dest_pk = destination.pubkey();

    let bh3: Hash = rpc.get_latest_blockhash().unwrap();

    let sweep_amount = p_balance.saturating_sub(MIN_REMAINING_LAMPORTS);

    let params = SweepSolParams {
        signer: &signer,
        to: dest_pk,
        amount: sweep_amount,
        recent_blockhash: bh3,
    };

    let sweep_tx = build_and_sign_sweep_sol_transaction(&params).expect("build sweep failed");

    rpc.send_and_confirm_transaction(&sweep_tx)
        .expect("sweep failed");

    // ---------------------------------------------------------------------
    // VERIFY RESULT
    // ---------------------------------------------------------------------
    let dest_balance = wait_for_balance(&rpc, &dest_pk, sweep_amount / 2, 40);
    println!("Destination received {} lamports", dest_balance);
    assert!(dest_balance > 0, "destination did not receive swept funds");
    println!("✔ E2E scalar-signer sweep test PASSED!");
}
