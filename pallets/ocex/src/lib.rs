// This file is part of Polkadex.

// Copyright (C) 2020-2021 Polkadex oü.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::StorageMap;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    traits::Get, PalletId,
};
use frame_system as system;
use frame_system::ensure_signed;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use sp_runtime::traits::AccountIdConversion;
use sp_std::prelude::*;

use polkadex_primitives::assets::AssetId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct LinkedAccount<T: Config> {
    prev: T::AccountId,
    next: Option<T::AccountId>,
    proxies: Vec<T::AccountId>,
}

impl<T: Config> LinkedAccount<T> {
    fn from(prev: T::AccountId) -> Self {
        LinkedAccount{
            prev: prev,
            next: None,
            proxies: vec![]
        }
    }
}

impl<T: Config> Default for LinkedAccount<T> {
    fn default() -> Self {
        LinkedAccount {
            prev: Module::<T>::get_genesis_acc(),
            next: None,
            proxies: vec![],
        }
    }
}

pub trait Config:
    system::Config + orml_tokens::Config + pallet_substratee_registry::Config
{
    /// Events
    type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;
    /// Bonding Account
    type OcexId: Get<PalletId>;
    /// LinkedList Genesis Account
    type GenesisAccount: Get<PalletId>;
    /// Currency for transfer currencies
    type Currency: MultiCurrencyExtended<
        Self::AccountId,
        CurrencyId = AssetId,
        Balance = Self::Balance,
    >;
    type ProxyLimit: Get<usize>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Config>::AccountId,
        <T as orml_tokens::Config>::Balance
    {
        TokenDeposited(AssetId, AccountId, Balance),
        TokenWithdrawn(AssetId, AccountId, Balance),
        MainAccountRegistered(AccountId),
        ProxyAdded(AccountId,AccountId),
        ProxyRemoved(AccountId,AccountId),
    }
);

// TODO: Implement a vec of MRENCLAVES set by governance

decl_error! {
    pub enum Error for Module<T: Config> {
        NotARegisteredEnclave,
        AlreadyRegistered,
        NotARegisteredMainAccount,
        ProxyLimitReached
    }
}

decl_storage! {
    trait Store for Module<T: Config> as OCEX {
        pub LastAccount: T::AccountId = T::GenesisAccount::get().into_account();
        pub MainAccounts get(fn get_main_accounts): map hasher(blake2_128_concat) T::AccountId => LinkedAccount<T>;
    }
}
decl_module! {
    pub struct Module<T: Config> for enum Call where
    origin: T::Origin {

        type Error = Error<T>;

        fn deposit_event() = default;

        /// Deposit
        #[weight = 10000]
        pub fn deposit(origin, asset_id:  AssetId, amount: T::Balance) -> DispatchResult{
            let from: T::AccountId = ensure_signed(origin)?;
            <T as Config>::Currency::transfer(asset_id, &from, &Self::get_account(), amount)?;
            Self::deposit_event(RawEvent::TokenDeposited(asset_id, from, amount));
            Ok(())
        }

        /// Release
        #[weight = 10000]
        pub fn release(origin, asset_id:  AssetId, amount: T::Balance, to: T::AccountId) -> DispatchResult{
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(pallet_substratee_registry::EnclaveIndex::<T>::contains_key(&sender), Error::<T>::NotARegisteredEnclave);
            // TODO: Check if the latest MRENCLAVE is registered by this sender
            // TODO: Handle software updated to enclave
            <T as Config>::Currency::transfer(asset_id, &Self::get_account(), &to, amount)?;
            Ok(())
        }

        /// Withdraw
        /// It helps to notify enclave about sender's intend to withdraw via on-chain
        #[weight = 10000]
        pub fn withdraw(origin, asset_id:  AssetId, to: T::AccountId,amount: T::Balance) -> DispatchResult{
            let _: T::AccountId = ensure_signed(origin)?;
            Self::deposit_event(RawEvent::TokenWithdrawn(asset_id, to, amount));
            Ok(())
        }

        /// Register MainAccount
        #[weight = 10000]
        pub fn register(origin) -> DispatchResult{
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(!<MainAccounts<T>>::contains_key(&sender), Error::<T>::AlreadyRegistered);
            Self::register_acc(sender.clone())?;
            Self::deposit_event(RawEvent::MainAccountRegistered(sender));
            Ok(())
        }

        /// Add Proxy
        #[weight = 10000]
        pub fn add_proxy(origin, proxy: T::AccountId) -> DispatchResult{
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(<MainAccounts<T>>::contains_key(&sender), Error::<T>::NotARegisteredMainAccount);
            Self::add_proxy_(sender.clone(),proxy.clone())?;
            Self::deposit_event(RawEvent::ProxyAdded(sender,proxy));
            Ok(())
        }

        /// Remove Proxy
        #[weight = 10000]
        pub fn remove_proxy(origin, proxy: T::AccountId) -> DispatchResult{
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(<MainAccounts<T>>::contains_key(&sender), Error::<T>::NotARegisteredMainAccount);
            Self::remove_proxy_(sender.clone(),proxy.clone())?;
            Self::deposit_event(RawEvent::ProxyRemoved(sender,proxy));
            Ok(())
        }

    }
}

impl<T: Config> Module<T> {
    // Note add_proxy doesn't check if given main or proxy is already registered
    pub fn add_proxy_(main: T::AccountId, proxy: T::AccountId) -> Result<(), Error<T>> {
        let mut acc: LinkedAccount<T> = <MainAccounts<T>>::get(&main);
        if acc.proxies.len() < T::ProxyLimit::get() {
            acc.proxies.push(proxy);
            <MainAccounts<T>>::insert(main, acc);
        } else {
            return Err(Error::<T>::ProxyLimitReached);
        }
        Ok(())
    }

    // Note remove_proxy doesn't check if given main or proxy is already registered
    pub fn remove_proxy_(main: T::AccountId, proxy: T::AccountId) -> Result<(), Error<T>> {
        <MainAccounts<T>>::try_mutate(main.clone(), |ref mut linked_account: &mut LinkedAccount<T>| {
            let index = linked_account.proxies.iter().position(|x| *x == proxy).unwrap();
            linked_account.proxies.remove(index);
            Ok(())
        })
    }

    pub fn get_account() -> T::AccountId {
        T::OcexId::get().into_account()
    }

    pub fn get_genesis_acc() -> T::AccountId {
        T::GenesisAccount::get().into_account()
    }

    pub fn register_acc(sender: T::AccountId) -> Result<(), Error<T>> {
        let last_account: T::AccountId = <LastAccount<T>>::get();
        <MainAccounts<T>>::try_mutate(last_account.clone(), |ref mut last_linked_account| {
            let new_linked_account: LinkedAccount<T> = LinkedAccount::from(last_account);
            <MainAccounts<T>>::insert(&sender, new_linked_account);
            <LastAccount<T>>::put(&sender);
            last_linked_account.next = Some(sender);
            Ok(())
        })
    }
}