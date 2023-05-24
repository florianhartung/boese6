use std::path::Path;

use libloading::{Library, Symbol};
use self_cell::self_cell;

use crate::error::Error;
use crate::game::player_decision::DecideFn;

type SymbolDecideFn<'a> = Symbol<'a, DecideFn>;

self_cell!(
    pub struct LoadedDecideFn {
        owner: Library,
        #[covariant]
        dependent: SymbolDecideFn,
    }
);

impl LoadedDecideFn {
    pub fn to_decide_fn(&self) -> &DecideFn {
        &*self.borrow_dependent()
    }
}

pub fn load_strategy(library_file: &Path) -> Result<LoadedDecideFn, Error> {
    if !library_file.exists() {
        return Err(Error::StrategyLibraryFileMissing);
    }

    let library =
        unsafe { Library::new(library_file) }.map_err(|err| Error::StrategyLibraryLoading(err))?;

    let loaded_decide_fn = LoadedDecideFn::try_new(library, |library| {
        unsafe { library.get(b"decide") }.map_err(|err| Error::StrategyLibraryLoading(err))
    })?;

    Ok(loaded_decide_fn)
}
