//! Unit tests for xcm-support implementations.

#![cfg(test)]

use super::*;

use orml_traits::{location::AbsoluteReserveProvider, location::RelativeLocations, ConcreteFungibleAsset};

#[derive(Debug, PartialEq, Eq)]
pub enum TestCurrencyId {
	TokenA,
	TokenB,
	RelayChainToken,
}

pub struct CurrencyIdConvert;
impl Convert<MultiLocation, Option<TestCurrencyId>> for CurrencyIdConvert {
	fn convert(l: MultiLocation) -> Option<TestCurrencyId> {
		use TestCurrencyId::*;
		let mut token_a = [0u8; 32];
		let mut token_b = [0u8; 32];
		let mut key: Vec<u8> = "TokenA".into();
		for (i, byte) in key.iter().enumerate() {
			token_a[i] = *byte
		}
		key = "TokenB".into();
		for (i, byte) in key.iter().enumerate() {
			token_b[i] = *byte
		}

		if l == MultiLocation::parent() {
			return Some(RelayChainToken);
		}
		if l == MultiLocation::sibling_parachain_general_key(1, token_a) {
			return Some(TokenA);
		}
		if l == MultiLocation::sibling_parachain_general_key(2, token_b) {
			return Some(TokenB);
		}
		None
	}
}

type MatchesCurrencyId = IsNativeConcrete<TestCurrencyId, CurrencyIdConvert>;

#[test]
fn is_native_concrete_matches_native_currencies() {
	let mut token_a = [0u8; 32];
		let mut token_b = [0u8; 32];
		let mut key: Vec<u8> = "TokenA".into();
		for (i, byte) in key.iter().enumerate() {
			token_a[i] = *byte
		}
		key = "TokenB".into();
		for (i, byte) in key.iter().enumerate() {
			token_b[i] = *byte
		}
	assert_eq!(
		MatchesCurrencyId::matches_fungible(&MultiAsset::parent_asset(100)),
		Some(100),
	);

	assert_eq!(
		MatchesCurrencyId::matches_fungible(&MultiAsset::sibling_parachain_asset(1, token_a, 100)),
		Some(100),
	);

	assert_eq!(
		MatchesCurrencyId::matches_fungible(&MultiAsset::sibling_parachain_asset(2, token_b, 100)),
		Some(100),
	);
}

#[test]
fn is_native_concrete_does_not_matches_non_native_currencies() {
	let mut token_c = [0u8; 32];
		let mut token_b = [0u8; 32];
		let mut key: Vec<u8> = "TokenC".into();
		for (i, byte) in key.iter().enumerate() {
			token_c[i] = *byte
		}
		key = "TokenB".into();
		for (i, byte) in key.iter().enumerate() {
			token_b[i] = *byte
		}
	assert!(
		<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&MultiAsset::sibling_parachain_asset(
			2,
			token_c,
			100
		))
		.is_none()
	);
	assert!(
		<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&MultiAsset::sibling_parachain_asset(
			1,
			token_b,
			100
		))
		.is_none()
	);
	assert!(
		<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&MultiAsset {
			fun: Fungible(100),
			id: Concrete(MultiLocation::new(1, X1(GeneralKey(token_b)))),
		})
		.is_none()
	);
}

#[test]
fn multi_native_asset() {
	let mut token_a = [0u8; 32];
	let mut key: Vec<u8> = "TokenA".into();
	for (i, byte) in key.iter().enumerate() {
		token_a[i] = *byte
	}
	assert!(MultiNativeAsset::<AbsoluteReserveProvider>::contains(
		&MultiAsset {
			fun: Fungible(10),
			id: Concrete(MultiLocation::parent())
		},
		&Parent.into()
	));
	assert!(MultiNativeAsset::<AbsoluteReserveProvider>::contains(
		&MultiAsset::sibling_parachain_asset(1, token_a, 100),
		&MultiLocation::new(1, X1(Parachain(1))),
	));
	assert!(!MultiNativeAsset::<AbsoluteReserveProvider>::contains(
		&MultiAsset::sibling_parachain_asset(1, token_a, 100),
		&MultiLocation::parent(),
	));
}
