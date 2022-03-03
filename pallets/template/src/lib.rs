#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use serde::Deserialize;
    use serde::Serialize;
    use frame_support::traits::Randomness;
    use frame_support::sp_runtime::traits::Hash;
    use frame_support::inherent::Vec;
    use frame_support::ensure;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        // For random pet's gender
        type PetRandomness: Randomness<Self::Hash, Self::BlockNumber>;

        // Maximum pet an account can own
        #[pallet::constant]
        type MaxPetOwned: Get<u32>;
	}

    type AccountOf<T> = <T as frame_system::Config>::AccountId;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Pet<T: Config> {
        pub name: Vec<u8>,
        pub gender: Gender,
        pub owner: AccountOf<T>,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum Gender {
        Male,
        Female,
    }

    impl<T: Config> MaxEncodedLen for Pet<T> {
        fn max_encoded_len() -> usize {
            let len: usize = 4;
            len
        }
    }

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-itemsConfig
	pub type Something<T> = StorageValue<_, u32>;

    // Storage item to keep a count of all existing Pet.
	#[pallet::storage]
	#[pallet::getter(fn pet_cnt)]
	/// Keeps track of the number of Pets in existence.
	pub(super) type PetCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

        
    #[pallet::storage]
    #[pallet::getter(fn pets)]
    /// Stores a Pet, by storing the Pet object and associating it with its Pet ID.
    pub(super) type Pets<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::Hash,
        Pet<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pets_owned)]
    /// Keeps track of what accounts own what Pets.
    pub(super) type PetsOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<T::Hash, T::MaxPetOwned>,
        ValueQuery,
    >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

        /// A new Pet was successfully created. \[sender, pet_id\]
        Created(T::AccountId, T::Hash, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

        /// Handles arithmetic overflow when incrementing the Pet counter.
        PetCntOverflow,

        /// Handles checking whether the Pet exists.
        PetAlreadyExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn save_data(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

        /// Create a new unique Pet.
		///
		/// The actual pet creation is done in the `mint()` function.
		#[pallet::weight(100)]
		pub fn create_pet(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let pet_id = Self::mint(&sender, None, &name)?;
            
            // Logging to the console
            log::info!("A pet is born with ID: {:?}.", pet_id);

            Self::deposit_event(Event::Created(sender, pet_id, name));

			Ok(())
		}
	}

    //** Our helper functions.**//

    impl<T: Config> Pallet<T> {

        // ACTION #4: helper function for Pet struct
        fn gen_gender() -> Gender {
            let random = T::PetRandomness::random(&b"gender"[..]).0;
            match random.as_ref()[0] % 2 {
                0 => Gender::Male,
                _ => Gender::Female,
            }
        }

        pub fn mint(
            owner: &T::AccountId,
            gender: Option<Gender>,
            name: &Vec<u8>,
        ) -> Result<T::Hash, Error<T>> {
            let pet = Pet::<T> {
                name: name.clone(),
                gender: gender.unwrap_or_else(Self::gen_gender),
                owner: owner.clone(),
            };

            let pet_id = T::Hashing::hash_of(&name);

            // Performs this operation first as it may fail
            let new_cnt = Self::pet_cnt().checked_add(1).ok_or(<Error<T>>::PetCntOverflow)?;
            
            // Check Pet name is exist
            let existed = <Pets<T>>::contains_key(pet_id);
            ensure!(!existed, <Error<T>>::PetAlreadyExist);

            <Pets<T>>::insert(pet_id, pet);
            <PetCnt<T>>::put(new_cnt);
            Ok(pet_id)
        }
    }
}
