// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    psp37,
    psp37::balances,
    traits::psp37::{
        extensions::mintable::*,
        *,
    },
};
pub use psp37::{
    Internal as _,
    Transfer as _,
};

use ink::{
    prelude::vec::Vec,
    storage::traits::{
        AutoStorableHint,
        ManualKey,
        Storable,
        StorableHint,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    OccupiedStorage,
    Storage,
};

use ink::prelude::vec;

impl<B, T> PSP37Mintable for T
where
    B: balances::BalancesManager,
    B: Storable
        + StorableHint<ManualKey<{ psp37::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ psp37::STORAGE_KEY }>>, Type = B>,
    T: Storage<psp37::Data<B>>,
    T: OccupiedStorage<{ psp37::STORAGE_KEY }, WithData = psp37::Data<B>>,
{
    default fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
        self._mint_to(to, ids_amounts)
    }

    default fn claim_0_token(&mut self) -> Result<(), PSP37Error> {
        
        let to = Self::env().caller();
        let balance = self.balance_of(to, Some(Id::U64(0)));

        if balance == 0 {
            self._mint_to(to,  vec![(Id::U64(0), 1)])
        } else {
            Err(PSP37Error::HasMinted)
        }

        
        
    }
}
