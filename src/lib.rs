#[macro_use]
extern crate bitmask;
use codec::{Encode, Decode};


bitmask! {
	/// Reasons for moving funds out of an account.
	#[derive(Encode, Decode)]
	pub mask Withdraw2Reasons: i8 where

	/// Reason for moving funds out of an account.
	#[derive(Encode, Decode)]
	flags Withdraw2Reason {
		/// In order to pay for (system) transaction costs.
		TransactionPayment = 0b00000001,
		/// In order to transfer ownership.
		Transfer = 0b00000010,
		/// In order to reserve some funds for a later return or repatriation.
		Reserve = 0b00000100,
		/// In order to pay some other (higher-level) fees.
		Fee = 0b00001000,
		/// In order to tip a validator for transaction inclusion.
		Tip = 0b00010000,
	}
}
pub fn main() -> i8{
    Withdraw2Reason::Transfer as i8
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(main(), main2());
    }
}
