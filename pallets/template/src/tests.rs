use crate::{self as merkle_root, Config, Error, RawEvent};
use frame_support::{assert_err, assert_ok, construct_runtime, parameter_types};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		MerkleRoots: merkle_root::{Module, Call, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Index = u64;
	type Call = Call;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl Config for TestRuntime {
	type Event = Event;
}

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = frame_system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn set_merkleroot_works() {
	ExternalityBuilder::build().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(MerkleRoots::set_merkleroot_entry(Origin::signed(1), hash.clone()));

		let expected_event = Event::merkle_root(RawEvent::MerkleRootEntry(1, hash.clone()));

		assert_eq!(System::events()[0].event, expected_event);
	})
}

#[test]
fn get_merkleroot_throws() {
	ExternalityBuilder::build().execute_with(|| {
		assert_err!(
			MerkleRoots::get_merkleroot(Origin::signed(2), [0,1,2].to_vec()),
			Error::<TestRuntime>::NoValueStored
		);
	})
}

#[test]
fn get_merkleroot_works() {
	ExternalityBuilder::build().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(MerkleRoots::set_merkleroot_entry(Origin::signed(2), hash.clone()));
		assert_ok!(MerkleRoots::get_merkleroot(Origin::signed(1), [0,1,2].to_vec()));

		let expected_event = Event::merkle_root(RawEvent::MerkleRootEntryFound(1, hash.clone()));

		assert_eq!(System::events()[1].event, expected_event);
		
	})
}
