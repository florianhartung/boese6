pub enum Error {
    StrategyLibraryFileMissing,
    StrategyLibraryLoading(libloading::Error),
}
