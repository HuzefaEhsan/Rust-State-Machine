mod balances;
mod support;
mod system;

/// Type definitions for the runtime.
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

/// The main runtime struct, which aggregates all pallets.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

/// Implements the `system::Config` trait for the `Runtime`.
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

/// Implements the `balances::Config` trait for the `Runtime`.
impl balances::Config for Runtime {
	// `AccountId` is inherited from `system::Config`.
	type Balance = types::Balance;
}

impl Runtime {
	/// Constructs a new instance of the runtime.
	fn new() -> Self {
		Self { system: system::Pallet::<Self>::new(), balances: balances::Pallet::<Self>::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Set up the genesis state.
	runtime.balances.set_balance(&alice, 100);

	// --- Block 1 Execution ---
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// Transaction 1: Alice -> Bob.
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("[Transaction 1 Failed] {}", e));

	// Transaction 2: Alice -> Charlie.
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), charlie, 20)
		.map_err(|e| eprintln!("[Transaction 2 Failed] {}", e));

	println!("{:#?}", runtime);
}
