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
use crate::{AccountId, Balance};

use ajuna_payment_handler::{
	TransferFungibleAssets, WithdrawCreditOrVoucher, WithdrawFungibles, WithdrawKind,
};
use pallet_ajuna_affiliates::traits::AffiliateUnlockRules;
use pallet_ajuna_tournament::EntityRank;
use pallet_sage::AffiliateMethods;

use ajuna_payment_handler::VoucherHandler;
use ajuna_primitives::{asset_manager::*, sage_api::SageApi, season_manager::*};
use pallet_sage::*;
use sp_runtime::DispatchError;

mod casino_jam;

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
