use crate::{AccountId, AffiliateMaxLevel, AssetId, Assets, Balance, Balances};
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

/// Fee handler facilitating payment in the native currency and with whitelisted assets.
pub type AjunaAssetFeeHandler<Affiliates, AffiliateMaxLevel, Tournament> = AssetGameFeeHandler<
	AccountId,
	NativeAndAssets,
	WithdrawCreditOrVoucher<
		WithdrawWhitelistedCredit<
			AllowAllAssets<NativeOrWithId<AssetId>>,
			WithdrawFungibles<AccountId, NativeAndAssets>,
		>,
		DummyVoucherHandler,
	>,
	Affiliates,
	AffiliateMaxLevel,
	Tournament,
>;
