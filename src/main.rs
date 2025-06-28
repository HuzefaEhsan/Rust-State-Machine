/// Author: Huzefa Ehsan
/// Project: Rust State Machine Exercise by Shawn Tabrizi (dotcodeschool.com)
/// Date: 2025-06-28
/// Description: A simple runtime simulation in Rust that demonstrates a proof of existence
/// system with basic account balances and block execution.
mod balances;
mod proof_of_existence;
mod support;
mod system;

// Import the `Dispatch` trait to satisfy the trait bounds of the macros.
use support::Dispatch;

/// Concrete types used throughout the runtime.
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Content = &'static str;

	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

/// The main runtime struct.
///
/// The `#[macros::runtime]` attribute automatically generates the `RuntimeCall` enum,
/// the `new()` and `execute_block()` functions, and the `Dispatch` trait implementation.
#[macros::runtime]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

/// Implements the `system::Config` trait for the `Runtime`.
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

/// Implements the `balances::Config` trait for the `Runtime`.
impl balances::Config for Runtime {
	type Balance = types::Balance;
}

/// Implements the `proof_of_existence::Config` trait for the `Runtime`.
impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

// The `enum RuntimeCall`, `impl Runtime`, and `impl support::Dispatch for Runtime`
// are now all generated automatically by the `#[macros::runtime]` attribute.

/// The main entry point for the runtime simulation.
fn main() {
	// Instantiate the runtime.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Set up the genesis state.
	runtime.balances.set_balance(&alice, 100);

	// Construct block 1: Balance transfers.
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: bob.clone(),
					amount: 30,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie, amount: 20 }),
			},
		],
	};

	// Construct block 2: Proof of Existence claims.
	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Hello, world!",
				}),
			},
		],
	};

	// Construct block 3: Claim revocation and re-claim.
	let block_3 = types::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
					claim: "Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Hello, world!",
				}),
			},
		],
	};

	// Execute the blocks.
	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");

	// Print the final runtime state for verification.
	println!("{:#?}", runtime);

	// Verify the final state.
	assert_eq!(runtime.system.block_number(), 3);
	assert_eq!(runtime.system.nonce(&alice), 4);
	assert_eq!(runtime.system.nonce(&bob), 2);
	assert_eq!(runtime.balances.balance(&alice), 50);
	assert_eq!(runtime.proof_of_existence.get_claim(&"Hello, world!"), Some(&bob));
}
