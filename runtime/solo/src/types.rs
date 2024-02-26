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

use sp_runtime::traits::{IdentifyAccount, Verify};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = sp_runtime::MultiSignature;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like the signature, this
/// also isn't a fixed size when encoded, as different cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Identifier of an asset.
pub type AssetId = u32;

/// Id used for identifying non-fungible collections.
pub type CollectionId = u32;

/// Id used for identifying non-fungible items.
pub type ItemId = Hash;

pub mod proxy {
	use frame_support::{
		pallet_prelude::MaxEncodedLen, traits::InstanceFilter, RuntimeDebugNoBound,
	};
	use parity_scale_codec::{Decode, Encode};
	use scale_info::TypeInfo;

	/// Proxy type enum lists the type of calls that are supported by the proxy
	/// pallet
	#[derive(
		Copy,
		Clone,
		Eq,
		PartialEq,
		Ord,
		PartialOrd,
		MaxEncodedLen,
		Decode,
		Encode,
		RuntimeDebugNoBound,
		TypeInfo,
	)]
	pub enum ProxyType {
		Any,
	}

	impl Default for ProxyType {
		fn default() -> Self {
			Self::Any
		}
	}

	impl<Call> InstanceFilter<Call> for ProxyType {
		fn filter(&self, _c: &Call) -> bool {
			match self {
				ProxyType::Any => true,
			}
		}
		fn is_superset(&self, o: &Self) -> bool {
			self == &ProxyType::Any || self == o
		}
	}
}
