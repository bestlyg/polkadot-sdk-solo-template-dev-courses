#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use pallet::*;
/**The `pallet` module in each FRAME pallet hosts the most important items needed
to construct this pallet.

The main components of this pallet are:
- [`Pallet`], which implements all of the dispatchable extrinsics of the pallet, among
other public functions.
	- The subset of the functions that are dispatchable can be identified either in the
	[`dispatchables`] module or in the [`Call`] enum.
- [`storage_types`], which contains the list of all types that are representing a
storage item. Otherwise, all storage items are listed among [*Type Definitions*](#types).
- [`Config`], which contains the configuration trait of this pallet.
- [`Event`] and [`Error`], which are listed among the [*Enums*](#enums).
		*/
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::{OriginFor, *};
    /**
Configuration trait of this pallet.

The main purpose of this trait is to act as an interface between this pallet and the runtime in
which it is embedded in. A type, function, or constant in this trait is essentially left to be
configured by the runtime that includes this pallet.

Consequently, a runtime that wants to include this pallet must implement this trait.*/
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MaxClaimLength: Get<u32>;
    }
    /**
				The `Pallet` struct, the main type that implements traits and standalone
				functions within the pallet.
			*/
    pub struct Pallet<T>(frame_support::__private::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        #[automatically_derived]
        impl<T> ::core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(::core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> ::core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        #[automatically_derived]
        impl<T> ::core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        impl<T> ::core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    #[allow(type_alias_bounds)]
    ///
    ///Storage type is [`StorageMap`] with key type `BoundedVec < u8, T :: MaxClaimLength >` and value type `(T :: AccountId, BlockNumberFor < T >)`.
    pub type Proofs<T: Config> = StorageMap<
        _GeneratedPrefixForStorageProofs<T>,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, BlockNumberFor<T>),
    >;
    ///The `Event` enum of this pallet
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimTransferred(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(::core::marker::PhantomData<(T)>, frame_support::Never),
    }
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::ClaimCreated(ref _0, ref _1) => {
                        Self::ClaimCreated(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                        )
                    }
                    Self::ClaimRevoked(ref _0, ref _1) => {
                        Self::ClaimRevoked(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                        )
                    }
                    Self::ClaimTransferred(ref _0, ref _1, ref _2) => {
                        Self::ClaimTransferred(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                            ::core::clone::Clone::clone(_2),
                        )
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                        )
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> ::core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (
                        Self::ClaimCreated(_0, _1),
                        Self::ClaimCreated(_0_other, _1_other),
                    ) => true && _0 == _0_other && _1 == _1_other,
                    (
                        Self::ClaimRevoked(_0, _1),
                        Self::ClaimRevoked(_0_other, _1_other),
                    ) => true && _0 == _0_other && _1 == _1_other,
                    (
                        Self::ClaimTransferred(_0, _1, _2),
                        Self::ClaimTransferred(_0_other, _1_other, _2_other),
                    ) => true && _0 == _0_other && _1 == _1_other && _2 == _2_other,
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::ClaimCreated { .. }, Self::ClaimRevoked { .. }) => false,
                    (Self::ClaimCreated { .. }, Self::ClaimTransferred { .. }) => false,
                    (Self::ClaimCreated { .. }, Self::__Ignore { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::ClaimTransferred { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::__Ignore { .. }) => false,
                    (Self::ClaimTransferred { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::ClaimTransferred { .. }, Self::ClaimRevoked { .. }) => false,
                    (Self::ClaimTransferred { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimRevoked { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimTransferred { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self::ClaimCreated(ref _0, ref _1) => {
                        fmt.debug_tuple("Event::ClaimCreated")
                            .field(&_0)
                            .field(&_1)
                            .finish()
                    }
                    Self::ClaimRevoked(ref _0, ref _1) => {
                        fmt.debug_tuple("Event::ClaimRevoked")
                            .field(&_0)
                            .field(&_1)
                            .finish()
                    }
                    Self::ClaimTransferred(ref _0, ref _1, ref _2) => {
                        fmt.debug_tuple("Event::ClaimTransferred")
                            .field(&_0)
                            .field(&_1)
                            .field(&_2)
                            .finish()
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Event::__Ignore").field(&_0).field(&_1).finish()
                    }
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
        {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        Event::ClaimCreated(ref aa, ref ba) => {
                            0_usize
                                .saturating_add(::codec::Encode::size_hint(aa))
                                .saturating_add(::codec::Encode::size_hint(ba))
                        }
                        Event::ClaimRevoked(ref aa, ref ba) => {
                            0_usize
                                .saturating_add(::codec::Encode::size_hint(aa))
                                .saturating_add(::codec::Encode::size_hint(ba))
                        }
                        Event::ClaimTransferred(ref aa, ref ba, ref ca) => {
                            0_usize
                                .saturating_add(::codec::Encode::size_hint(aa))
                                .saturating_add(::codec::Encode::size_hint(ba))
                                .saturating_add(::codec::Encode::size_hint(ca))
                        }
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::ClaimCreated(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::ClaimRevoked(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::ClaimTransferred(ref aa, ref ba, ref ca) => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Encode,
        {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Event<T>
        where
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Decode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Decode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Decode,
            BoundedVec<u8, T::MaxClaimLength>: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Event`, failed to read variant byte")
                    })?
                {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::ClaimCreated(
                                    {
                                        let __codec_res_edqy = <T::AccountId as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ClaimCreated.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                    {
                                        let __codec_res_edqy = <BoundedVec<
                                            u8,
                                            T::MaxClaimLength,
                                        > as ::codec::Decode>::decode(__codec_input_edqy);
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ClaimCreated.1`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                ),
                            )
                        })();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 1usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::ClaimRevoked(
                                    {
                                        let __codec_res_edqy = <T::AccountId as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ClaimRevoked.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                    {
                                        let __codec_res_edqy = <BoundedVec<
                                            u8,
                                            T::MaxClaimLength,
                                        > as ::codec::Decode>::decode(__codec_input_edqy);
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ClaimRevoked.1`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                ),
                            )
                        })();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 2usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::ClaimTransferred(
                                    {
                                        let __codec_res_edqy = <T::AccountId as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ClaimTransferred.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                    {
                                        let __codec_res_edqy = <T::AccountId as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ClaimTransferred.1`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                    {
                                        let __codec_res_edqy = <BoundedVec<
                                            u8,
                                            T::MaxClaimLength,
                                        > as ::codec::Decode>::decode(__codec_input_edqy);
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ClaimTransferred.2`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                ),
                            )
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Event`, variant doesn't exist"),
                            )
                        })();
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Event<T>
        where
            T::AccountId: ::scale_info::TypeInfo + 'static,
            BoundedVec<u8, T::MaxClaimLength>: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            BoundedVec<u8, T::MaxClaimLength>: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            BoundedVec<u8, T::MaxClaimLength>: ::scale_info::TypeInfo + 'static,
            ::core::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new_with_replace(
                            "Event",
                            "pallet_poe::pallet",
                            &[],
                        ),
                    )
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(&["The `Event` enum of this pallet"])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "ClaimCreated",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| f.ty::<T::AccountId>().type_name("T::AccountId"))
                                                .field(|f| {
                                                    f
                                                        .ty::<BoundedVec<u8, T::MaxClaimLength>>()
                                                        .type_name("BoundedVec<u8, T::MaxClaimLength>")
                                                }),
                                        )
                                },
                            )
                            .variant(
                                "ClaimRevoked",
                                |v| {
                                    v
                                        .index(1usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| f.ty::<T::AccountId>().type_name("T::AccountId"))
                                                .field(|f| {
                                                    f
                                                        .ty::<BoundedVec<u8, T::MaxClaimLength>>()
                                                        .type_name("BoundedVec<u8, T::MaxClaimLength>")
                                                }),
                                        )
                                },
                            )
                            .variant(
                                "ClaimTransferred",
                                |v| {
                                    v
                                        .index(2usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| f.ty::<T::AccountId>().type_name("T::AccountId"))
                                                .field(|f| f.ty::<T::AccountId>().type_name("T::AccountId"))
                                                .field(|f| {
                                                    f
                                                        .ty::<BoundedVec<u8, T::MaxClaimLength>>()
                                                        .type_name("BoundedVec<u8, T::MaxClaimLength>")
                                                }),
                                        )
                                },
                            ),
                    )
            }
        }
    };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    ///The `Error` enum of this pallet.
    pub enum Error<T> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::__private::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Encode for Error<T> {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        Error::ProofAlreadyExist => 0_usize,
                        Error::ClaimNotExist => 0_usize,
                        Error::NotClaimOwner => 0_usize,
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Error::ProofAlreadyExist => {
                        #[allow(clippy::unnecessary_cast)]
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    Error::ClaimNotExist => {
                        #[allow(clippy::unnecessary_cast)]
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    }
                    Error::NotClaimOwner => {
                        #[allow(clippy::unnecessary_cast)]
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T> ::codec::EncodeLike for Error<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Decode for Error<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Error`, failed to read variant byte")
                    })?
                {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Error::<T>::ProofAlreadyExist)
                        })();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 1usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Error::<T>::ClaimNotExist)
                        })();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 2usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Error::<T>::NotClaimOwner)
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Error`, variant doesn't exist"),
                            )
                        })();
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T> ::scale_info::TypeInfo for Error<T>
        where
            frame_support::__private::sp_std::marker::PhantomData<
                (T),
            >: ::scale_info::TypeInfo + 'static,
            T: 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new_with_replace(
                            "Error",
                            "pallet_poe::pallet",
                            &[],
                        ),
                    )
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(&["The `Error` enum of this pallet."])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "ProofAlreadyExist",
                                |v| v.index(0usize as ::core::primitive::u8),
                            )
                            .variant(
                                "ClaimNotExist",
                                |v| v.index(1usize as ::core::primitive::u8),
                            )
                            .variant(
                                "NotClaimOwner",
                                |v| v.index(2usize as ::core::primitive::u8),
                            ),
                    )
            }
        }
    };
    const _: () = {
        impl<T> frame_support::traits::PalletError for Error<T> {
            const MAX_ENCODED_SIZE: usize = 1;
        }
    };
    impl<T: Config> Pallet<T> {
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let sender = ensure_signed(origin)?;
                {
                    if !!Proofs::<T>::contains_key(&claim) {
                        {
                            return Err(Error::<T>::ProofAlreadyExist.into());
                        };
                    }
                };
                Proofs::<
                    T,
                >::insert(
                    &claim,
                    (sender.clone(), frame_system::Pallet::<T>::block_number()),
                );
                Self::deposit_event(Event::ClaimCreated(sender, claim));
                Ok(().into())
            })
        }
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let sender = ensure_signed(origin)?;
                let (owner, _) = Proofs::<T>::get(&claim)
                    .ok_or(Error::<T>::ClaimNotExist)?;
                {
                    if !(owner == sender) {
                        {
                            return Err(Error::<T>::NotClaimOwner.into());
                        };
                    }
                };
                Proofs::<T>::remove(&claim);
                Self::deposit_event(Event::ClaimRevoked(sender, claim));
                Ok(().into())
            })
        }
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            to: T::AccountId,
        ) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let from = ensure_signed(origin)?;
                let (owner, block_number) = Proofs::<T>::get(&claim)
                    .ok_or(Error::<T>::ClaimNotExist)?;
                {
                    if !(owner == from) {
                        {
                            return Err(Error::<T>::NotClaimOwner.into());
                        };
                    }
                };
                Proofs::<T>::insert(&claim, (to.clone(), block_number));
                Self::deposit_event(Event::ClaimTransferred(from, to, claim));
                Ok(().into())
            })
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_documentation_metadata() -> frame_support::__private::sp_std::vec::Vec<
            &'static str,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata() -> frame_support::__private::sp_std::vec::Vec<
            frame_support::__private::metadata_ir::PalletConstantMetadataIR,
        > {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        frame_support::__private::metadata_ir::PalletConstantMetadataIR {
                            name: "MaxClaimLength",
                            ty: frame_support::__private::scale_info::meta_type::<u32>(),
                            value: {
                                let value = <<T as Config>::MaxClaimLength as frame_support::traits::Get<
                                    u32,
                                >>::get();
                                frame_support::__private::codec::Encode::encode(&value)
                            },
                            docs: ::alloc::vec::Vec::new(),
                        }
                    },
                ]),
            )
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<
            frame_support::__private::metadata_ir::PalletErrorMetadataIR,
        > {
            Some(frame_support::__private::metadata_ir::PalletErrorMetadataIR {
                ty: frame_support::__private::scale_info::meta_type::<Error<T>>(),
            })
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        type InCodeStorageVersion = frame_support::traits::NoStorageVersionSet;
        fn in_code_storage_version() -> Self::InCodeStorageVersion {
            core::default::Default::default()
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version: frame_support::traits::StorageVersion = core::default::Default::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name_hash() -> [u8; 16] {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name_hash::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn module_name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::module_name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 0u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn infos() -> frame_support::__private::sp_std::vec::Vec<
            frame_support::traits::PalletInfoData,
        > {
            use frame_support::traits::PalletInfoAccess;
            let item = frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([item]))
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::__private::sp_std::vec::Vec<
            frame_support::traits::StorageInfo,
        > {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info = <Proofs<
                    T,
                > as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    use frame_support::traits::{
        StorageInfoTrait, TrackedStorageKey, WhitelistedStorageKeys,
    };
    impl<T: Config> WhitelistedStorageKeys for Pallet<T> {
        fn whitelisted_storage_keys() -> frame_support::__private::sp_std::vec::Vec<
            TrackedStorageKey,
        > {
            use frame_support::__private::sp_std::vec;
            ::alloc::vec::Vec::new()
        }
    }
    #[doc(hidden)]
    mod warnings {}
    #[allow(unused_imports)]
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains a variant per dispatchable extrinsic that this pallet has.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(::core::marker::PhantomData<(T,)>, frame_support::Never),
        #[codec(index = 0u8)]
        create_claim { #[allow(missing_docs)] claim: BoundedVec<u8, T::MaxClaimLength> },
        #[codec(index = 1u8)]
        revoke_claim { #[allow(missing_docs)] claim: BoundedVec<u8, T::MaxClaimLength> },
        #[codec(index = 2u8)]
        transfer_claim {
            #[allow(missing_docs)]
            claim: BoundedVec<u8, T::MaxClaimLength>,
            #[allow(missing_docs)]
            to: T::AccountId,
        },
    }
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Call::__Ignore").field(&_0).field(&_1).finish()
                    }
                    Self::create_claim { ref claim } => {
                        fmt.debug_struct("Call::create_claim")
                            .field("claim", &claim)
                            .finish()
                    }
                    Self::revoke_claim { ref claim } => {
                        fmt.debug_struct("Call::revoke_claim")
                            .field("claim", &claim)
                            .finish()
                    }
                    Self::transfer_claim { ref claim, ref to } => {
                        fmt.debug_struct("Call::transfer_claim")
                            .field("claim", &claim)
                            .field("to", &to)
                            .finish()
                    }
                }
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                        )
                    }
                    Self::create_claim { ref claim } => {
                        Self::create_claim {
                            claim: ::core::clone::Clone::clone(claim),
                        }
                    }
                    Self::revoke_claim { ref claim } => {
                        Self::revoke_claim {
                            claim: ::core::clone::Clone::clone(claim),
                        }
                    }
                    Self::transfer_claim { ref claim, ref to } => {
                        Self::transfer_claim {
                            claim: ::core::clone::Clone::clone(claim),
                            to: ::core::clone::Clone::clone(to),
                        }
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> ::core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::create_claim { claim }, Self::create_claim { claim: _0 }) => {
                        true && claim == _0
                    }
                    (Self::revoke_claim { claim }, Self::revoke_claim { claim: _0 }) => {
                        true && claim == _0
                    }
                    (
                        Self::transfer_claim { claim, to },
                        Self::transfer_claim { claim: _0, to: _1 },
                    ) => true && claim == _0 && to == _1,
                    (Self::__Ignore { .. }, Self::create_claim { .. }) => false,
                    (Self::__Ignore { .. }, Self::revoke_claim { .. }) => false,
                    (Self::__Ignore { .. }, Self::transfer_claim { .. }) => false,
                    (Self::create_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::create_claim { .. }, Self::revoke_claim { .. }) => false,
                    (Self::create_claim { .. }, Self::transfer_claim { .. }) => false,
                    (Self::revoke_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::revoke_claim { .. }, Self::create_claim { .. }) => false,
                    (Self::revoke_claim { .. }, Self::transfer_claim { .. }) => false,
                    (Self::transfer_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::transfer_claim { .. }, Self::create_claim { .. }) => false,
                    (Self::transfer_claim { .. }, Self::revoke_claim { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        Call::create_claim { ref claim } => {
                            0_usize.saturating_add(::codec::Encode::size_hint(claim))
                        }
                        Call::revoke_claim { ref claim } => {
                            0_usize.saturating_add(::codec::Encode::size_hint(claim))
                        }
                        Call::transfer_claim { ref claim, ref to } => {
                            0_usize
                                .saturating_add(::codec::Encode::size_hint(claim))
                                .saturating_add(::codec::Encode::size_hint(to))
                        }
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create_claim { ref claim } => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(claim, __codec_dest_edqy);
                    }
                    Call::revoke_claim { ref claim } => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(claim, __codec_dest_edqy);
                    }
                    Call::transfer_claim { ref claim, ref to } => {
                        __codec_dest_edqy.push_byte(2u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(claim, __codec_dest_edqy);
                        ::codec::Encode::encode_to(to, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Call`, failed to read variant byte")
                    })?
                {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Call::<T>::create_claim {
                                claim: {
                                    let __codec_res_edqy = <BoundedVec<
                                        u8,
                                        T::MaxClaimLength,
                                    > as ::codec::Decode>::decode(__codec_input_edqy);
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Call::create_claim::claim`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                            })
                        })();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Call::<T>::revoke_claim {
                                claim: {
                                    let __codec_res_edqy = <BoundedVec<
                                        u8,
                                        T::MaxClaimLength,
                                    > as ::codec::Decode>::decode(__codec_input_edqy);
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Call::revoke_claim::claim`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                            })
                        })();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 2u8 as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Call::<T>::transfer_claim {
                                claim: {
                                    let __codec_res_edqy = <BoundedVec<
                                        u8,
                                        T::MaxClaimLength,
                                    > as ::codec::Decode>::decode(__codec_input_edqy);
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Call::transfer_claim::claim`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                                to: {
                                    let __codec_res_edqy = <T::AccountId as ::codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Call::transfer_claim::to`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                            })
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Call`, variant doesn't exist"),
                            )
                        })();
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            ::core::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            BoundedVec<u8, T::MaxClaimLength>: ::scale_info::TypeInfo + 'static,
            BoundedVec<u8, T::MaxClaimLength>: ::scale_info::TypeInfo + 'static,
            BoundedVec<u8, T::MaxClaimLength>: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new_with_replace(
                            "Call",
                            "pallet_poe::pallet",
                            &[],
                        ),
                    )
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "Contains a variant per dispatchable extrinsic that this pallet has.",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "create_claim",
                                |v| {
                                    v
                                        .index(0u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| {
                                                    f
                                                        .ty::<BoundedVec<u8, T::MaxClaimLength>>()
                                                        .name("claim")
                                                        .type_name("BoundedVec<u8, T::MaxClaimLength>")
                                                }),
                                        )
                                },
                            )
                            .variant(
                                "revoke_claim",
                                |v| {
                                    v
                                        .index(1u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| {
                                                    f
                                                        .ty::<BoundedVec<u8, T::MaxClaimLength>>()
                                                        .name("claim")
                                                        .type_name("BoundedVec<u8, T::MaxClaimLength>")
                                                }),
                                        )
                                },
                            )
                            .variant(
                                "transfer_claim",
                                |v| {
                                    v
                                        .index(2u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| {
                                                    f
                                                        .ty::<BoundedVec<u8, T::MaxClaimLength>>()
                                                        .name("claim")
                                                        .type_name("BoundedVec<u8, T::MaxClaimLength>")
                                                })
                                                .field(|f| {
                                                    f.ty::<T::AccountId>().name("to").type_name("T::AccountId")
                                                }),
                                        )
                                },
                            ),
                    )
            }
        }
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `create_claim`.
        pub fn new_call_variant_create_claim(
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> Self {
            Self::create_claim { claim }
        }
        ///Create a call with the variant `revoke_claim`.
        pub fn new_call_variant_revoke_claim(
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> Self {
            Self::revoke_claim { claim }
        }
        ///Create a call with the variant `transfer_claim`.
        pub fn new_call_variant_transfer_claim(
            claim: BoundedVec<u8, T::MaxClaimLength>,
            to: T::AccountId,
        ) -> Self {
            Self::transfer_claim { claim, to }
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create_claim { ref claim } => {
                    let __pallet_base_weight = { 0 };
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<
                        (&BoundedVec<u8, T::MaxClaimLength>,),
                    >>::weigh_data(&__pallet_base_weight, (claim,));
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<
                        (&BoundedVec<u8, T::MaxClaimLength>,),
                    >>::classify_dispatch(&__pallet_base_weight, (claim,));
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<
                        (&BoundedVec<u8, T::MaxClaimLength>,),
                    >>::pays_fee(&__pallet_base_weight, (claim,));
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::revoke_claim { ref claim } => {
                    let __pallet_base_weight = { 0 };
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<
                        (&BoundedVec<u8, T::MaxClaimLength>,),
                    >>::weigh_data(&__pallet_base_weight, (claim,));
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<
                        (&BoundedVec<u8, T::MaxClaimLength>,),
                    >>::classify_dispatch(&__pallet_base_weight, (claim,));
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<
                        (&BoundedVec<u8, T::MaxClaimLength>,),
                    >>::pays_fee(&__pallet_base_weight, (claim,));
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::transfer_claim { ref claim, ref to } => {
                    let __pallet_base_weight = { 0 };
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<
                        (&BoundedVec<u8, T::MaxClaimLength>, &T::AccountId),
                    >>::weigh_data(&__pallet_base_weight, (claim, to));
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<
                        (&BoundedVec<u8, T::MaxClaimLength>, &T::AccountId),
                    >>::classify_dispatch(&__pallet_base_weight, (claim, to));
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<
                        (&BoundedVec<u8, T::MaxClaimLength>, &T::AccountId),
                    >>::pays_fee(&__pallet_base_weight, (claim, to));
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__Ignore cannot be used")
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::CheckIfFeeless for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        #[allow(unused_variables)]
        fn is_feeless(&self, origin: &Self::Origin) -> bool {
            match *self {
                Self::create_claim { ref claim } => false,
                Self::revoke_claim { ref claim } => false,
                Self::transfer_claim { ref claim, ref to } => false,
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__Ignore cannot be used")
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> frame_support::traits::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create_claim { .. } => "create_claim",
                Self::revoke_claim { .. } => "revoke_claim",
                Self::transfer_claim { .. } => "transfer_claim",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__PhantomItem cannot be used.")
                        ),
                    );
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["create_claim", "revoke_claim", "transfer_claim"]
        }
    }
    impl<T: Config> frame_support::traits::GetCallIndex for Call<T> {
        fn get_call_index(&self) -> u8 {
            match *self {
                Self::create_claim { .. } => 0u8,
                Self::revoke_claim { .. } => 1u8,
                Self::transfer_claim { .. } => 2u8,
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__PhantomItem cannot be used.")
                        ),
                    );
                }
            }
        }
        fn get_call_indices() -> &'static [u8] {
            &[0u8, 1u8, 2u8]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type RuntimeOrigin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::RuntimeOrigin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            frame_support::dispatch_context::run_in_context(|| {
                match self {
                    Self::create_claim { claim } => {
                        let __within_span__ = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "create_claim",
                                        "pallet_poe::pallet",
                                        ::tracing::Level::TRACE,
                                        ::core::option::Option::Some("pallets/poe/src/lib.rs"),
                                        ::core::option::Option::Some(5u32),
                                        ::core::option::Option::Some("pallet_poe::pallet"),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        let __tracing_guard__ = __within_span__.enter();
                        <Pallet<T>>::create_claim(origin, claim)
                            .map(Into::into)
                            .map_err(Into::into)
                    }
                    Self::revoke_claim { claim } => {
                        let __within_span__ = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "revoke_claim",
                                        "pallet_poe::pallet",
                                        ::tracing::Level::TRACE,
                                        ::core::option::Option::Some("pallets/poe/src/lib.rs"),
                                        ::core::option::Option::Some(5u32),
                                        ::core::option::Option::Some("pallet_poe::pallet"),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        let __tracing_guard__ = __within_span__.enter();
                        <Pallet<T>>::revoke_claim(origin, claim)
                            .map(Into::into)
                            .map_err(Into::into)
                    }
                    Self::transfer_claim { claim, to } => {
                        let __within_span__ = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "transfer_claim",
                                        "pallet_poe::pallet",
                                        ::tracing::Level::TRACE,
                                        ::core::option::Option::Some("pallets/poe/src/lib.rs"),
                                        ::core::option::Option::Some(5u32),
                                        ::core::option::Option::Some("pallet_poe::pallet"),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        let __tracing_guard__ = __within_span__.enter();
                        <Pallet<T>>::transfer_claim(origin, claim, to)
                            .map(Into::into)
                            .map_err(Into::into)
                    }
                    Self::__Ignore(_, _) => {
                        let _ = origin;
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("__PhantomItem cannot be used.")
                                ),
                            );
                        };
                    }
                }
            })
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type RuntimeCall = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::__private::metadata_ir::PalletCallMetadataIR {
            frame_support::__private::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> frame_support::__private::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::__private::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::__private::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("`__Ignore` can never be constructed")
                        ),
                    );
                }
                Self::ProofAlreadyExist => "ProofAlreadyExist",
                Self::ClaimNotExist => "ClaimNotExist",
                Self::NotClaimOwner => "NotClaimOwner",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use frame_support::__private::codec::Encode;
            let index = <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Pallet<T>,
            >()
                .expect("Every active module has an index in the runtime; qed") as u8;
            let mut encoded = err.encode();
            encoded.resize(frame_support::MAX_MODULE_ERROR_ENCODED_SIZE, 0);
            frame_support::sp_runtime::DispatchError::Module(frame_support::sp_runtime::ModuleError {
                index,
                error: TryInto::try_into(encoded)
                    .expect(
                        "encoded error is resized to be equal to the maximum encoded error size; qed",
                    ),
                message: Some(err.as_str()),
            })
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
            let event = <<T as Config>::RuntimeEvent as Into<
                <T as frame_system::Config>::RuntimeEvent,
            >>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::__private::metadata_ir::PalletStorageMetadataIR {
            frame_support::__private::metadata_ir::PalletStorageMetadataIR {
                prefix: <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                    Pallet<T>,
                >()
                    .expect(
                        "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                    ),
                entries: {
                    #[allow(unused_mut)]
                    let mut entries = ::alloc::vec::Vec::new();
                    {
                        <Proofs<
                            T,
                        > as frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(
                            ::alloc::vec::Vec::new(),
                            &mut entries,
                        );
                    }
                    entries
                },
            }
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageProofs<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
    for _GeneratedPrefixForStorageProofs<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
                .expect(
                    "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
        fn pallet_prefix_hash() -> [u8; 16] {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name_hash::<
                Pallet<T>,
            >()
                .expect(
                    "No name_hash found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
        const STORAGE_PREFIX: &'static str = "Proofs";
        fn storage_prefix_hash() -> [u8; 16] {
            [
                145u8,
                222u8,
                78u8,
                1u8,
                129u8,
                17u8,
                171u8,
                113u8,
                121u8,
                8u8,
                63u8,
                139u8,
                161u8,
                210u8,
                19u8,
                182u8,
            ]
        }
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<
        T: Config,
    > frame_support::traits::Hooks<frame_system::pallet_prelude::BlockNumberFor<T>>
    for Pallet<T> {}
    impl<
        T: Config,
    > frame_support::traits::OnFinalize<frame_system::pallet_prelude::BlockNumberFor<T>>
    for Pallet<T> {
        fn on_finalize(n: frame_system::pallet_prelude::BlockNumberFor<T>) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_poe::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("pallets/poe/src/lib.rs"),
                            ::core::option::Option::Some(5u32),
                            ::core::option::Option::Some("pallet_poe::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame_support::traits::Hooks<
                frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_finalize(n)
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OnIdle<frame_system::pallet_prelude::BlockNumberFor<T>>
    for Pallet<T> {
        fn on_idle(
            n: frame_system::pallet_prelude::BlockNumberFor<T>,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            <Self as frame_support::traits::Hooks<
                frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_idle(n, remaining_weight)
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OnPoll<frame_system::pallet_prelude::BlockNumberFor<T>>
    for Pallet<T> {
        fn on_poll(
            n: frame_system::pallet_prelude::BlockNumberFor<T>,
            weight: &mut frame_support::weights::WeightMeter,
        ) {
            <Self as frame_support::traits::Hooks<
                frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_poll(n, weight);
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OnInitialize<
        frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_initialize(
            n: frame_system::pallet_prelude::BlockNumberFor<T>,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_poe::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("pallets/poe/src/lib.rs"),
                            ::core::option::Option::Some(5u32),
                            ::core::option::Option::Some("pallet_poe::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame_support::traits::Hooks<
                frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_initialize(n)
        }
    }
    impl<T: Config> frame_support::traits::BeforeAllRuntimeMigrations for Pallet<T> {
        fn before_all_runtime_migrations() -> frame_support::weights::Weight {
            use frame_support::traits::{Get, PalletInfoAccess};
            use frame_support::__private::hashing::twox_128;
            use frame_support::storage::unhashed::contains_prefixed_key;
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "before_all",
                            "pallet_poe::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("pallets/poe/src/lib.rs"),
                            ::core::option::Option::Some(5u32),
                            ::core::option::Option::Some("pallet_poe::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_hashed_prefix = <Self as PalletInfoAccess>::name_hash();
            let exists = contains_prefixed_key(&pallet_hashed_prefix);
            if !exists {
                let default_version = frame_support::traits::StorageVersion::new(0);
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "🐥 New pallet {0:?} detected in the runtime. The pallet has no defined storage version, so the on-chain version is being initialized to {1:?}.",
                                << T as frame_system::Config >::PalletInfo as
                                frame_support::traits::PalletInfo >::name::< Self > ()
                                .unwrap_or("<unknown pallet name>"), default_version
                            ),
                            lvl,
                            &(
                                frame_support::LOG_TARGET,
                                "pallet_poe::pallet",
                                "pallets/poe/src/lib.rs",
                            ),
                            5u32,
                            (),
                        );
                    }
                };
                default_version.put::<Self>();
                <T as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
            } else {
                <T as frame_system::Config>::DbWeight::get().reads(1)
            }
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "pallet_poe::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("pallets/poe/src/lib.rs"),
                            ::core::option::Option::Some(5u32),
                            ::core::option::Option::Some("pallet_poe::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "✅ no migration for {0}", << T as frame_system::Config
                            >::PalletInfo as frame_support::traits::PalletInfo >::name::<
                            Self > ().unwrap_or("<unknown pallet name>")
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "pallet_poe::pallet",
                            "pallets/poe/src/lib.rs",
                        ),
                        5u32,
                        (),
                    );
                }
            };
            <Self as frame_support::traits::Hooks<
                frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_runtime_upgrade()
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OffchainWorker<
        frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn offchain_worker(n: frame_system::pallet_prelude::BlockNumberFor<T>) {
            <Self as frame_support::traits::Hooks<
                frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::offchain_worker(n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            frame_support::__private::sp_io::TestExternalities::default()
                .execute_with(|| {
                    <Self as frame_support::traits::Hooks<
                        frame_system::pallet_prelude::BlockNumberFor<T>,
                    >>::integrity_test()
                });
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
    pub use __tt_extra_parts_7 as tt_extra_parts;
    pub use __tt_default_parts_v2_7 as tt_default_parts_v2;
}
