use crate::{AccountId, HeroJamAsset, Runtime, SageHeroJam, System};

use ajuna_primitives::{
	asset_manager::{AssetInspector, AssetManager, Lock, LockIdentifier},
	chain_inspector::ChainInspector,
};
use example_transition::asset::AssetId;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::DispatchError;

pub struct HeroJamAssetMediator;

impl AssetManager for HeroJamAssetMediator {
	type AccountId = AccountId;
	type AssetId = AssetId;
	type Asset = HeroJamAsset;

	fn ensure_ownership(
		owner: &Self::AccountId,
		asset_id: &Self::AssetId,
	) -> Result<Self::Asset, DispatchError> {
		<SageHeroJam as AssetManager>::ensure_ownership(owner, asset_id)
	}

	fn lock_asset(
		lock_id: LockIdentifier,
		owner: Self::AccountId,
		asset_id: Self::AssetId,
	) -> Result<Self::Asset, DispatchError> {
		<SageHeroJam as AssetManager>::lock_asset(lock_id, owner, asset_id)
	}

	fn unlock_asset(
		lock_id: LockIdentifier,
		owner: Self::AccountId,
		asset_id: Self::AssetId,
	) -> Result<Self::Asset, DispatchError> {
		<SageHeroJam as AssetManager>::unlock_asset(lock_id, owner, asset_id)
	}

	fn is_locked(asset: &Self::AssetId) -> Option<Lock<Self::AccountId>> {
		<SageHeroJam as AssetManager>::is_locked(asset)
	}
}

impl AssetInspector for HeroJamAssetMediator {
	type AccountId = AccountId;
	type AssetId = AssetId;
	type Asset = HeroJamAsset;

	fn get_asset(asset_id: &Self::AssetId) -> Result<Self::Asset, DispatchError> {
		<SageHeroJam as AssetInspector>::get_asset(asset_id)
	}

	fn iter_assets_from(
		account_id: &Self::AccountId,
	) -> impl Iterator<Item = (Self::AssetId, Self::Asset)> {
		<SageHeroJam as AssetInspector>::iter_assets_from(account_id)
	}
}

impl ChainInspector for HeroJamAssetMediator {
	type BlockNumber = BlockNumberFor<Runtime>;

	fn get_current_block_number() -> Self::BlockNumber {
		System::block_number()
	}
}
