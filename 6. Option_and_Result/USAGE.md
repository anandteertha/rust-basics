# connSim CLI (Option_and_Result)

Small Rust CLI to simulate connection lifecycle using in-memory state.

## Run

```powershell
cargo run
```

Running with no arguments starts REPL mode (interactive loop).

## REPL Commands

Inside REPL, type:

```text
create <id> <address>
activate <id> <address>
retry <id> <address>
fail <id> <address>
disconnect <id> <address>
reset <id> <address>
inspect <id> <address>
```

Exit REPL:

```text
exit
```

or

```text
quit
```

## Example Session

```text
connSim> create c1 10.0.0.1
connSim> activate c1 10.0.0.1
connSim> inspect c1 10.0.0.1
connSim> fail c1 10.0.0.1
connSim> inspect c1 10.0.0.1
connSim> reset c1 10.0.0.1
connSim> quit
```

## One-shot Mode

You can still run exactly one command and exit:

```powershell
cargo run -- create c1 10.0.0.1
```

Important: in one-shot mode, memory is not persisted across runs because each `cargo run` starts a new process.

## Notes

- `id` must be unique for `create`.
- For existing connections, command `address` must match the stored address.
- State is currently in-memory only (no file/database persistence).
