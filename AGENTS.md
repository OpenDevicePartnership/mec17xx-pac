# AGENTS.md

Canonical guidance for AI coding assistants (and humans) working on
`mec17xx-pac`. This file is the source of truth for project conventions,
build/test commands, and what is safe (and unsafe) to change.

If you are an AI agent reading this for the first time: **read this whole
file before editing anything.** The bulk of this crate is auto-generated
from vendor SVDs; touching the wrong file by hand will be lost on the
next regeneration.

---

## What this crate is

`mec17xx-pac` is a [Peripheral Access Crate (PAC)] for the Microchip
**MEC17xx / CEC17xx** family of Arm Cortex-M4F microcontrollers. It
provides type-safe `unsafe` register access wrappers used by higher-level
HALs (e.g. `embedded-hal` drivers, embassy HALs).

- Crate name: `mec17xx-pac` (`Cargo.toml`)
- Version: `0.1.3` (`Cargo.toml`)
- License: MIT (`LICENSE`, `Cargo.toml`)
- Edition: `2021` (`Cargo.toml`)
- MSRV: **`rust-version = "1.85"`** (`Cargo.toml`, verified by the
  `msrv` CI job in `.github/workflows/check.yml`)
- Targets: `thumbv7em-none-eabihf` (Cortex-M4F with FPU); see
  `rust-toolchain.toml` and `.github/workflows/nostd.yml`.
- `no_std`: yes (`#![no_std]` at the top of `src/lib.rs`)
- Upstream code generator: [`svd2rust`] (per `README.md`), with a
  `chiptool`-based pipeline in the helper script `update` (see
  "Regenerating the PAC" below). The actual PAC tree (`src/chips/*`)
  is produced by `chiptool`.

[Peripheral Access Crate (PAC)]: https://docs.rust-embedded.org/book/start/registers.html
[`svd2rust`]: https://github.com/rust-embedded/svd2rust

### Supported parts (Cargo features)

One **chip-selection feature** must be enabled per build. From
`Cargo.toml`:

CEC family:
- `cec1712h_b2_sx`, `cec1712h_n2_sx`, `cec1712h_s2_sx`
- `cec1734_s0_2hw`, `cec1734_s0_2zw`
- `cec1736_s0_2hw`, `cec1736_s0_2zw`

MEC family:
- `mec1721n_b0_lj`, `mec1721n_b0_sz`
- `mec1723n_b0_lj`, `mec1723n_b0_sz`, `mec1723n_f0_sz`, `mec1723n_p0_9y`
- `mec1724n_b0_lj`, `mec1724n_b0_sz`
- `mec1725n_b0_lj`
- `mec1727n_b0_sz`

**Temporarily disabled (generated code does not compile)** — do not
re-enable without regenerating and fixing the SVDs first:
`cec1702`, `mec1701h`, `mec1701q`, `mec1703h`, `mec1703q`,
`mec1704q`, `mec1705q` (see commented entries in `Cargo.toml` and the
matching commented `#[cfg_attr]` lines in `src/lib.rs`).

The mapping from chip feature → generated module lives in `src/lib.rs`
via `#[cfg_attr(feature = "...", path = "./chips/<chip>/pac.rs")]`.
Several MEC17[23-5] variants are deliberately aliased to a single
`mec1723_4` module because they share an SVD (see also the special case
in `build.rs`).

### Other features

- `default = ["rt"]` (`Cargo.toml`). **The default `rt` feature alone
  does not select a chip** — you must add `-F <chip_feature>` for any
  `check`/`build`/`clippy`/`doc` command, otherwise `build.rs` panics
  with `"No chip Cargo feature enabled"`.
- `rt` → enables `cortex-m-rt/device` and pulls in the per-chip
  `device.x` linker script (`build.rs` sets `cargo:rustc-link-search`).
- `defmt` → optional `defmt = "1.0"` integration on register types
  (`Cargo.toml`).

### Runtime dependencies

From `Cargo.toml`:
- `cortex-m = "0.7.7"`
- `cortex-m-rt = ">=0.7.5,<0.8"` (optional, behind `rt`)
- `defmt = "1.0"` (optional, behind `defmt`)

No `embedded-hal` dependency in the PAC itself; it is pulled in
transitively only by some build-time paths. HALs consume this crate;
this crate does not depend on a HAL trait crate.

---

## Layout

```
.
├── AGENTS.md                 ← you are here
├── Cargo.toml                ← crate metadata, chip features
├── README.md
├── LICENSE                   ← MIT
├── CONTRIBUTING.md           ← upstream ODP contribution policy
├── CODE_OF_CONDUCT.md
├── CODEOWNERS
├── SECURITY.md
├── deny.toml                 ← cargo-deny configuration
├── rustfmt.toml              ← nightly-only options, see "Formatting"
├── rust-toolchain.toml       ← targets + components hint
├── build.rs                  ← selects single chip feature, wires device.x
├── update                    ← `cargo +nightly -Zscript` regen helper
├── .github/
│   ├── workflows/
│   │   ├── check.yml         ← fmt / clippy / doc / hack / deny / msrv
│   │   └── nostd.yml         ← cross-compile for thumbv7em-none-eabihf
│   └── copilot-instructions.md  ← commit-message & AI-attribution rules
├── svd/                      ← vendor SVDs + chiptool transforms (excluded from publish)
│   ├── *.svd
│   └── transforms.yaml
└── src/
    ├── lib.rs                ← #![no_std] + per-feature `#[cfg_attr]`-routed `mod inner`
    └── chips/
        ├── cec1712/{pac.rs, device.x}
        ├── cec1734/{pac.rs, device.x}
        ├── cec1736/{pac.rs, device.x}
        ├── mec1721/{pac.rs, device.x}
        ├── mec1723_4/{pac.rs, device.x}   ← shared by mec1723*, mec1724*, mec1725*
        ├── mec1727/{pac.rs, device.x}
        └── (cec1702/, mec1701/, mec1703/, mec1704/, mec1705/  ← currently disabled)
```

Notes:
- `pac.rs` files are **generated** and may be several MB. `mec1723_4/pac.rs`
  is ~2.8 MB.
- `device.x` is the `cortex-m-rt` linker fragment with the chip's
  interrupt vector table. Also generated.
- `Cargo.toml` excludes `svd/*` and `transforms/*` from the published
  package via `exclude = [...]`.

---

## Building & testing

All commands below are taken verbatim from the CI workflows
(`.github/workflows/check.yml`, `.github/workflows/nostd.yml`) and have
been run locally on this checkout (results in parentheses).

`mec1723n_b0_sz` is the "canonical" chip feature CI uses everywhere; use
it as a default when you just need to sanity-check something.

### Host check (default feature `rt` + chip)

```sh
cargo check -F mec1723n_b0_sz
```
(✓ passes locally — same command used by the `msrv` job.)

### Cross-compile `no_std` (the embedded target the crate is for)

```sh
rustup target add thumbv7em-none-eabihf
cargo check --target thumbv7em-none-eabihf --no-default-features -F mec1723n_b0_sz
```
(✓ passes locally — matches `.github/workflows/nostd.yml`.)

### Format check

```sh
cargo fmt --check
```
CI runs this on **nightly** because `rustfmt.toml` uses unstable
options (`group_imports`, `imports_granularity`). On stable you will see
warnings like
`can't set 'imports_granularity = Module', unstable features are only available in nightly channel`
but the check still exits `0`. Use `cargo +nightly fmt` when actually
reformatting anything.

### Clippy

```sh
cargo clippy -F mec1723n_b0_sz -- \
  -F clippy::suspicious -F clippy::correctness \
  -F clippy::perf -F clippy::style
```
CI runs this on `stable` and `beta` via `giraffate/clippy-action@v1`
(reporter `github-pr-check`). **The generated `src/chips/*/pac.rs` may
produce a large number of clippy warnings/errors on newer toolchains**
(this is a known property of `chiptool`/`svd2rust`-generated code).

> Agents: **do not "fix" clippy findings inside `src/chips/*/pac.rs` by
> hand-editing the generated files.** If a clippy lint is genuinely
> wrong for generated code, suppress it via an attribute in
> `src/lib.rs` (which already does this for `non_camel_case_types` and
> `non_snake_case`) or via the `chiptool`/`svd2rust` transforms in
> `svd/transforms.yaml`, then regenerate.

### Docs

```sh
RUSTDOCFLAGS=--cfg docsrs cargo doc --no-deps -F mec1723n_b0_sz
```
(✓ passes locally.) CI runs this on **nightly** for `#[doc(cfg(...))]`
support.

### Feature combinations (cargo-hack)

CI iterates each chip feature in turn:

```sh
cargo install cargo-hack
cargo check -F rt,<chip_feature> --no-default-features
```
where `<chip_feature>` is each entry in the matrix in
`.github/workflows/check.yml` (`hack` job).

### Supply chain

```sh
cargo install cargo-deny
cargo deny --manifest-path ./Cargo.toml check --features mec1723n_b0_sz
```
Configuration: `deny.toml`.

### MSRV

```sh
rustup install 1.85
cargo +1.85 check -F mec1723n_b0_sz
```

### There are no unit tests

The crate is purely generated register definitions; there is no
`tests/` directory and CI does not run `cargo test`. Do not invent
tests inside `src/chips/*/pac.rs`.

---

## Code conventions

- **`no_std` only.** Top of `src/lib.rs`:
  ```rust
  #![no_std]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  ```
  Never introduce a `std::` import. If you need an allocation,
  reconsider — the PAC has no business allocating.
- **Edition 2021**, **MSRV 1.85** (`Cargo.toml`). Do not use language
  features stabilized after 1.85 in hand-written code (`src/lib.rs`,
  `build.rs`).
- **Formatting** (`rustfmt.toml`):
  - `max_width = 120`
  - `group_imports = "StdExternalCrate"` *(nightly-only)*
  - `imports_granularity = "Module"` *(nightly-only)*
  Use `cargo +nightly fmt` when changing hand-written code.
- **Generated files keep their generated style.** Never reformat
  `src/chips/*/pac.rs` or `src/chips/*/device.x` by hand. Changes there
  must come from regenerating with `chiptool` (see below).
- **Line endings: LF.** The repo has no `.gitattributes` and no
  `.editorconfig`, but every tracked file is `i/lf` per
  `git ls-files --eol`. The repo-local git config sets
  `core.autocrlf = false`. Keep it that way; do not commit CRLF.
- `src/lib.rs` deliberately re-exports the chip-specific PAC as
  `pub use inner::*;`. Do not change the module name `inner` or the
  per-feature `#[cfg_attr(..., path = "...")]` indirection without a
  very good reason — downstream HALs depend on `mec17xx_pac::*`
  resolving directly to peripheral types.

---

## PAC specifics (read this before editing anything under `src/chips/`)

### Source of truth

1. **Vendor SVDs in `svd/*.svd`** are the upstream source of truth for
   peripheral layout. They are copied (and renamed) from Microchip's
   ATPACKs by the `update` script (`MEC17xx_DFP` and `CEC_DFP`).
2. **`svd/transforms.yaml`** is a `chiptool` transform applied to every
   SVD during generation (e.g. it is responsible for the hand-curated
   GPIO `CTRL1`/`CTRL2` enums — see commits "Add CTRL2 register enums"
   and "gpio: Add CTRL1 register enums"). Add new enum/transform tweaks
   here, not in `pac.rs`.
3. **`build.rs`** selects exactly one chip Cargo feature, lower-cases
   it, replaces `_` with `-`, and adds the per-chip directory to
   `rustc-link-search` so `cortex-m-rt` can find `device.x`. Several
   parts are intentionally collapsed onto the shared `mec1723_4`
   directory:
   ```rust
   if _chip.starts_with("mec1723") || _chip.starts_with("mec1724") {
       println!("cargo:rustc-link-search={}/src/chips/mec1723_4", ...);
   }
   ```
   `mec1725n_b0_lj` is also routed to `mec1723_4` via `src/lib.rs`.

### Regenerating the PAC

The top-level `update` script is a single-file `cargo +nightly -Zscript`
program (note the shebang `#!/usr/bin/env -S cargo +nightly -Zscript`).
It:

1. Installs `svd2rust`, [`chiptool`](https://github.com/embassy-rs/chiptool),
   and `sd` via `cargo install`.
2. Downloads two Microchip ATPACKs (`Microchip.MEC17xx_DFP.1.4.221.atpack`
   and `Microchip.CEC_DFP.2.0.261.atpack`) from
   `https://packs.download.microchip.com/`.
3. Unpacks them, copies a curated subset of SVDs into `svd/` with the
   canonical short names (e.g. `MEC1723N_B0_LJ.svd` → `mec1723_4.svd`).
4. For each `svd/*.svd`, runs
   `chiptool generate --svd <file> --transform svd/transforms.yaml`,
   then `rustfmt lib.rs`, strips the `#![no_std]` line (the inner
   module is not the crate root), and moves `lib.rs` → `src/chips/<chip>/pac.rs`
   and `device.x` → `src/chips/<chip>/device.x`.
5. On Windows, it also runs `dos2unix` on the generated `.rs` to keep
   LF line endings — **this must remain LF** to match the repo policy.

Run it from the repo root with nightly Rust available:

```sh
./update
```

> **Do not hand-edit `src/chips/*/pac.rs` or `src/chips/*/device.x`.**
> Any fix to those files must be done by:
>   1. updating `svd/transforms.yaml` (or, as a last resort, the SVD
>      itself), and
>   2. regenerating via `./update`.
>
> Hand edits will be silently overwritten the next time anybody runs
> the regenerator and will be flagged in review.

### Adding a new chip

1. Place the new SVD in `svd/<chip>.svd`. If it shares a layout with an
   existing family, alias it (`MEC1723…` style).
2. Add the part-number Cargo feature to `[features]` in `Cargo.toml`
   under the appropriate `## <family>` comment.
3. Add a `#[cfg_attr(feature = "<feature>", path = "./chips/<chip>/pac.rs")]`
   line in `src/lib.rs`.
4. Add the feature to the `hack` matrix in
   `.github/workflows/check.yml`.
5. Regenerate (`./update`).
6. Verify locally:
   ```sh
   cargo check -F rt,<feature> --no-default-features
   cargo check --target thumbv7em-none-eabihf --no-default-features -F <feature>
   ```

### Removing/re-enabling a chip

Several MEC17[01-5] / CEC1702 entries are commented out in both
`Cargo.toml` and `src/lib.rs` because the generated code did not
compile at the time of their last attempt. Before re-enabling one, run
the full host + cross check; if it still fails, fix the underlying SVD
or transform — do not patch the generated `pac.rs`.

---

## Commit & PR conventions

Cross-checked against `CONTRIBUTING.md`, `.github/copilot-instructions.md`,
and `git log --pretty=%s | head -30`.

### Subject line

- Capitalized, **50 chars or less**, imperative mood ("Add", "Fix",
  not "Added"/"Fixes").
- Recent merges follow this faithfully:
  - `Update LICENSE copyright and add AI attribution instructions`
  - `don't enable any chips by default`  *(lowercased — sloppier but
    accepted; prefer capitalized)*
  - `gpio: Add CTRL1 register enums`
  - `Generate PAC`
- A `subsystem:` prefix is sometimes used (e.g. `gpio:`). Use it when
  the change is scoped to a single peripheral.

### Body

- Blank line between subject and body.
- Wrap body text at **72 chars**.
- Explain **what** and **why**, not **how**.

### AI attribution (mandatory for AI-assisted commits)

Per `.github/copilot-instructions.md`, every AI-assisted commit **must**
carry an `Assisted-by` trailer:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

- `AGENT_NAME` — e.g. `GitHub Copilot`.
- `MODEL_VERSION` — the *actual* model you are; verify it before
  composing the trailer, do not copy a value from a previous session.
- Optional analysis tools (`coccinelle`, `sparse`, `clang-tidy`, …) may
  follow. Routine dev tools (`git`, `cargo`, editors) are **not**
  listed.
- **AI agents must NOT add `Signed-off-by:`** — only humans can certify
  the DCO.

### PR etiquette

From `CONTRIBUTING.md`:
- Open as a **draft PR first**; wait for `.github/workflows/*` (lint /
  sanity) to go green before requesting review.
- **Squash-merge is disabled.** Maintain a clean commit history:
  - Each commit must build successfully **without warnings**.
  - Squash any typo/formatting fix-up commits into their parent before
    review.
- Report regressions with the offending commit identified via
  `git bisect`.

---

## What not to do

- ❌ Do not hand-edit `src/chips/*/pac.rs` or `src/chips/*/device.x`.
  Fix the SVD or `svd/transforms.yaml` and run `./update`.
- ❌ Do not commit CRLF line endings. Leave `core.autocrlf = false`.
- ❌ Do not add new dependencies without a strong reason. This is a PAC;
  it should be lean.
- ❌ Do not change the default `rt` feature or the `inner` module
  re-export pattern in `src/lib.rs` without coordinating with downstream
  HAL consumers.
- ❌ Do not enable multiple chip features in one build (`build.rs`
  panics with `"Multiple chip Cargo features enabled"`).
- ❌ Do not enable zero chip features in a `cargo build`/`check` that
  hits `build.rs` (`"No chip Cargo feature enabled"`). For library-only
  invocations you must always pass `-F <chip_feature>`.
- ❌ Do not add `Signed-off-by:` from an AI session.
- ❌ Do not run `cargo update` or bump dependency versions as part of an
  unrelated change. Crate publishability and HAL ABI matter.
- ❌ Do not introduce `std`, `alloc`, or panics into hand-written code
  paths (`src/lib.rs`).
- ❌ Do not add a `tests/` directory with synthetic tests against
  generated register addresses; CI does not run `cargo test` and they
  add noise.

---

## Context

- **Project:** [Open Device Partnership](https://opendevicepartnership.com/),
  hosted under `OpenDevicePartnership/` on GitHub.
- **Purpose of this PAC:** to be the base layer for Rust embedded
  software running on Microchip MEC17xx / CEC17xx EC-class MCUs (e.g.
  embedded controllers, security co-processors).
- **Consumers:** higher-level HALs / drivers in the ODP ecosystem;
  they expect `mec17xx_pac::*` to resolve to the chip selected via a
  Cargo feature.
- **License:** MIT only; new files should not introduce other licenses
  without coordinating with maintainers (see `CONTRIBUTING.md`,
  "Other Contribution Information").

---

## Incorporated from `.github/copilot-instructions.md`

The original `.github/copilot-instructions.md` covered two topics; both
are summarized above and reproduced here verbatim so this file is
self-contained:

### Commit Messages
- Subject line: capitalized, 50 characters or less, imperative mood
  (e.g., "Fix bug" not "Fixed bug")
- Separate subject from body with a blank line
- Wrap body text at 72 characters
- Use the body to explain *what* and *why*, not *how*

### AI Attribution
Every commit that includes AI-generated or AI-assisted work **must**
contain an `Assisted-by` trailer in the commit message:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

Where:
- `AGENT_NAME` is the name of the AI tool or framework (e.g.,
  `GitHub Copilot`)
- `MODEL_VERSION` is the specific model version used (e.g.,
  `claude-opus-4.6`)
- `[TOOL1] [TOOL2]` are optional specialized analysis tools used
  (e.g., `coccinelle`, `sparse`, `smatch`, `clang-tidy`)

Basic development tools (git, cargo, editors) should not be listed.

AI agents **must** verify their own identity (agent name and model
version) before composing the `Assisted-by` trailer — do not assume or
hard-code a model name from a previous session.

AI agents **MUST NOT** add `Signed-off-by` tags. Only humans can
certify the Developer Certificate of Origin.

## Model selection & cost discipline

Premium models (Opus, GPT-5 family, "high"/"xhigh" reasoning variants)
cost an order of magnitude more than standard models (Sonnet, Haiku,
mini). Most steps in a typical task do not need premium reasoning,
and over-using premium models wastes credits without improving
outcomes. The rules below apply to *all* model selection: your own
session, sub-agents launched via the `task` tool, and parallel work
launched via `/fleet`.

### Default posture

- **Default to the cheapest model that can do the job.** Reach for a
  premium model only when one of the escalation triggers below is hit.
- **Plan with premium, execute with cheap.** Spend at most one or two
  premium turns on design / planning, then downshift to a cheaper
  model for mechanical execution of the plan.
- **Never bump the model "just in case."** If you cannot articulate
  *why* a cheaper model would fail, use the cheaper model.

### Escalation triggers (use a premium model)

Reach for a premium model when *any* of these are true:

- Cross-module refactor, architectural design, or API design from
  scratch.
- Subtle correctness reasoning: concurrency, lifetimes, `unsafe`,
  FFI ABI, cryptography, safety-critical control paths.
- Debugging a failure that survived one prior cheap-model attempt.
- Reviewing code on a safety-, security-, or money-critical path.
- The diff cannot be predicted in advance — i.e. there is genuine
  creative or design work to do, not just typing.

### De-escalation triggers (use a cheap model)

Use the cheapest available model when *any* of these are true:

- Searching, reading, summarising files or docs.
- Single-file mechanical edits: rename, format, lint fix, dependency
  bump, boilerplate, scaffolding from a known template.
- Generating tests for code that already works.
- Running builds, tests, linters, or other commands where the model
  only needs to report success/failure.
- Routine commits, PR descriptions, changelog entries.
- The diff is essentially predictable before generation.

### Sub-agent routing (the `task` tool)

When delegating with the `task` tool, set `model:` explicitly. Do not
let sub-agents inherit a premium default for cheap work.

| Sub-agent type    | Default model             | Override to                                     |
|-------------------|---------------------------|-------------------------------------------------|
| `explore`         | cheap                     | keep cheap (`claude-haiku-4.5` or `gpt-5-mini`) |
| `task` (run cmd)  | cheap                     | keep cheap                                      |
| `research`        | cheap for breadth         | premium only for the final synthesis            |
| `general-purpose` | match task                | cheap for mechanical work; premium for design   |
| `rubber-duck`     | premium                   | keep premium — this is where reasoning pays off |
| `code-review`     | premium on critical paths | cheap on cosmetic / mechanical diffs            |

### `/fleet` (parallel sub-agents) rules

- Fleet mode multiplies cost by the fleet width. Apply the rules
  above *per worker*, not in aggregate.
- Split a fleet job along complexity lines: route the cheap,
  parallelisable workers (file edits, test runs, doc updates) to a
  cheap model; reserve premium models for the small number of
  workers that need real reasoning.
- If every worker in a fleet would need a premium model, the work is
  probably not a good fit for fleet mode — reconsider the
  decomposition before paying N× premium.

### Session hygiene

- Keep sessions short and focused. Long premium sessions are the
  single largest source of waste because every turn re-processes the
  full history.
- Use `/compact` when the conversation grows long, and `/new` for
  unrelated work.
- Prefer `/ask` for one-off side questions so they don't extend the
  main session.

### When in doubt

Ask: *"If a cheaper model produced the wrong answer here, would I
catch it in seconds (compiler, tests, my own review) or in
weeks (production incident)?"* If the former, use the cheap model
and let the feedback loop do its job.
