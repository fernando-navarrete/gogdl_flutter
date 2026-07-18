# gogdl_flutter API Reference

`gogdl_flutter` is a Flutter plugin that exposes [`gogdl-lib2`](../gogdl-lib2)
— a Rust library for GOG's Galaxy backend (auth, catalog, downloads, repairs,
verification, cloud saves, Proton-GE releases) — to Dart via
[`flutter_rust_bridge`](https://pub.dev/packages/flutter_rust_bridge) 2.12.0.
This document is the complete specification of the Dart-facing surface: every
class, method, field, and flow a consuming Flutter app needs.

- All long-running operations (download/repair/verify/save-transfer/proton
  download) are exposed as `Stream<T>` methods on `Gog`, not
  `Future` + separate progress channel — see [§1](#1-core-concepts) for how
  that mapping works and what it means for error handling.
- The only entry point is the `Gog` class. You never touch the underlying
  Rust `GogDl`/`DownloadControl` types directly; this package wraps them in
  Dart-visible, `RustOpaqueInterface`-backed classes.
- Fields on the plugin's data classes are `BigInt` for anything that maps to
  a Rust `u64`/`usize` (byte counts, chunk counts, file counts) and `int` for
  `i32` (game ids, pages). String/`List<String>` map directly.

## Table of contents

1. [Core concepts](#1-core-concepts)
2. [Installation & setup](#2-installation--setup)
3. [Error handling (`GogError`)](#3-error-handling-gogerror)
4. [Authentication](#4-authentication)
5. [Game catalog](#5-game-catalog)
6. [Download flow](#6-download-flow)
   - [6.1 Discovering what to download](#61-discovering-what-to-download)
   - [6.2 Downloading (`downloadGame`)](#62-downloading-downloadgame)
   - [6.3 Pause / resume / cancel (`DownloadControl`)](#63-pause--resume--cancel-downloadcontrol)
   - [6.4 Repairing an install (`repairDownload`)](#64-repairing-an-install-repairdownload)
   - [6.5 Verifying an install (`verifyDownload`)](#65-verifying-an-install-verifydownload)
   - [6.6 Tuning network behavior (`setDownloadConfig`)](#66-tuning-network-behavior-setdownloadconfig)
7. [Cloud saves](#7-cloud-saves)
8. [Proton-GE releases](#8-proton-ge-releases)
9. [Known gaps & caveats](#9-known-gaps--caveats)
10. [Full `Gog` method index](#10-full-gog-method-index)

---

## 1. Core concepts

**`Gog` is the only entry point.** Construct one with `Gog()` (a synchronous
factory — no network calls happen here, auth starts unset) and call methods
on it. Internally it owns an `Arc<GogDl>` on the Rust side, so it's cheap to
hold as a long-lived singleton in your app (e.g. behind a provider/service
locator); you don't need to recreate it per screen.

**Long-running operations return a `Stream<T>`, not a `Future`.** On the Rust
side, `gogdl-lib2` pushes progress onto an `mpsc` channel while a separate
`async fn` resolves with the final `Result`. `flutter_rust_bridge`'s
`StreamSink<T>` parameter pattern collapses both of those into a single Dart
`Stream<T>`:

```dart
final stream = gog.downloadGame(/* ... */);
await for (final snapshot in stream) {
  // one snapshot per progress tick — see DownloadStream below
}
// the stream closing (without throwing) means the Rust future resolved Ok(())
```

- Every `stream.add(...)` call on the Rust side is one Dart stream **data**
  event.
- If the Rust future resolves `Err(GogDlError)` (mapped to `GogError`), the
  Dart stream emits one **error** event (catchable via `stream.listen(...,
  onError: ...)` or `try`/`catch` around an `await for`) and then closes.
- If the Rust future resolves `Ok(())`, the stream just closes — there is no
  separate "final" event marker beyond whatever terminal snapshot the method
  happens to `add()` right before returning (documented per-method below).

**One error type wraps everything.** Every fallible `Gog` method (`Future`-
returning or `Stream`-returning) surfaces failures as `GogError` — see
[§3](#3-error-handling-gogerror).

**Catalog/manifest data is cached in-memory for the lifetime of the
underlying `GogDl` instance**, i.e. for as long as your `Gog()` object is
alive (see `gogdl-lib2`'s own `api.md` §1 for the exact caching rules per
method — `getOwnedGames`, `getGameBuilds`, `getDownloadableProducts`, etc. all
inherit this). Dispose and recreate `Gog()` if you need to force a refetch.

**This package narrows the raw crate's download flow.** `gogdl-lib2` has an
explicit `get_downloadable_files` step between "list products" and
"download/repair/verify" that resolves per-chunk manifests you inspect before
committing. This bridge does not expose that step or its `DepotFile`/`Chunk`
types at all — `downloadGame`/`repairDownload`/`verifyDownload` take
`selectedProducts` (a list of product-id strings) directly and resolve
manifests internally. You cannot preview per-depot size/language/chunk data
through this package; if you need that, use `getDownloadableProducts` (§6.1)
for a coarse per-product listing, or extend the bridge.

---

## 2. Installation & setup

```yaml
dependencies:
  gogdl_flutter:
    path: ../gogdl_flutter  # or a git/pub dependency, per your setup
```

```dart
import 'package:gogdl_flutter/gogdl_flutter.dart';

Future<void> main() async {
  await RustLib.init(); // loads the native library; call once, before using Gog
  final gog = Gog();
  runApp(MyApp(gog: gog));
}
```

- `RustLib.init({ExternalLibrary? externalLibrary})` must be awaited once
  before any `Gog` method is called. Passing no arguments loads the plugin's
  bundled native library for the current platform (the normal case for a
  Flutter app that depends on this package as a plugin); `externalLibrary` is
  an escape hatch for custom native-library loading setups.
- `RustLib.dispose()` tears down the native bridge; call it only if your app
  needs to fully unload the library (not needed for normal app lifecycle).
- The package supports Android, iOS, Linux, macOS, and Windows (declared as
  an `ffiPlugin` for each in `pubspec.yaml`).

---

## 3. Error handling (`GogError`)

```dart
sealed class GogError implements FrbException {
  // one constructor per variant, all shaped `{required String message}`:
  const factory GogError.authError({required String message}) = GogError_AuthError;
  const factory GogError.gameError({required String message}) = GogError_GameError;
  const factory GogError.depotError({required String message}) = GogError_DepotError;
  const factory GogError.secureLinksError({required String message}) = GogError_SecureLinksError;
  const factory GogError.downloadError({required String message}) = GogError_DownloadError;
  const factory GogError.savesError({required String message}) = GogError_SavesError;
  const factory GogError.protonError({required String message}) = GogError_ProtonError;
}
```

`GogError` is a `freezed` sealed union — every variant just carries a
human-readable `message` string (the `Display` output of the corresponding
Rust `GogDlError` variant, e.g. `"Download error: Http error: <body>, status:
404"`). It's thrown from `Future`-returning methods and surfaces as a stream
error from `Stream`-returning ones.

```dart
try {
  final title = await gog.getGameTitle(gameId: 12345);
} on GogError catch (e) {
  final message = e.map(
    authError: (e) => 'Auth: ${e.message}',
    gameError: (e) => 'Game: ${e.message}',
    depotError: (e) => 'Depot: ${e.message}',
    secureLinksError: (e) => 'Secure links: ${e.message}',
    downloadError: (e) => 'Download: ${e.message}',
    savesError: (e) => 'Saves: ${e.message}',
    protonError: (e) => 'Proton: ${e.message}',
  );
}
```

`.when(...)` works the same way as `.map(...)` (standard `freezed` sealed-
class API); both let you branch on *which subsystem* failed. You cannot get
finer-grained detail than the `message` string — this mirrors the underlying
crate's own type-reachability limits (its inner error enums aren't
re-exported; see `gogdl-lib2`'s `api.md` §9).

A message containing `"status: 401"` typically means the session needs
`refreshAuthWithCallback`; `"status: 404"` typically means a bad id/build
name.

---

## 4. Authentication

```dart
class Gog {
  String getLoginUrl();
  Future<String> loginWithCode({required String code});
  Future<void> restoreAuthFromString({required String token});
  Future<void> refreshAuthWithCallback({
    required FutureOr<void> Function(String) callback,
  });
}
```

- **`getLoginUrl()`** — synchronous. The URL to open in a browser/webview.
  After login, GOG redirects to
  `embed.gog.com/on_login_success?...code=...`; extract the `code` query
  parameter.
- **`loginWithCode(code)`** — exchanges the code for tokens, stores them
  internally on the `Gog` instance, and returns them serialized as an opaque
  JSON string. **Persist this string yourself** (secure storage/keychain) —
  the package does not persist anything for you.
- **`restoreAuthFromString(token)`** — restores a previously-persisted string
  into this `Gog` instance (e.g. on app startup) instead of making the user
  log in again.
- **`refreshAuthWithCallback(callback)`** — uses the stored refresh token to
  obtain a new access token, then **invokes your callback with the new
  persisted-auth string** (rather than returning it) so you can persist it
  immediately. Fails with `GogError` if no token was ever restored/set or the
  refresh token itself is no longer valid (in which case the user needs to
  log in again from `getLoginUrl()`).

### Flow

```dart
final gog = Gog();
// First run:
final code = await showLoginWebview(gog.getLoginUrl());
final persisted = await gog.loginWithCode(code: code);
await secureStorage.write(key: 'gog_auth', value: persisted);

// Every subsequent run:
final saved = await secureStorage.read(key: 'gog_auth');
if (saved != null) {
  await gog.restoreAuthFromString(token: saved);
}

// Proactively before expiry, or after a call fails with an auth-shaped GogError:
await gog.refreshAuthWithCallback(
  callback: (newToken) async {
    await secureStorage.write(key: 'gog_auth', value: newToken);
  },
);
```

> **Caveat — automatic refresh is not persisted for you.** Like the
> underlying crate, every authenticated call already retries once on an HTTP
> 401 by silently refreshing in memory — your call still succeeds. But that
> fresh token is not handed to you unless you call
> `refreshAuthWithCallback` yourself. If your process restarts before you've
> done that, `restoreAuthFromString` restores the old, already-rotated-out
> refresh token, which may be rejected. Call `refreshAuthWithCallback`
> proactively on a schedule for long-lived sessions.

> **Caveat — numeric ids only.** `gameId`/product-id strings passed
> internally are parsed as integers and will panic (not return a `GogError`)
> if non-numeric. GOG ids are always numeric in practice.

---

## 5. Game catalog

```dart
class Gog {
  Future<Int32List> getOwnedGames();
  Future<String> getGameTitle({required int gameId});
  Future<List<GameBuild>> getGameBuilds({required int gameId});
  Future<String> getGameBoxartLink({required int gameId});
  Future<String> getBackgroundImageLink({required int gameId});
  Future<String> getGameSummary({required int gameId});
  Future<List<String>> getGameScreenshots({required int gameId});
}
```

- **`getOwnedGames()`** — the authenticated account's owned GOG product ids.
  ⚠️ Internally this also resolves each id's title (4-way concurrent) just to
  filter the list — **any id whose title lookup fails for any reason
  (network error, the id being DLC and not a game, etc.) is silently dropped
  from the result**, with no way to tell "zero owned games" apart from "every
  lookup happened to fail." If you need the title anyway, prefer calling
  `getGameTitle` per id yourself and handling errors explicitly rather than
  relying on this filtering.
- **`getGameTitle(gameId)`** — a single game's title.
- **`getGameBuilds(gameId)`** — published builds, **Windows-only** (no OS
  parameter; matches the underlying crate). Returns:

  ```dart
  abstract class GameBuild {
    String get buildId;
    String get versionName;       // pass this as `buildName` elsewhere
    String get releaseDate;        // RFC 2822 formatted
    PlatformInt64 get releaseDateTimestamp; // unix seconds
  }
  ```
- **`getGameBoxartLink`/`getBackgroundImageLink`** — box art / Galaxy
  background image URLs. Unauthenticated calls. Note both hit the same
  underlying GOG endpoint but are cached and fetched independently — calling
  both for the same game fetches twice.
- **`getGameSummary(gameId)`** — the game's English store description.
  Unauthenticated call.
- **`getGameScreenshots(gameId)`** — resolved screenshot URLs (already
  flattened from the raw templated data; a screenshot without a usable
  variant is silently skipped). Unauthenticated call.

---

## 6. Download flow

### 6.1 Discovering what to download

```dart
Future<List<DownloadableProduct>> getDownloadableProducts({
  required int gameId,
  required String buildName, // == some GameBuild.versionName
});
```

```dart
abstract class DownloadableProduct {
  String get id;           // product id — pass into selectedProducts below
  String get name;         // e.g. "My Game", "Soundtrack DLC"
  String get productType;  // e.g. "game", "dlc"
}
```

Use this to build a "select which DLC to install" UI: fetch the list, let the
user pick, then pass the chosen `.id`s as `selectedProducts` to
`downloadGame`/`repairDownload`/`verifyDownload`. Like `getOwnedGames`, any
product whose detail lookup fails (4-way concurrent) is silently dropped from
the result.

> Unlike the raw `gogdl-lib2` crate, this list carries no depot/size/language
> data — see the note in [§1](#1-core-concepts).

### 6.2 Downloading (`downloadGame`)

```dart
Stream<DownloadStream> downloadGame({
  required String path,               // install root; created if missing
  required String buildName,
  required List<String> selectedProducts, // product ids from §6.1
  required int gameId,
  required DownloadControl control,   // see §6.3 — pass DownloadControl()
});
```

Downloads every file for the selected products into `path` from scratch (no
existing-install diffing — use `repairDownload` for that). Emits a
`DownloadStream` snapshot after every meaningful step.

```dart
abstract class DownloadStream {
  DownloadStatus get status; // FetchingFiles | Allocating | Downloading | Downloaded
  BigInt get totalFiles;
  BigInt get allocatedFiles;
  BigInt get totalChunks;
  BigInt get downloadedChunks;
  BigInt get totalBytes;
  BigInt get downloadedBytes;
  List<String> get errorFiles;  // paths that hit an allocation or download error
  JobStatus get jobStatus;      // pause/resume/cancel state — see §6.3
}

abstract class DownloadStatus {
  String name(); // "fetchingFiles" | "allocating" | "downloading" | "downloaded"
}
```

**Snapshot sequence:**
1. `status: FetchingFiles` — immediately, before any network I/O
   (`totalFiles`/etc. still zero).
2. `status: Allocating` — once manifests are resolved; `totalFiles` populated,
   then one snapshot per file as it's pre-sized on disk (`allocatedFiles`
   increments; files that already existed at the correct size count too).
3. `status: Downloading` — one snapshot per progress tick once byte transfer
   starts; `totalBytes`/`totalChunks` set once at the start of this stage,
   `downloadedBytes` accumulates, `downloadedChunks` increments per finished
   chunk.
4. `status: Downloaded` — exactly once, right before the stream closes.

**Track overall progress** as `downloadedBytes / totalBytes`.
**`errorFiles` is a live snapshot, not sticky-then-cleared** — a chunk-level
error (secure-link/download/write/path-resolve) adds its path and it stays
for the rest of the stream; there's no per-chunk retry visibility beyond that.
A non-empty `errorFiles` on the final `Downloaded` snapshot means some files
did not fully download even though the stream closed without throwing (see
next point).

> **The terminal `status: Downloaded` does not mean "fully succeeded."** The
> Rust future only returns `Err` for setup-level failures (e.g. can't create
> the install directory); a chunk that exhausts retries does not fail the
> whole job. Check `errorFiles.isEmpty` on the final snapshot to know whether
> everything actually downloaded, and check `jobStatus` (§6.3) to know
> whether the job finished because it completed or because it was cancelled.

### 6.3 Pause / resume / cancel (`DownloadControl`)

```dart
abstract class DownloadControl implements RustOpaqueInterface {
  factory DownloadControl();  // fresh handle in the Running state

  void pause();
  void resume();
  void cancel();
  JobStatus status();
  bool isPaused();
  bool isCancelled();
}

abstract class JobStatus implements RustOpaqueInterface {
  String name(); // "running" | "pausing" | "paused" | "cancelling" | "cancelled" | "completed"
}
```

`DownloadControl` drives and observes one `downloadGame`/`repairDownload`
job's lifecycle from outside the code awaiting/listening to its stream.
Construct one **per job** (don't reuse a handle across two calls) and pass it
to `control:`; keep a reference (e.g. in your download-manager state) to call
`pause()`/`resume()`/`cancel()` from a UI button handler.

```dart
final control = DownloadControl();
final stream = gog.downloadGame(
  path: installDir,
  buildName: build.versionName,
  selectedProducts: selectedIds,
  gameId: gameId,
  control: control,
);

stream.listen((snapshot) {
  updateUi(bytes: snapshot.downloadedBytes, total: snapshot.totalBytes,
           jobStatus: snapshot.jobStatus.name());
});

// pause button:
control.pause();
// resume button:
control.resume();
// cancel button:
control.cancel();
```

**How pause/resume/cancel surface to Dart:** unlike the raw crate (where
`DownloadControl` state is entirely separate from the progress `mpsc`
channel — pausing emits nothing on it), **this bridge folds `jobStatus` into
every `DownloadStream`/`RepairStream` snapshot** so you only need to listen
to one stream. The Rust side keeps this fresh in near-real time even while no
chunk events are arriving (i.e. while parked) by racing the progress channel
against `DownloadControl::subscribe()` internally — you don't need to poll
`control.status()` yourself, though it's available (`status()`/`isPaused()`/
`isCancelled()`) for synchronous checks (e.g. to set initial button state).

**Two-phase transitions**, same as the underlying crate: `pause()` while
chunks are mid-transfer publishes `jobStatus: pausing` immediately; once the
last already-in-flight chunk finishes writing (chunks are never discarded
mid-transfer — they always finish and flush), it settles to `paused`. Same
shape for `cancel()` → `cancelling` → `cancelled`. If nothing is in flight
when you call either, the settled state appears immediately.

**`cancel()` still lets the stream close normally** (no thrown `GogError`) —
cancellation is a successful wind-down, not a failure. The way to tell "the
user cancelled" apart from "the job actually finished" is the terminal
snapshot's `jobStatus.name()`: it reads `"cancelled"` if `cancel()` was
called, or `"completed"` if the job ran to completion on its own (`status` on
that same snapshot will still read `"downloaded"`/`"repaired"` either way,
since that field only tracks which *stage* the pipeline was in — see the
caveat in §6.2/§6.4).

> **Behavior change from earlier versions:** a finished (non-cancelled) job's
> terminal `jobStatus` used to stay `"running"` — the only distinguishable
> terminal state was `"cancelled"`. As of the underlying crate's
> `JobStatus::Completed` addition, a job that runs to completion now settles
> to `"completed"` instead. If your app's `jobStatus.name()` switch only
> handled `"running"`/`"pausing"`/`"paused"`/`"cancelling"`/`"cancelled"`, add
> a `"completed"` case.

**`verifyDownload` has no `DownloadControl` parameter** — it's a read-only
checksum pass and is not pausable/cancellable (§6.5).

### 6.4 Repairing an install (`repairDownload`)

```dart
Stream<RepairStream> repairDownload({
  required String path,
  required String buildName,
  required List<String> selectedProducts,
  required int gameId,
  required DownloadControl control,  // same handle semantics as §6.3
});
```

Repairs an existing install: verifies files against the manifests for
`selectedProducts`, then re-allocates/re-downloads only what's missing or
doesn't match.

```dart
abstract class RepairStream {
  RepairStatus get status;
  // file-verify stage progress
  BigInt get totalFiles;
  BigInt get processedFiles;
  // download stage progress (populated once the download stage starts)
  BigInt get totalChunks;
  BigInt get downloadedChunks;
  BigInt get totalBytes;
  BigInt get downloadedBytes;
  List<String> get errorFiles;  // paths still broken as of this snapshot
  JobStatus get jobStatus;      // pause/resume/cancel state — see §6.3
}

abstract class RepairStatus {
  String name(); // "fetchingFiles" | "verifyingFiles" | "allocating"
                  // | "verifyingChunks" | "downloading" | "repaired"
}
```

**Stage order** (all reported on the same stream): `fetchingFiles` →
`verifyingFiles` (checks every file's on-disk size against the manifest —
size only, not checksum) → `allocating` (re-)allocates files that failed
verification → `verifyingChunks` (for files that passed the size check,
verifies each chunk's MD5 individually) → `downloading` (downloads every
chunk queued by the previous two stages) → `repaired` (final).

**`errorFiles` is reconciled, not raw.** A file getting flagged during
`verifyingFiles`/`allocating`/`verifyingChunks` is the normal, expected way
repair discovers work to do — it is **not** itself an error. `errorFiles`
only lists a path once the drain loop's bookkeeping determines it's *still*
broken: either the download stage never confirmed a chunk for it, or it hit a
hard download-stage error (which is sticky regardless of arrival order, since
one path's chunks can succeed and fail in any order). In practice this means
`errorFiles` will typically be empty mid-repair (a flagged file that
downloads successfully a moment later drops off the list) and only reflects
genuinely-unresolved paths by the time you reach the `repaired` snapshot.

Same terminal-snapshot caveat as `downloadGame` applies: `status: repaired`
means the pipeline reached the end, not that it succeeded — check
`errorFiles.isEmpty` and `jobStatus`.

### 6.5 Verifying an install (`verifyDownload`)

```dart
Stream<VerificationStream> verifyDownload({
  required String path,
  required String buildName,
  required List<String> selectedProducts,
  required int gameId,
});
```

Read-only integrity check: verifies every chunk's MD5 against an existing
install. Does not modify anything on disk. **No `DownloadControl` parameter**
— not pausable/cancellable, matching the underlying crate (it's a fast,
bounded checksum pass, not a network-bound download).

```dart
abstract class VerificationStream {
  VerificationStatus get status;
  BigInt get totalChunks;
  BigInt get verifiedChunks;
  List<String> get errorChunks; // paths with a checksum mismatch / not found / unresolved
}

abstract class VerificationStatus implements RustOpaqueInterface {}
// no variants, no name(), and no `==` — see the caveat below.
```

**Snapshot sequence:** `status: fetchingFiles` → `status: verifying`
(repeated once per chunk result, with `totalChunks` populated on the second
snapshot onward) → `status: verified` (final).

> **Caveat — `VerificationStatus` cannot be inspected from Dart.** Every
> other status enum in this package (`DownloadStatus`, `RepairStatus`,
> `JobStatus`, `SaveDownloadStatus`, `SaveUploadStatus`,
> `ProtonDownloadStatus`) has a `name()` accessor and `==` support so Dart
> code can branch on which stage a snapshot is in. `VerificationStatus` was
> not given either (it's an empty `RustOpaqueInterface` with no exposed
> methods), so today there is no way to distinguish `fetchingFiles` /
> `verifying` / `verified` from Dart — you can only read `totalChunks`/
> `verifiedChunks`/`errorChunks` and infer progress from those (e.g.
> "verified" is safely inferable as "the stream is about to close" /
> `verifiedChunks == totalChunks` once `totalChunks > 0`). This is a gap in
> the current implementation, not intentional API design — treat it as
> something to fix if you need stage-level UI for verification specifically.

### 6.6 Tuning network behavior (`setDownloadConfig`)

```dart
Future<void> setDownloadConfig({
  required BigInt minConcurrency,
  required BigInt maxConcurrency,
  required BigInt idleTimeout, // seconds
});
```

Overrides network tuning for every subsequent `downloadGame`/
`repairDownload`/`verifyDownload` call on this `Gog` instance.
`minConcurrency`/`maxConcurrency` bound the adaptive chunk-download
concurrency controller (raw crate defaults: 4 / 64); `idleTimeout` is the
deadline, in seconds, for each body-stream read (raw crate default: 15s).

> **This only exposes 3 of the underlying crate's 8 `DownloadConfig` knobs**
> (`sample_interval`, `step`, `response_timeout`, `max_retries`,
> `base_backoff` are left at their crate defaults and have no Dart-facing
> equivalent today). See `gogdl-lib2`'s `api.md` §5.6 for what those unexposed
> knobs do if you need to extend this method.

---

## 7. Cloud saves

```dart
class Gog {
  Future<SaveAuthIds> getSaveAuthIds({required int gameId});
  Future<CloudSaveConfig> getSaveRemoteConfig({required String clientId});
  Future<List<CloudSaveFile>> getSaveFileList({
    required String clientId,
    required String clientSecret,
  });
  Stream<SaveDownloadStream> downloadSave({
    required CloudSaveFile saveFile,
    required String clientId,
    required String clientSecret,
    required String path,
  });
  Stream<SaveUploadStream> uploadSave({
    required String clientId,
    required String clientSecret,
    required String path,
    required String urlPath,
  });
  Future<void> deleteSave({
    required CloudSaveFile saveFile,
    required String clientId,
    required String clientSecret,
  });
}
```

```dart
abstract class SaveAuthIds {
  String get clientId;
  String get clientSecret;
}

abstract class CloudSaveConfig {
  bool isSupported();
  (String, String) localPath(); // (knownFolderName, relativePath)
}

abstract class CloudSaveFile {
  String path(); // display path, "saves/" prefix already stripped
}

abstract class SaveDownloadStream {
  SaveDownloadStatus get status; // "downloading" | "downloaded"
  BigInt get transferred;
  BigInt get total; // 0 if the server didn't report a size
}

abstract class SaveUploadStream {
  SaveUploadStatus get status; // "uploading" | "uploaded"
  BigInt get transferred;
  BigInt get total;
}
```

### Flow

```dart
final auth = await gog.getSaveAuthIds(gameId: gameId);
final remoteConfig = await gog.getSaveRemoteConfig(clientId: auth.clientId);
if (!remoteConfig.isSupported()) {
  return; // this game doesn't use cloud saves on this OS
}
final (knownFolder, relativePath) = remoteConfig.localPath();

final saves = await gog.getSaveFileList(
  clientId: auth.clientId,
  clientSecret: auth.clientSecret,
);

final stream = gog.downloadSave(
  saveFile: saves.first,
  clientId: auth.clientId,
  clientSecret: auth.clientSecret,
  path: '/local/save/slot1.sav',
);
await for (final snapshot in stream) {
  updateProgress(snapshot.transferred, snapshot.total);
}
```

- **`downloadSave`** decompresses, verifies the MD5 if the server supplied
  one, and restores the cloud-stored modified time on the downloaded file.
- **`uploadSave`** gzip-compresses in transit; `urlPath` is the destination
  under `saves/{url_path}` in cloud storage.
- **`deleteSave`** — ⚠️ **unverified against a live GOG account.** This
  endpoint was extrapolated from GOG's REST conventions in the underlying
  crate and has not been confirmed to actually work in production. Treat a
  successful return cautiously and verify by re-calling `getSaveFileList`.
- `knownFolderName` from `localPath()` is one of: `"Saved Games"`,
  `"Documents"`, `"Desktop"`, `"AppData/Roaming"`, `"AppData/Local"`,
  `"ProgramData"`, `"Users/Public"`, or the sentinel `"INSTALLATION_PATH"`
  (relative to the game's own install directory, not a Windows known folder).

---

## 8. Proton-GE releases

```dart
class Gog {
  Future<List<ProtonRelease>> getProtonReleases({required int page});
  Stream<ProtonDownloadStream> downloadProtonRelease({
    required ProtonRelease release,
    required String path,
  });
}
```

```dart
abstract class ProtonRelease {
  String tagName();       // e.g. "GE-Proton1-1"
  BigInt downloadSize();  // 0 if unknown
}

abstract class ProtonDownloadStream {
  ProtonDownloadStatus get status; // "downloading" | "extracting" | "downloaded"
  BigInt get transferred;
  BigInt get total;
}
```

### Flow

```dart
var page = 1;
final allReleases = <ProtonRelease>[];
while (true) {
  final releases = await gog.getProtonReleases(page: page);
  if (releases.isEmpty) break; // no pagination metadata — an empty page means the end
  allReleases.addAll(releases);
  page++;
}

final stream = gog.downloadProtonRelease(
  release: allReleases.first,
  path: '/opt/proton-ge/GE-Proton1-1',
);
await for (final snapshot in stream) {
  print('${snapshot.status.name()}: ${snapshot.transferred}/${snapshot.total}');
}
```

**Snapshot sequence:** `status: downloading` (repeated as bytes arrive) →
`status: extracting` (exactly once, once the byte-transfer channel closes —
checksum verification and tarball extraction are still running at this
point, with no further progress granularity) → `status: downloaded` (final).

> ⚠️ The tarball is downloaded **entirely into memory** before any checksum
> verification or extraction — expect a memory footprint briefly matching
> the compressed download size (hundreds of MB for a Proton-GE build). If
> the release metadata includes a SHA-256 digest, it's verified before
> extraction; if not, extraction proceeds unverified.
- `getProtonReleases` has no caching — call again with an empty result to
  know you've reached the end; there's no total-pages metadata.

---

## 9. Known gaps & caveats

Summarized from the sections above, for quick reference when integrating:

| Gap | Where | Impact |
|---|---|---|
| `VerificationStatus` has no `name()`/`==` | §6.5 | Can't distinguish verify stages from Dart; infer from chunk counts instead |
| `getOwnedGames`/`getDownloadableProducts` silently drop entries whose detail lookup fails | §5, §6.1 | A partial-failure batch looks identical to "fewer items exist"; use the raw per-id calls if you need to detect this |
| `setDownloadConfig` only exposes 3/8 tuning knobs | §6.6 | `sampleInterval`/`step`/`responseTimeout`/`maxRetries`/`baseBackoff` aren't reachable from Dart |
| Terminal `status` (`downloaded`/`repaired`) doesn't imply success | §6.2, §6.4 | Always check `errorFiles`/`jobStatus` on the last snapshot, not just that the stream closed without throwing |
| No depot/size/language preview before download | §1, §6.1 | This bridge collapses the raw crate's `get_downloadable_files` step; extend it if you need pre-download size estimates |
| `deleteSave` unverified against production | §7 | Inherited caveat from `gogdl-lib2`; re-verify with `getSaveFileList` after calling it |

---

## 10. Full `Gog` method index

| Method | Returns | Section |
|---|---|---|
| `Gog()` | `Gog` (factory, sync) | [§1](#1-core-concepts) |
| `getLoginUrl()` | `String` (sync) | [§4](#4-authentication) |
| `loginWithCode(code)` | `Future<String>` | [§4](#4-authentication) |
| `restoreAuthFromString(token)` | `Future<void>` | [§4](#4-authentication) |
| `refreshAuthWithCallback(callback)` | `Future<void>` | [§4](#4-authentication) |
| `getOwnedGames()` | `Future<Int32List>` | [§5](#5-game-catalog) |
| `getGameTitle(gameId)` | `Future<String>` | [§5](#5-game-catalog) |
| `getGameBuilds(gameId)` | `Future<List<GameBuild>>` | [§5](#5-game-catalog) |
| `getGameBoxartLink(gameId)` | `Future<String>` | [§5](#5-game-catalog) |
| `getBackgroundImageLink(gameId)` | `Future<String>` | [§5](#5-game-catalog) |
| `getGameSummary(gameId)` | `Future<String>` | [§5](#5-game-catalog) |
| `getGameScreenshots(gameId)` | `Future<List<String>>` | [§5](#5-game-catalog) |
| `getDownloadableProducts(gameId, buildName)` | `Future<List<DownloadableProduct>>` | [§6.1](#61-discovering-what-to-download) |
| `downloadGame(path, buildName, selectedProducts, gameId, control)` | `Stream<DownloadStream>` | [§6.2](#62-downloading-downloadgame) |
| `repairDownload(path, buildName, selectedProducts, gameId, control)` | `Stream<RepairStream>` | [§6.4](#64-repairing-an-install-repairdownload) |
| `verifyDownload(path, buildName, selectedProducts, gameId)` | `Stream<VerificationStream>` | [§6.5](#65-verifying-an-install-verifydownload) |
| `setDownloadConfig(minConcurrency, maxConcurrency, idleTimeout)` | `Future<void>` | [§6.6](#66-tuning-network-behavior-setdownloadconfig) |
| `getSaveAuthIds(gameId)` | `Future<SaveAuthIds>` | [§7](#7-cloud-saves) |
| `getSaveRemoteConfig(clientId)` | `Future<CloudSaveConfig>` | [§7](#7-cloud-saves) |
| `getSaveFileList(clientId, clientSecret)` | `Future<List<CloudSaveFile>>` | [§7](#7-cloud-saves) |
| `downloadSave(saveFile, clientId, clientSecret, path)` | `Stream<SaveDownloadStream>` | [§7](#7-cloud-saves) |
| `uploadSave(clientId, clientSecret, path, urlPath)` | `Stream<SaveUploadStream>` | [§7](#7-cloud-saves) |
| `deleteSave(saveFile, clientId, clientSecret)` | `Future<void>` | [§7](#7-cloud-saves) |
| `getProtonReleases(page)` | `Future<List<ProtonRelease>>` | [§8](#8-proton-ge-releases) |
| `downloadProtonRelease(release, path)` | `Stream<ProtonDownloadStream>` | [§8](#8-proton-ge-releases) |

### `DownloadControl` method index

| Method | Returns | Section |
|---|---|---|
| `DownloadControl()` | `DownloadControl` (factory, sync) | [§6.3](#63-pause--resume--cancel-downloadcontrol) |
| `pause()` | `void` (sync) | [§6.3](#63-pause--resume--cancel-downloadcontrol) |
| `resume()` | `void` (sync) | [§6.3](#63-pause--resume--cancel-downloadcontrol) |
| `cancel()` | `void` (sync) | [§6.3](#63-pause--resume--cancel-downloadcontrol) |
| `status()` | `JobStatus` (sync) | [§6.3](#63-pause--resume--cancel-downloadcontrol) |
| `isPaused()` | `bool` (sync) | [§6.3](#63-pause--resume--cancel-downloadcontrol) |
| `isCancelled()` | `bool` (sync) | [§6.3](#63-pause--resume--cancel-downloadcontrol) |

### Full list of exported types (`package:gogdl_flutter/gogdl_flutter.dart`)

`RustLib`, `Gog`, `GameBuild`, `DownloadableProduct`, `VerificationStream`,
`VerificationStatus`, `RepairStream`, `RepairStatus`, `DownloadStream`,
`DownloadStatus`, `DownloadControl`, `JobStatus`, `CloudSaveFile`,
`CloudSaveConfig`, `SaveAuthIds`, `SaveDownloadStream`, `SaveDownloadStatus`,
`SaveUploadStream`, `SaveUploadStatus`, `ProtonRelease`,
`ProtonDownloadStream`, `ProtonDownloadStatus`, `GogError`.
