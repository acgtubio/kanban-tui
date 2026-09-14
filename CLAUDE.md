# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A terminal Kanban board built with [Ratatui](https://ratatui.rs) (crate name `editui`), scaffolded from Ratatui's "event driven" template. Tasks are persisted to a local SQLite file (`kanban.db`, created next to the binary at runtime; ignored by git).

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Run a single test: `cargo test <test_name>` (e.g. `cargo test should_be_focused_on_kanban_pending`)
- Release build (matches CI release profile): `cargo build --release`

Tests live inline as `#[cfg(test)] mod tests` blocks in the same file as the code they cover (see `src/app.rs`, `src/state/app_state.rs`). There is no separate `tests/` integration directory. Tests that need a DB use `SqliteDb::new_in_memory()` rather than touching the real `kanban.db`.

CI (`.github/workflows/rust.yml`) runs `cargo build --verbose` and `cargo test --verbose` on PRs to `main`. Releases (`.github/workflows/release.yml`) build on tag push (`v*`) for Linux and macOS (aarch64) targets and publish binaries via `softprops/action-gh-release`.

## Architecture

The app follows a unidirectional event-loop architecture: **crossterm input → typed event → handler mutates state → UI redraws from state**. There is no direct calling from UI components into handlers or vice versa; everything flows through the `Event`/`AppEvent` channel.

1. **Event source** (`src/event.rs`): `EventHandler` spawns a background thread (`EventThread`) that polls crossterm at a fixed tick rate (30 FPS) and forwards both `Tick` and raw `Crossterm` events over an `mpsc` channel. `AppEvent` is the enum of all app-level (non-raw) events — one variant per screen/modal (`KanbanScreenEvent`, `AddTaskEvent`, `MoveTaskEvent`, `MainScreenEvent`), plus `Quit`. `NavigationEvent`/`InputEvent` are shared sub-enums reused across those screen events.

2. **Key mapping** (`src/event_mux.rs`): translates raw crossterm `KeyEvent`s into `AppEvent`s. Which mapping function runs is decided by `state.active_pane` (the `Pane` enum in `src/state/app_state.rs`: `Kanban(TaskStatus)`, `Column`, `AddTask`, `MoveTaskModal`, `Preview`). This is the place to add/change keybindings — add a `KeyCode` arm to the relevant `handle_*_event` function, keyed by which pane is active.

3. **Dispatch** (`src/app.rs`): `App::handle_events` reads the next `Event` and routes `AppEvent` variants to their handler (`self.handle_kanban_screen_event`, etc.). Each handler is a static-method struct in `src/handler/` (`ColumnHandler`, `AddTaskModalHandler`, `MoveTaskHandler`, `MainScreenHandler`) that takes `&mut AppState` and the event, and mutates state directly — handlers contain the only business logic that touches `AppState`.

4. **State** (`src/state/app_state.rs`): `AppState` is the single source of truth — task list (`HashMap<TaskStatus, Vec<Task>>`), current `active_pane`, kanban/modal focus, and the add-task form state (`src/state/add_task_state.rs`, `src/state/task_field.rs`, `src/state/task_field_value.rs`). `AppState` owns the `SqliteDb` and is the only place DB calls are made — writes (`add_task`, `update_task`, `delete_task`) are fire-and-forget with `TODO: Handle errors` markers; don't assume error propagation exists yet.

5. **Rendering** (`src/components/`): components implement the `Component` trait (`draw` + `get_children_layout`) and render read-only from `&mut AppState` — they never mutate state directly, only read it (e.g. `Kanban` in `src/components/tasks.rs` renders the three status columns; `TaskCard` in `src/components/task/task_card.rs` renders one task line; modals live in `src/components/dialog/`). `App::render` in `src/app.rs` draws the base layout and conditionally overlays the move-task or add-task modal based on `AppState` flags.

6. **Persistence** (`src/db/`): `Db` trait + `SqliteDb` impl in `src/db/db.rs` wraps `rusqlite` with a single `tasks` table (`uuid`, `name`, `description`, `status`, `priority`, all `TEXT`). `TaskModel` (`src/db/task_model.rs`) is the DB-row shape; `Task` (`src/components/task/task.rs`) is the domain/UI shape. Conversion is explicit both ways (`Task::from_task_model`, `From<Task> for TaskModel`) — status/priority enums serialize to upper-snake-case strings (`"IN_PROGRESS"`, `"CRITICAL"`, etc.) via each enum's own `to_string`/`from_string`/`FromSql` impls.

### Adding a new keybinding or feature end-to-end

Typically touches four layers in order: add an event variant (`event.rs`) → map the key in the right pane's handler function (`event_mux.rs`) → implement the state mutation in the matching `src/handler/*_handler.rs` (calling into new/existing `AppState` methods) → update the component that reads the affected state (`src/components/`).
