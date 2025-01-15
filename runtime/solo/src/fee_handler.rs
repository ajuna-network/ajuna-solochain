use crate::{
	AccountId, AffiliateMaxLevel, AffiliatesHeroJam, AssetId, Assets, Balance, Balances,
	TournamentHeroJam,
};
use ajuna_payment_handler::{
	AllowAllAssets, AssetGameFeeHandler, VoucherHandler, WithdrawCreditOrVoucher,
	WithdrawFungibles, WithdrawWhitelistedCredit,
};
use frame_support::traits::fungible::{NativeFromLeft, NativeOrWithId, UnionOf};
use sp_runtime::DispatchError;

pub type NativeAndAssets = UnionOf<
	Balances,
	Assets,
	NativeFromLeft,
	NativeOrWithId<example_transition::asset::AssetId>,
	AccountId,
>;

pub type HeroJamFeeHandler = AssetGameFeeHandler<
	AccountId,
	NativeAndAssets,
	WithdrawCreditOrVoucher<
		WithdrawWhitelistedCredit<
			AllowAllAssets<NativeOrWithId<AssetId>>,
			WithdrawFungibles<AccountId, NativeAndAssets>,
		>,
		DummyVoucherHandler,
	>,
	AffiliatesHeroJam,
	AffiliateMaxLevel,
	TournamentHeroJam,
>;

pub struct DummyVoucherHandler;

impl VoucherHandler for DummyVoucherHandler {
	type AccountId = AccountId;
	type Balance = Balance;

	fn consume_vouchers_from(
		_account: &Self::AccountId,
		_amount: Self::Balance,
	) -> Result<(), DispatchError> {
		Ok(())
	}
}
