# Note

//*****************************************************************************
//
// This set of tests takes a heavy handed approach to errors, whereby the
// process is exited. This is done because panic! and assert_eq! failures
// are trapped within `dioxus::runtime::RuntimeGuard`.
// Unfortunately panic! is still made silent.
//
// Errors will be shown with:
// cargo test -- --nocapture
//
//*****************************************************************************
