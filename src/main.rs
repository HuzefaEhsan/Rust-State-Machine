mod balances;
mod system;

#[derive(Debug)]
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	// Create a mutable variable `runtime`, which is a new instance of `Runtime`.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Set the balance of `alice` to 100, allowing us to execute other transactions.
	runtime.balances.set_balance(&alice, 100);

	// start emulating a block
	// Increment the block number in system.
	runtime.system.inc_block_number();
	// Assert the block number is what we expect.
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	// Increment the nonce of `alice`.
	runtime.system.inc_nonce(&alice);
	// Execute a transfer from `alice` to `bob` for 30 tokens.
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob.clone(), 30)
		.map_err(|e| eprintln!("{}", e));

	// second transaction
	// Increment the nonce of `alice` again.
	runtime.system.inc_nonce(&alice);
	// Execute another balance transfer, this time from `alice` to `charlie` for 20.
	let _res = runtime
		.balances
		.transfer(alice.clone(), charlie.clone(), 20)
		.map_err(|e| eprintln!("{}", e));

	// Print the final state of our runtime.
	println!("{:#?}", runtime)
}
