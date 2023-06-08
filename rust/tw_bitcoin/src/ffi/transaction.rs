use super::p2tr_key_path::{TWTxInputP2TRKeyPath, TWTxOutputP2TRKeyPath};
use super::p2wpkh::{TWTxInputP2WPKH, TWTxOutputP2WPKH};
use crate::{output::TxOutputP2TRKeyPath, Recipient, TransactionBuilder};
use bitcoin::PublicKey;
use bitcoin::ScriptBuf;
use secp256k1::KeyPair;
use tw_memory::ffi::c_byte_array::{CByteArray, CByteArrayResult};
use tw_memory::ffi::c_byte_array_ref::CByteArrayRef;
use tw_memory::ffi::c_result::ErrorCode;
use tw_memory::ffi::RawPtrTrait;
use tw_misc::{try_or_else, try_or_false};

pub struct TWTransactionBuilder(TransactionBuilder);

impl RawPtrTrait for TWTransactionBuilder {}

#[no_mangle]
//pub unsafe extern "C" fn tw_build_pay_to_taproot_key_spend_script(
pub unsafe extern "C" fn tw_transaction_builder_create() -> *mut TWTransactionBuilder {
    let builder = TransactionBuilder::new();

    TWTransactionBuilder(builder).into_ptr()
}

pub unsafe extern "C" fn tw_transaction_add_p2wpkh_input(
    builder: *mut TWTransactionBuilder,
    input: *mut TWTxInputP2WPKH,
) -> *mut TWTransactionBuilder {
    let builder = try_or_else!(TWTransactionBuilder::from_ptr(builder), std::ptr::null_mut);
    let input = try_or_else!(TWTxInputP2WPKH::from_ptr(input), std::ptr::null_mut);

    let builder = builder.0.add_input(input.0.into());

    TWTransactionBuilder(builder).into_ptr()
}

pub unsafe extern "C" fn tw_transaction_add_p2tr_key_path_input(
    builder: *mut TWTransactionBuilder,
    input: *mut TWTxInputP2TRKeyPath,
) -> *mut TWTransactionBuilder {
    let builder = try_or_else!(TWTransactionBuilder::from_ptr(builder), std::ptr::null_mut);
    let input = try_or_else!(TWTxInputP2TRKeyPath::from_ptr(input), std::ptr::null_mut);

    let builder = builder.0.add_input(input.0.into());

    TWTransactionBuilder(builder).into_ptr()
}

pub unsafe extern "C" fn tw_transaction_add_p2wpkh_output(
    builder: *mut TWTransactionBuilder,
    output: *mut TWTxOutputP2WPKH,
) -> *mut TWTransactionBuilder {
    let builder = try_or_else!(TWTransactionBuilder::from_ptr(builder), std::ptr::null_mut);
    let output = try_or_else!(TWTxOutputP2WPKH::from_ptr(output), std::ptr::null_mut);

    let builder = builder.0.add_output(output.0.into());

    TWTransactionBuilder(builder).into_ptr()
}

pub unsafe extern "C" fn tw_transaction_add_p2tr_key_path_output(
    builder: *mut TWTransactionBuilder,
    output: *mut TWTxOutputP2TRKeyPath,
) -> *mut TWTransactionBuilder {
    let builder = try_or_else!(TWTransactionBuilder::from_ptr(builder), std::ptr::null_mut);
    let output = try_or_else!(TWTxOutputP2TRKeyPath::from_ptr(output), std::ptr::null_mut);

    let builder = builder.0.add_output(output.0.into());

    TWTransactionBuilder(builder).into_ptr()
}

#[repr(C)]
pub enum CTransactionBuilderError {
    Ok = 0,
    InvalidBuilder = 1,
    InvalidSecretKey = 2,
    FailedSigning = 3,
    FailedSerialization = 4,
}

impl From<CTransactionBuilderError> for ErrorCode {
    fn from(e: CTransactionBuilderError) -> Self {
        e as ErrorCode
    }
}

#[no_mangle]
pub unsafe extern "C" fn tw_transaction_sign(
    builder: *mut TWTransactionBuilder,
    secret_key: *const u8,
    secret_key_len: usize,
) -> CByteArrayResult {
    let Some(builder) = TWTransactionBuilder::from_ptr(builder) else {
		return CByteArrayResult::error(CTransactionBuilderError::InvalidBuilder);
	};

    // Convert secret key to keypair.
    let Some(slice) = CByteArrayRef::new(secret_key, secret_key_len).as_slice() else {
		return CByteArrayResult::error(CTransactionBuilderError::InvalidSecretKey);
	};

    let Ok(keypair) = KeyPair::from_seckey_slice(&secp256k1::Secp256k1::new(), slice) else {
		return CByteArrayResult::error(CTransactionBuilderError::InvalidSecretKey);
	};

    // Sign transaction.
    let Ok(builder) = builder.0.sign_inputs(keypair) else {
		return CByteArrayResult::error(CTransactionBuilderError::FailedSigning);
	};

    builder
        .serialize()
        .map(CByteArray::from)
        .map_err(|_| CTransactionBuilderError::FailedSerialization)
        .into()
}