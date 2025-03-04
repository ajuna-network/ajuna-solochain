// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use super::*;
use crate::{
	fee_handler::{AjunaAssetFeeHandler, NativeAndAssets},
	Balances, BlockNumber, CasinoJamAffiliates, CasinoJamRandom, CasinoJamSage, CasinoJamSeasons,
	CasinoJamTournament, Runtime, RuntimeEvent,
};

use example_transition::prelude::*;

use frame_support::{
	pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo},
	parameter_types,
	traits::fungible::NativeOrWithId,
	PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_core::H256;
use sp_runtime::DispatchError;
use sp_std::{cmp::Ordering, prelude::*};

pub type CasinoJamFeeHandler = AjunaAssetFeeHandler<
	CasinoJamAffiliates,
	AffiliateMaxLevel,
	CasinoJamTournament,
	DummyVoucherHandler,
>;

pub struct CasinoJamSageEngine;

/// Runtime specific sage implementation so that we don't have to
/// pass our type definitions all the time.
macro_rules! impl_runtime_sage_api {
	(
		$impl_target:ident,
		$runtime:ident,
		$sage_instance:ident,
		$season_manager:ident,
		$asset_id:ident,
		$asset:ident,
		$transition_config:ident,
	) => {
		impl_sage_api!(
			$impl_target,
			$runtime,
			$sage_instance,
			$season_manager,
			CasinoJamRandom,
			AccountId,
			$asset_id,
			$asset,
			FungiblesAssetId,
			Balance,
			BlockNumber,
			CasinoJamSeasonId,
			$transition_config,
			H256,
		);
	};
}

// Every new game we add can simply call that macro for another sage instance to
// implement the sage api given that the other types are identical.
impl_runtime_sage_api!(
	CasinoJamSageEngine,
	Runtime,
	CasinoJamSageInstance,
	CasinoJamSeasons,
	AssetId,
	CasinoJamAsset,
	CasinoJamTransitionConfig,
);

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

parameter_types! {
	pub const SageCasinoJamId: PalletId = PalletId(*b"sage/hjm");
}

pub type CasinoJamAsset = Asset<BlockNumberFor<Runtime>>;
pub type CasinoJamGameTransition =
	CasinoJamTransition<AccountId, BlockNumberFor<Runtime>, CasinoJamSageEngine>;

pub type CasinoJamAssetFilter = GameFilter<BlockNumberFor<Runtime>>;
#[cfg(feature = "runtime-benchmarks")]
pub type CasinoJamBenchmarkHelper = GameBenchmarkHelper<BlockNumberFor<Runtime>>;

type FungiblesAssetId = WithdrawKind<NativeOrWithId<AssetId>>;
type TransferWithdraw =
	WithdrawCreditOrVoucher<WithdrawFungibles<AccountId, NativeAndAssets>, DummyVoucherHandler>;

pub type CasinoJamSageInstance = pallet_sage::Instance1;
impl pallet_sage::Config<CasinoJamSageInstance> for Runtime {
	type PalletId = SageCasinoJamId;
	type SageGameTransition = CasinoJamGameTransition;
	type SeasonHandler = CasinoJamSeasons;
	type FeeHandler = CasinoJamFeeHandler;
	type TransferFunds = TransferFungibleAssets<TransferWithdraw, FungiblesAssetId>;
	type FungiblesAssetId = FungiblesAssetId;
	type FilterHandler = CasinoJamAssetFilter;
	type Fungible = Balances;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = CasinoJamBenchmarkHelper;
}

pub type CasinoJamSeasonId = u32;

pub type SeasonsCasinoJamInstance = pallet_ajuna_seasons::Instance1;
impl pallet_ajuna_seasons::Config<SeasonsCasinoJamInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SeasonId = CasinoJamSeasonId;
	type AssetId = AssetId;
	type AccountHandler = CasinoJamSage;
	type Currency = Balances;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub const AffiliateMaxLevel: u32 = 2;
}

pub type CasinoJamRuleIdentifier = AffiliateMethods<CasinoAction>;

#[cfg(feature = "runtime-benchmarks")]
pub struct AffiliatesCasinoJamBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl pallet_ajuna_affiliates::BenchmarkHelper<CasinoJamRuleIdentifier, ()>
	for AffiliatesCasinoJamBenchmarkHelper
{
	fn create_rule_id(_id: u32) -> CasinoJamRuleIdentifier {
		AffiliateMethods::StateTransition(CasinoAction::Create(AssetType::Player))
	}

	fn create_params(_id: u32) {}
}

pub struct MockAffiliateUnlockRules;

impl AffiliateUnlockRules for MockAffiliateUnlockRules {
	type AccountId = AccountId;
	type UnlockParameters = ();

	fn execute_unlock_rule_for(
		_account: &Self::AccountId,
		_params: Self::UnlockParameters,
	) -> Result<(), DispatchError> {
		Ok(())
	}
}

pub type AffiliatesCasinoJamInstance = pallet_ajuna_affiliates::Instance1;
impl pallet_ajuna_affiliates::Config<AffiliatesCasinoJamInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WhitelistKey = ();
	type AccountManager = CasinoJamSage;
	type RuleIdentifier = CasinoJamRuleIdentifier;
	type AffiliateMaxLevel = AffiliateMaxLevel;
	type UnlockParameters = ();
	type AffiliatesUnlockRules = MockAffiliateUnlockRules;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AffiliatesCasinoJamBenchmarkHelper;
}

parameter_types! {
	pub const TournamentPalletId1: PalletId = PalletId(*b"aj/trmt1");
	pub const MinimumTournamentPhaseDuration: BlockNumber = 100;
}

pub type CasinoTournamentCategoryId = u32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct CasinoJamEntityRanker;

impl EntityRank for CasinoJamEntityRanker {
	type EntityId = AssetId;
	type Entity = CasinoJamAsset;

	fn can_rank(&self, _: (&Self::EntityId, &Self::Entity)) -> bool {
		true
	}

	fn rank_against(
		&self,
		_: (&Self::EntityId, &Self::Entity),
		_: (&Self::EntityId, &Self::Entity),
	) -> Ordering {
		Ordering::Equal
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub struct MockTournamentBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl
	pallet_ajuna_tournament::BenchmarkHelper<
		CasinoTournamentCategoryId,
		BlockNumberFor<Runtime>,
		Balance,
		CasinoJamEntityRanker,
		AccountId,
		AssetId,
		CasinoJamAsset,
	> for MockTournamentBenchmarkHelper
{
	fn create_category_id(id: u32) -> CasinoTournamentCategoryId {
		id
	}

	fn create_default_tournament_config() -> pallet_ajuna_tournament::TournamentConfig<
		BlockNumberFor<Runtime>,
		Balance,
		CasinoJamEntityRanker,
	> {
		pallet_ajuna_tournament::TournamentConfig {
			start: 0_u32,
			active_end: 100_u32,
			claim_end: 200_u32,
			initial_reward: None,
			max_reward: None,
			take_fee_percentage: None,
			reward_distribution: Default::default(),
			golden_duck_config: Default::default(),
			max_players: 0,
			ranker: CasinoJamEntityRanker,
		}
	}

	fn create_entities(_: AccountId, _: u32) -> Vec<(AssetId, CasinoJamAsset)> {
		vec![]
	}
}

type TournamentCasinoJamInstance = pallet_ajuna_tournament::Instance1;
impl pallet_ajuna_tournament::Config<TournamentCasinoJamInstance> for Runtime {
	type PalletId = TournamentPalletId1;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type TournamentCategoryId = CasinoTournamentCategoryId;
	type EntityId = AssetId;
	type RankedEntity = CasinoJamAsset;
	type EntityRanker = CasinoJamEntityRanker;
	type AccountManager = CasinoJamSage;
	type AssetManager = CasinoJamSage;
	type MinimumTournamentPhaseDuration = MinimumTournamentPhaseDuration;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = MockTournamentBenchmarkHelper;
}
