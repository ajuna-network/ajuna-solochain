use crate::{AccountId, AssetId, Assets, Balances};
use ajuna_payment_handler::{
	AllowAllAssets, AssetGameFeeHandler, WithdrawCreditOrVoucher, WithdrawFungibles,
	WithdrawWhitelistedCredit,
};
use frame_support::traits::fungible::{NativeFromLeft, NativeOrWithId, UnionOf};

pub type NativeAndAssets = UnionOf<
	Balances,
	Assets,
	NativeFromLeft,
	NativeOrWithId<example_transition::asset::AssetId>,
	AccountId,
>;

/// Fee handler facilitating payment in the native currency and with whitelisted assets.
pub type AjunaAssetFeeHandler<Affiliates, AffiliateMaxLevel, Tournament, VoucherHandler> =
	AssetGameFeeHandler<
		AccountId,
		NativeAndAssets,
		WithdrawCreditOrVoucher<
			WithdrawWhitelistedCredit<
				AllowAllAssets<NativeOrWithId<AssetId>>,
				WithdrawFungibles<AccountId, NativeAndAssets>,
			>,
			VoucherHandler,
		>,
		Affiliates,
		AffiliateMaxLevel,
		Tournament,
	>;
