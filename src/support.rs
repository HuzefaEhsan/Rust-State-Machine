/// A generic representation of a blockchain block.
pub struct Block<Header, Extrinsic> {
	/// The block header, containing metadata about the block.
	pub header: Header,
	/// The list of extrinsics, representing state transitions to be executed.
	pub extrinsics: Vec<Extrinsic>,
}

/// A simplified block header containing only the block number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

/// An "extrinsic," representing an external message from outside the blockchain.
///
/// Contains the caller and the specific call to be executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

/// A result type for dispatchable functions within the runtime.
pub type DispatchResult = Result<(), &'static str>;

/// A trait for dispatching extrinsics to the appropriate runtime function.
pub trait Dispatch {
	/// The type representing the caller of the extrinsic.
	type Caller;
	/// The type representing the `Call` to be executed.
	type Call;

	/// Dispatches a `call` on behalf of a `caller`.
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
