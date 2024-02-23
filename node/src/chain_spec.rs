use ajuna_solo_runtime::{AccountId, AuraId, Signature, AJUNS};
use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

const TOKEN_SYMBOL: &'static str = "AJUN";
const TOKEN_DECIMALS: u32 = 12;
const SS58_FORMAT: u32 = 42;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> ajuna_solo_runtime::SessionKeys {
	ajuna_solo_runtime::SessionKeys { aura: keys }
}

pub fn get_well_known_accounts() -> WellKnownAccounts {
	WellKnownAccounts {
		alice: get_account_id_from_seed::<sr25519::Public>("Alice"),
		bob: get_account_id_from_seed::<sr25519::Public>("Bob"),
		charlie: get_account_id_from_seed::<sr25519::Public>("Charlie"),
		dave: get_account_id_from_seed::<sr25519::Public>("Dave"),
		eve: get_account_id_from_seed::<sr25519::Public>("Eve"),
		ferdie: get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		alice_stash: get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		bob_stash: get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		charlie_stash: get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		dave_stash: get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		eve_stash: get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		ferdie_stash: get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		alice_authority: authority_keys_from_seed("Alice"),
		bob_authority: authority_keys_from_seed("Bob"),
		charlie_authority: authority_keys_from_seed("Charlie"),
	}
}

pub fn development_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58_FORMAT.into());

	ChainSpec::builder(
		ajuna_solo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		NoExtension,
	)
	.with_name("Ajuna Dev Testnet")
	.with_id("ajuna-dev-testnet")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_patch(development_config_genesis())
	.build()
}

pub fn local_testnet_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58_FORMAT.into());

	ChainSpec::builder(
		ajuna_solo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		NoExtension,
	)
		.with_name("Ajuna Testnet")
		.with_id("ajuna-testnet")
		.with_chain_type(ChainType::Local)
		.with_genesis_config_patch(development_config_genesis())
		.build()
}

fn development_config_genesis() -> serde_json::Value {
	let accounts = get_well_known_accounts();
	let (aura_authorities, grandpa_authorities) = [accounts.alice_authority].into_iter().unzip();

	const INITIAL_BALANCE: Balance = 1_000_000_000 * AJUNS;
	const INITIAL_ASSET_BALANCE: Balance = 1_000_000_000;
	const VEST_BALANCE: Balance = 123 * AJUNS;
	let vest_alice_from_0_to_10_at_period_1 = (accounts.alice.clone(), 0, 1, 10, VEST_BALANCE);
	let vest_bob_from_0_to_20_at_period_2 = (accounts.bob.clone(), 0, 2, 10, VEST_BALANCE);
	let vest_charlie_from_0_to_36_at_period_3 = (accounts.charlie.clone(), 0, 3, 12, VEST_BALANCE);
	let cliff_vest_dave_at_10 = (accounts.dave.clone(), 9, 10, 1, VEST_BALANCE);
	let cliff_vest_eve_at_20 = (accounts.eve.clone(), 19, 20, 1, VEST_BALANCE);
	let cliff_vest_ferdie_at_30 = (accounts.ferdie.clone(), 29, 30, 1, VEST_BALANCE);

	compose_genesis_config(Config {
		aura: AuraConfig { authorities: aura_authorities },
		grandpa: GrandpaConfig { authorities: grandpa_authorities, ..Default::default() },
		sudo: SudoConfig { key: Some(accounts.alice.clone()) },
		council: CouncilConfig {
			members: vec![
				accounts.alice.clone(),
				accounts.bob.clone(),
				accounts.charlie.clone(),
				accounts.dave.clone(),
				accounts.eve.clone(),
			],
			phantom: Default::default(),
		},
		technical_committee: TechnicalCommitteeConfig {
			members: vec![accounts.alice.clone(), accounts.bob.clone(), accounts.charlie.clone()],
			phantom: Default::default(),
		},
		balances: BalancesConfig {
			balances: vec![
				(accounts.alice.clone(), INITIAL_BALANCE),
				(accounts.bob.clone(), INITIAL_BALANCE),
				(accounts.charlie.clone(), INITIAL_BALANCE),
				(accounts.dave, VEST_BALANCE),
				(accounts.eve, VEST_BALANCE),
				(accounts.ferdie, VEST_BALANCE),
				(accounts.alice_stash, INITIAL_BALANCE),
				(accounts.bob_stash, INITIAL_BALANCE),
			],
		},
		assets: AssetsConfig {
			assets: vec![(0, accounts.alice.clone(), true, 1)],
			metadata: vec![(0, "Dotmog".into(), "DMOG".into(), 3)],
			accounts: vec![
				(0, accounts.alice, INITIAL_ASSET_BALANCE),
				(0, accounts.bob, INITIAL_ASSET_BALANCE),
				(0, accounts.charlie, INITIAL_ASSET_BALANCE),
			],
		},
		vesting: VestingConfig {
			vesting: vec![
				vest_alice_from_0_to_10_at_period_1,
				vest_bob_from_0_to_20_at_period_2,
				vest_charlie_from_0_to_36_at_period_3,
				cliff_vest_dave_at_10,
				cliff_vest_eve_at_20,
				cliff_vest_ferdie_at_30,
			],
		},
	})
}

fn testnet_config_genesis() -> serde_json::Value {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	let accounts = get_well_known_accounts();

	const INITIAL_BALANCE: Balance = 1_000_000_000 * AJUNS;

	compose_genesis_config(Config {
		aura: AuraConfig {
			authorities: vec![
				// 5GRaE4bbSBxXtMmfGsWvycRSmLE1KA1ZUmAdyKQTyhFTFEy8
				hex!["c0db660b24bcf1b717a3a3e992cdd6d76710230848e664ddb4a06c1721df7c55"]
					.unchecked_into(),
			],
		},
		grandpa: GrandpaConfig {
			authorities: vec![
				// 5EpCKebe3iTSTUBMM4mFzwEKkbJBA3CdtGiVabsPjwMAyPsd
				(
					hex!["79a3d774934ac9660dd62e32b35679456d8836d61dc8537068d0559c0f4b566f"]
						.unchecked_into(),
					1,
				),
			],
			..Default::default()
		},
		sudo: SudoConfig { key: Some(accounts.alice.clone()) },
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig::default(),
		balances: BalancesConfig {
			balances: vec![
				(accounts.alice, INITIAL_BALANCE),
				(accounts.bob, INITIAL_BALANCE),
				(accounts.charlie, INITIAL_BALANCE),
				(accounts.dave, INITIAL_BALANCE),
				(accounts.eve, INITIAL_BALANCE),
				(accounts.ferdie, INITIAL_BALANCE),
				(accounts.alice_stash, INITIAL_BALANCE),
				(accounts.bob_stash, INITIAL_BALANCE),
				(accounts.charlie_stash, INITIAL_BALANCE),
				(accounts.dave_stash, INITIAL_BALANCE),
				(accounts.eve_stash, INITIAL_BALANCE),
				(accounts.ferdie_stash, INITIAL_BALANCE),
			],
		},
		assets: AssetsConfig::default(),
		vesting: VestingConfig::default(),
	})
}

// Composes config with defaults to return initial storage state for FRAME modules.
fn compose_genesis_config(config: Config) -> serde_json::Value {
	let wasm_binary = ajuna_solo_runtime::WASM_BINARY.expect(
		"Development wasm binary is not available. Please rebuild with SKIP_WASM_BUILD disabled.",
	);
	let Config { aura, grandpa, sudo, council, technical_committee, balances, assets, vesting } =
		config;

	serde_json::json!({
		"aura": aura,
		"grandpa": grandpa,
		"sudo": sudo,
		"council": council,
		"technical_committee": technical_committee,
		"balances": balances,
		"assets": assets,
		"vesting": vesting,
		// default config
		"system": SystemConfig { code: wasm_binary.to_vec(), ..Default::default() },
		"transaction_payment": Default::default(),
		"treasury": treasury: Default::default(),
		"democracy": democracy: Default::default(),
		"awesome_avatars": awesome_avatars: Default::default(),
	})
}
