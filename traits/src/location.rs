use xcm::v1::{
	AssetId, Fungibility,
	Junction::{self, *},
	Junctions::*,
	MultiAsset, MultiLocation,
};

pub trait Parse {
	/// Returns the "chain" location part. It could be parent, sibling
	/// parachain, or child parachain.
	fn chain_part(&self) -> Option<MultiLocation>;
	/// Returns "non-chain" location part.
	fn non_chain_part(&self) -> Option<MultiLocation>;
}

/// We can now just focus on the non-parent part
fn is_chain_junction(junction: Option<&Junction>) -> bool {
	matches!(junction, Some(Parachain(_)))
}

impl Parse for MultiLocation {
	fn chain_part(&self) -> Option<MultiLocation> {
		let first_interior = self.first_interior()?;
		let parents = self.parent_count();
		match (parents, first_interior.clone()) {
			(0, Parachain(id)) => Some(MultiLocation::new(0, X1(Parachain(id)))),
			(1, Parachain(id)) => Some(MultiLocation::new(1, X1(Parachain(id)))),
			(1, _) => Some(MultiLocation::parent()),
			_ => None,
		}
	}

	fn non_chain_part(&self) -> Option<MultiLocation> {
		let mut location = self.clone();
		while is_chain_junction(location.first_interior()) {
			let _ = location.take_first_interior();
		}

		if location.first_interior() != None {
			Some(MultiLocation {
				parents: 0,
				interior: location.interior
			})
		} else {
			None
		}
	}
}

pub trait Reserve {
	/// Returns assets reserve location.
	fn reserve(&self) -> Option<MultiLocation>;
}

impl Reserve for MultiAsset {
	fn reserve(&self) -> Option<MultiLocation> {
		match (&self.id, &self.fun) {
			(AssetId::Concrete(id), Fungibility::Fungible(_)) => id.chain_part(),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const PARACHAIN: Junction = Parachain(1);
	const GENERAL_INDEX: Junction = GeneralIndex(1);

	fn concrete_fungible(id: MultiLocation) -> MultiAsset {
		MultiAsset {
			id: AssetId::Concrete(id),
			fun: Fungibility::Fungible(1)
		} 
	}

	#[test]
	fn parent_as_reserve_chain() {
		assert_eq!(
			concrete_fungible(MultiLocation {
				parents: 1,
				interior: X1(GENERAL_INDEX)
			}).reserve(),
			Some(MultiLocation::parent())
		);
	}

	#[test]
	fn sibling_parachain_as_reserve_chain() {
		assert_eq!(
			concrete_fungible(MultiLocation {
				parents: 1,
				interior: X2(PARACHAIN, GENERAL_INDEX)
			}).reserve(),
			Some(MultiLocation::new(1, X1(PARACHAIN)))
		);
	}

	#[test]
	fn child_parachain_as_reserve_chain() {
		assert_eq!(
			concrete_fungible(MultiLocation {
				parents: 0,
				interior: X2(PARACHAIN, GENERAL_INDEX)
			}).reserve(),
			Some(MultiLocation::new(0, X1(PARACHAIN)))
		);
	}

	#[test]
	fn no_reserve_chain() {
		assert_eq!(
			concrete_fungible(MultiLocation {
				parents: 0,
				interior: X1(GeneralKey("DOT".into()))
			}).reserve(),
			None
		);
	}

	#[test]
	fn non_chain_part_works() {
		assert_eq!(MultiLocation::parent().non_chain_part(), None);
		assert_eq!(MultiLocation {
			parents: 1,
			interior: X1(PARACHAIN)
		}.non_chain_part(), None);

		assert_eq!(MultiLocation {
			parents: 0,
			interior: X1(PARACHAIN)
		}.non_chain_part(), None);

		assert_eq!(MultiLocation {
			parents: 1,
			interior: X1(GENERAL_INDEX)
		}.non_chain_part(), Some(GENERAL_INDEX.into()));

		assert_eq!(MultiLocation {
			parents: 1,
			interior: X2(GENERAL_INDEX, GENERAL_INDEX)
		}.non_chain_part(), Some((GENERAL_INDEX, GENERAL_INDEX).into()));

		assert_eq!(MultiLocation {
			parents: 1,
			interior: X2(PARACHAIN, GENERAL_INDEX)
		}.non_chain_part(), Some(GENERAL_INDEX.into()));

		assert_eq!(MultiLocation {
			parents: 0,
			interior: X2(PARACHAIN, GENERAL_INDEX)
		}.non_chain_part(), Some(GENERAL_INDEX.into()));
	}
}
