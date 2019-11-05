# zeitmanagement
CLI tool um Zeiten für ein oder mehrere Projekte zu protokolieren.

bauen ohne statisches linking um unter anderem Binary Size zu reduzieren:

cargo rustc --release -- -C prefer-dynamic