mod balances;
mod system;

// Concrete types used throughout this runtime.
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

// The main runtime struct.
// Accumulates all of the pallets to be used in the state machine.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

// Implementation of the `system::Config` trait for the Runtime.
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

// Implementation of the `balances::Config` trait for the Runtime.
impl balances::Config for Runtime {
	// `AccountId` is inherited from `system::Config`, so it is not defined here.
	type Balance = types::Balance;
}

impl Runtime {
	// Create a new instance of the main Runtime.
	fn new() -> Self {
		Self { system: system::Pallet::<Self>::new(), balances: balances::Pallet::<Self>::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

	// Emulate a block execution.
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// First transaction.
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("{}", e));

	// Second transaction.
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}
