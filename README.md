# Conflux app for Ledger Wallet

## Overview

This app adds support for the Conflux native token CFX to the Ledger Nano S/X hardware wallet.

Current Features:

- Public key queries
- Parse, display, and sign simple CFX transfer transactions
- Blind sign arbitrary contract-call transactions (enabled via the Settings)
- Sign arbitrary data (`personal_sign`)

## Prerequisites

Be sure to have your environment correctly set up (see [Getting Started](https://developers.ledger.com/docs/nano-app/introduction/)) and [ledgerblue](https://pypi.org/project/ledgerblue/) installed.

## Compilation

```
make DEBUG=1  # compile optionally with PRINTF
make load     # load the app on the Nano using ledgerblue
```

## Documentation

High level documentation such as [APDU](doc/APDU.md), and [commands](doc/COMMANDS.md) are included in the developer documentation which can be generated with [doxygen](https://www.doxygen.nl)

```
doxygen .doxygen/Doxyfile
```

the process outputs HTML and LaTeX documentations in `doc/html` and `doc/latex` folders.

## Tests & Continuous Integration

The flow processed in [GitHub Actions](https://github.com/features/actions) is the following:

- Code formatting with [clang-format](http://clang.llvm.org/docs/ClangFormat.html)
- Compilation of the application for Ledger Nano S in [ledger-app-builder](https://github.com/LedgerHQ/ledger-app-builder)
- End-to-end tests with [Speculos](https://github.com/LedgerHQ/speculos) emulator (see [tests/](tests/))

It outputs 2 artifacts:

- `conflux-app-debug` within output files of the compilation process in debug mode
- `speculos-log` within APDU command/response when executing end-to-end tests
