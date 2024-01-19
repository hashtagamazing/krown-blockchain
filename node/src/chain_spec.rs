use hex_literal::hex;
use krown_node_runtime::{
	AccountId, BabeConfig, BalancesConfig, RuntimeGenesisConfig, Signature,
	SudoConfig, SystemConfig, WASM_BINARY, BABE_GENESIS_EPOCH_CONFIG, SessionConfig, StakingConfig, SessionKeys,
	constants::currency::*, StakerStatus, MaxNominations,ImOnlineConfig,
};

use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sc_telemetry::TelemetryEndpoints;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use node_primitives::*;

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	// BASE KROWN CONFIG
	let mut properties = Properties::new();
	properties.insert("tokenName".into(), "Krown".into());
	properties.insert("tokenSymbol".into(), "KROWN".into());
	properties.insert("tokenDecimals".into(), 8.into());
	properties.insert("ss58Format".into(), 42.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	// BASE KROWN CONFIG
	let mut properties = Properties::new();
	properties.insert("tokenName".into(), "Krown".into());
	properties.insert("tokenSymbol".into(), "KROWN".into());
	properties.insert("tokenDecimals".into(), 8.into());
	properties.insert("ss58Format".into(), 42.into());
	
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		Some(properties),
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

    // BASE KROWN CONFIG
    let mut properties = Properties::new();
    properties.insert("tokenName".into(), "Krown".into());
    properties.insert("tokenSymbol".into(), "KROWN".into());
    properties.insert("tokenDecimals".into(), 8.into());
    properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		"Krown Blockchain",
		"krown_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		Some(properties),
		Default::default(),
	)
}

fn staging_network_config_genesis() -> RuntimeGenesisConfig {
	let wasm_binary = WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	);

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// Validator 1 Stash: 5FWieSUnH1TeP5VtBXkqLDYFTaED9tJBNXCBRCt91tZkEHmJ
			hex!["988bbcc7f6eee10fd30eb62cf34bc88ae5832ee88a6cc73f92ae15179197b82e"].into(),
			// Validator 1 Controller: 5Ef6np9CT8qPSDRkt45eRXUU4gWJfStTVmCDMkDfCkhSHyr2
			hex!["72b4030599be2dfc2ea1bb8f496c7e16302f62461ed8684f0060a7a7a329b800"].into(),
			// Validator 1 Babe: 5ES5r17NYsvZfc6rTiC5PktUtNqq8iWjKyxBJh5zvEDXR7cy
			hex!["68c6a3ed7b11ad83e255fab6571fda8b01c41b1d878eb8c2c75a9e1741b45e2b"]
				.unchecked_into(),
			// Validator 1 Grandpa: 5EwaLZdhvHxCP3QSTBB4aUzRUe14jKofqQ6G6Ve25CrRB29V
			hex!["7f44abcef47499ea1c3cac70f6e2bfda50e0b5d9f9f9e7715fc9a28fba7e010b"]
				.unchecked_into(),
			// Validator 1 ImOnline: 5F7AKFZDRsba6z34youyfztnuCRSFzRvrU3rrjM4rQV27jod
			hex!["86944201bc77bdc6ea29ae0760e999fed37f1aeb63a7798824430ed9c9b12b0b"]
				.unchecked_into(),
		),
		(
			// Validator 2 Stash: 5GuWELet7e4MXXVp7QjNoWEnT7W1zAkzfoWXVS4ABisqPtXW
			hex!["d628d21fc0f9c9616651abd8948d18ec9e8d40260dde6adc54a58a1a50ff4e30"].into(),
			// Validator 2 Controller: 5DLwSwPQiQL8YvVzSFMXbEWnUkdFnE9yNPf8Az4Q5WqMbFLh
			hex!["389de92c1504df1257a9413ef1b9b2dff455263e09eb634371319fff0d2aa06d"].into(),
			// Validator 2 Babe: 5HThzu1Nqpb5cno7PDjnoBwsptJrzCJFCUPCttMuQjgkhD5j
			hex!["eeb84a063edd227579363b534a3b5a5e17949255a899a7ad0e981c6774d2da2e"]
				.unchecked_into(),
			// Validator 2 Grandpa: 5DcJsf87Gu9mQa5fAiEr7T9oeieFZ4i2VoPrZTS9JyXoyW7A
			hex!["4456b80aa44e2f6cc0a760f9726cf0ff1c8ecfb55a5676fbd5ce76ebf84cca06"]
				.unchecked_into(),
			// Validator 2 ImOnline: 5GrpjxFRoa8uoRB18vqdTTcy4c3gYHJFyXs5mWRjqa4CmfNc
			hex!["d41d6591c14a822a3c6638d0ab23ecb50b83024543ffd4e9520a014bf8dce568"]
				.unchecked_into(),
		),
		(
			// Validator 3 Stash: 5DiCLeo8GwHrRPCKr2SP3qMBr8iAUUGAJmAFqoYXNLRhnP7w
			hex!["48d431a8c11e1ae049ff3b3d8dc3cb09db005eabf6ec05bb03da6d9c3253c968"].into(),
			// Validator 3 Controller: 5HQTdetJYBbrc89TU9tnEM4mNy72wH9hRDCZBXGEiZKCVdkY
			hex!["ec3e311871eba3e3865480f18bfc83c104892e29b4d539651d47770f65da7a6b"].into(),
			// Validator 3 Babe: 5DHkSxDw3sS8A1JDiRVeWd3vVUMViYjGhuev3HCZY2uqb7PJ
			hex!["362f256a3a3df0af6643341961886a3b07d86b7d2f1eeecec6bfb9007451676c"]
				.unchecked_into(),
			// Validator 3 Grandpa: 5DqeWDS8jGnxeriH61qYiUK2VsAqoiEJDUzZabjEPeCttqzb
			hex!["4e82fd99fe8184cf85a12e1302eece9925b66672238bb2f4a7642b7038758a30"]
				.unchecked_into(),
			// Validator 3 ImOnline: 5EJPR1AnT2KbGvWTEfbu3Yuw7di8MaYybff5ea3xHJday7VF
			hex!["62e7d1cfc1351a6419fcea05ffb401c09237e901c25a195c1f90224bc9ddf525"]
				.unchecked_into(),
		),
		(
			// Validator 4 Stash: 5FgWPHdNSHowTaK6PBGRG1sMs239htQWnPjNViinKwaVDPZz
			hex!["a002ea9a3f29e6f5b2cda612eb6d9e09c78a9d83d00cfa2e3ada65b74099f70a"].into(),
			// Validator 4 Controller: 5D2De1YmSrRAPzCGVnRa6j4Gqy2EChxaAB193RVcu4mSTKu1
			hex!["2a56c0656cc94ea9e93a0976441d793bd5881a680012995bc3dc0e9e8bad2c41"].into(),
			// Validator 4 Babe: 5DtH44X6wJL45pDgLyQhmMWLvQUpuRUNRajkSMBafWokRdQm
			hex!["5084843f4f5044346fc6411ad49e7fff1df0003442f31617908cf21a4d74b86e"]
				.unchecked_into(),
			// Validator 4 Grandpa: 5GjWxWJyCNDFENxXAwxUEZHRKaaf25yBKHP6ewy8essihW33
			hex!["ce8ad067b1ff95f42aea58ecbda88be9a26fe3df1d070677bfe8d5cf746e7cbd"]
				.unchecked_into(),
			// Validator 4 ImOnline: 5GL54KhhcW3SzwUZDJxx3EX4cdR5bKUEFcb1YDtxcbsJPv94
			hex!["bca8fbcd6131c9d44a5862e55e316d68e383f71f71a5869375ef2b3ebd545203"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// Sudo: 5HWP2qwU8dGBPkW4G7cc8xxxACSyfp1jYQF3eWHaY8Lh8i39
		"f0c22dbdecda22f4d84c9c341effa7ae5265f6826db33ac18c91df8cdaa3f633"
	]
		.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		wasm_binary,
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts,
		true,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> RuntimeGenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000 * KROWN;
	const STASH: Balance = ENDOWMENT / 50000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();
	RuntimeGenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			// Configure endowed accounts
			balances: endowed_accounts.iter().cloned().map(|k| (k, 200 * KROWN)).collect(),
		},
		babe: BabeConfig {
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		grandpa: Default::default(),
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
