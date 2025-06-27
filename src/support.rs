/// A generic representation of a blockchain block.
pub struct Block<Header, Extrinsic> {
	/// The block header.
	pub header: Header,
	/// A list of extrinsics contained in the block.
	pub extrinsics: Vec<Extrinsic>,
}

/// A simplified block header containing only the block number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

/// A transaction, or "extrinsic," from outside the blockchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

/// A result type for dispatchable functions.
pub type DispatchResult = Result<(), &'static str>;

/// A trait for dispatching extrinsics.
pub trait Dispatch {
	/// The caller of the extrinsic.
	type Caller;
	/// The `Call` to be executed.
	type Call;

	/// Dispatches a call on behalf of a caller.
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
