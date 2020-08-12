use super::*;
use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, impl_outer_dispatch, parameter_types, impl_outer_event, weights::Weight};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill, ModuleId,
	};

impl_outer_origin! {
	pub enum Origin for Test {}
}

// impl_outer_event! {
// 	pub enum Event for Test {
// 		frame_system<T>,
// 		balances<T>,
// 		Uniswap<T>
// 	}
// }

// impl_outer_dispatch! {
// 	pub enum Call for Test where origin: Origin {
// 		frame_system::System,
// 		balances::Balances,
// 	}
// }

type System = frame_system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;


// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl Trait for Test {
	type Event = ();
	type Balance = u128;
	type AssetId = u128;
	type Currency = Balances;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
use pallet_balances::Call as BalancesCall;
use pallet_balances::Error as BalancesError;


impl pallet_balances::Trait for Test {
	type Balance = u64;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub type Uniswap = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10), (2, 10), (3, 10), (4, 10), (5, 2)],
	}.assimilate_storage(&mut t).unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
