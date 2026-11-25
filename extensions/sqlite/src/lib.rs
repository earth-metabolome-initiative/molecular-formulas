//! SQLite registration for the `must_be_molecular_formula` scalar function.

use core::{
    ffi::{c_char, c_int},
    ptr, slice, str,
};
use std::panic::catch_unwind;

use libsqlite3_sys as sqlite;
use molecular_formulas::ChemicalFormula;

/// Registers the validator on every subsequently opened connection to this
/// SQLite library. Repeated calls are harmless. Connections already open are
/// unaffected.
///
/// # Errors
///
/// Returns the SQLite error if registration fails.
#[cfg(not(feature = "loadable-extension"))]
pub fn register_on_every_new_connection() -> Result<(), sqlite::Error> {
    // SAFETY: SQLite retains a static callback that neither opens nor closes a
    // connection.
    let status = unsafe { sqlite::sqlite3_auto_extension(Some(initialize)) };
    if status == sqlite::SQLITE_OK { Ok(()) } else { Err(sqlite::Error::new(status)) }
}

/// # Safety
///
/// SQLite owns the live connection and calls this initializer before using it.
/// For a loadable extension the host provides its process-lifetime API table.
/// The caller owns the error slot, which this initializer leaves untouched.
#[cfg_attr(feature = "loadable-extension", unsafe(export_name = "sqlite3_extension_init"))]
unsafe extern "C" fn initialize(
    connection: *mut sqlite::sqlite3,
    _error_message: *mut *mut c_char,
    _host_api: *const sqlite::sqlite3_api_routines,
) -> c_int {
    #[cfg(feature = "loadable-extension")]
    {
        // SAFETY: The SQLite loader provides the API table before any SQLite
        // call in this module.
        if unsafe { sqlite::rusqlite_extension_init2(_host_api.cast_mut()) }.is_err() {
            return sqlite::SQLITE_ERROR;
        }
    }
    // SAFETY: SQLite owns the connection and both the function name and
    // callback are static.
    unsafe {
        sqlite::sqlite3_create_function_v2(
            connection,
            c"must_be_molecular_formula".as_ptr(),
            1,
            sqlite::SQLITE_UTF8 | sqlite::SQLITE_DETERMINISTIC | sqlite::SQLITE_INNOCUOUS,
            ptr::null_mut(),
            Some(validate),
            None,
            None,
            None,
        )
    }
}

/// # Safety
///
/// SQLite supplies the live result context and exactly one protected value for
/// the duration of this callback, as specified by `initialize`.
unsafe extern "C" fn validate(
    context: *mut sqlite::sqlite3_context,
    _argument_count: c_int,
    arguments: *mut *mut sqlite::sqlite3_value,
) {
    // SAFETY: The registered arity guarantees one argument. SQLite copies the
    // result before returning. No conversion of the protected value follows
    // the UTF-8 byte-length query.
    unsafe {
        let argument = *arguments;
        match sqlite::sqlite3_value_type(argument) {
            sqlite::SQLITE_NULL => return sqlite::sqlite3_result_null(context),
            sqlite::SQLITE_TEXT => (),
            _ => return sqlite::sqlite3_result_error(context, c"expected TEXT".as_ptr(), -1),
        }
        let text = sqlite::sqlite3_value_text(argument);
        if text.is_null() {
            return sqlite::sqlite3_result_error_nomem(context);
        }
        let Ok(length) = usize::try_from(sqlite::sqlite3_value_bytes(argument)) else {
            return sqlite::sqlite3_result_error_toobig(context);
        };
        let Ok(formula) = str::from_utf8(slice::from_raw_parts(text, length)) else {
            return sqlite::sqlite3_result_error(context, c"expected UTF-8 text".as_ptr(), -1);
        };
        match catch_unwind(|| {
            formula.parse::<ChemicalFormula>().map(|_| ()).map_err(|error| error.to_string())
        }) {
            Ok(Ok(())) => sqlite::sqlite3_result_int(context, 1),
            Ok(Err(message)) => {
                match c_int::try_from(message.len()) {
                    Ok(length) => {
                        sqlite::sqlite3_result_error(context, message.as_ptr().cast(), length)
                    }
                    Err(_) => sqlite::sqlite3_result_error_toobig(context),
                }
            }
            Err(_) => {
                sqlite::sqlite3_result_error(
                    context,
                    c"molecular formula parser panicked".as_ptr(),
                    -1,
                )
            }
        }
    }
}
