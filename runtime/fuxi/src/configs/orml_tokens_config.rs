use orml_tokens::Config;
use uniarts_primitives::*;
use crate::*;
use crate::weights::orml_tokens::WeightInfo;
use orml_traits::parameter_type_with_key;
use sp_runtime::traits::Zero;

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Zero::zero()
	};
}

parameter_types! {
	pub TreasuryModuleAccount: AccountId = UniArtsTreasuryModuleId::get().into_account();
}

impl Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = orml_tokens::TransferDust<Runtime, TreasuryModuleAccount>;
    type WeightInfo = WeightInfo<Runtime>;
}