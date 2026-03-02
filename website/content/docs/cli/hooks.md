---
title: "Hook Commands"
description: "Commands for running and managing hooks."
---

Commands for running validation hooks and managing hook execution.

## `han hook run`

Run a validation hook from an installed plugin.

### Usage

```bash
# New format: Run a plugin's hook
han hook run <plugin-name> <hook-name>

# With options
han hook run <plugin-name> <hook-name> --verbose

# Disable caching
han hook run <plugin-name> <hook-name> --no-cache

# Legacy format: Run custom command across directories
han hook run --dirs-with <file> -- <command>
```

### Options

| Option | Description |
|--------|-------------|
| `--no-cache` | Disable caching (caching is ON by default in v2.0.0+) |
| `--no-checkpoints` | Disable checkpoint filtering (checkpoints are ON by default) |
| `--verbose` | Show full command output in real-time |
| `--directory <path>` | Limit execution to specific directory |
| `--checkpoint-type <type>` | Filter against checkpoint type (`session` or `agent`) |
| `--checkpoint-id <id>` | Filter against specific checkpoint ID |

**Legacy options:**

| Option | Description |
|--------|-------------|
| `--dirs-with <file>` | Only run in directories containing the specified file |
| `--test-dir <command>` | Only include directories where this command exits 0 |

**Breaking Change (v2.0.0):** Caching is enabled by default. Use `--no-cache` to disable it.

### Caching Behavior

Caching is enabled by default (since v2.0.0):

1. Han creates a checkpoint with file modification times
2. On subsequent runs, compares current file times to checkpoint
3. Skips execution if no files have changed
4. Clears checkpoint on failure (ensures retry on next run)

Checkpoints are session-scoped by default, meaning they're cleared when the Claude Code session ends. Use `--no-checkpoints` to disable checkpoint filtering entirely.

### Examples

```bash
# Run Bun tests (caching enabled by default)
han hook run bun test

# Run without caching
han hook run bun test --no-cache

# Run TypeScript type checking verbosely
han hook run typescript typecheck --verbose

# Run Biome lint in specific directory
han hook run biome lint --directory packages/core

# Legacy: Run npm test in directories with package.json
han hook run --dirs-with package.json -- npm test
```

### Plugin Hook Configuration

Hooks are defined in plugin `han-plugin.yml` files:

```yaml
hooks:
  test:
    command: bun test --only-failures
    dirs_with: [bun.lock, bun.lockb]
    description: Run Bun tests
    if_changed: ["**/*.ts", "**/*.test.ts"]
```

When you run `han hook run bun test`, Han:

1. Finds directories containing `bun.lock` or `bun.lockb`
2. Checks if files matching `**/*.ts` or `**/*.test.ts` have changed (caching is enabled by default)
3. Runs `bun test --only-failures` in each directory
4. Records the result and updates checkpoints

## `han hook list`

List available hooks from installed plugins.

### Usage

```bash
# List all available hooks
han hook list

# Filter by plugin
han hook list --plugin bun

# Show detailed information
han hook list --verbose
```

### Options

| Option | Description |
|--------|-------------|
| `--plugin <name>` | Filter hooks by plugin name |
| `--verbose` | Show detailed hook configuration |
| `--json` | Output as JSON for scripting |

### Output

```
Available Hooks:

  bun:
    test - Run Bun tests
    build - Build the Bun project

  typescript:
    typecheck - Type-check TypeScript code for type errors

  biome:
    lint - Lint Biome code for issues and style violations
```

### Examples

```bash
# List all hooks
han hook list

# List only Bun hooks
han hook list --plugin bun

# Get JSON output for scripting
han hook list --json | jq '.[] | select(.plugin == "bun")'
```

## `han hook explain`

Show detailed explanation of a hook's configuration and behavior.

### Usage

```bash
# Explain a specific hook
han hook explain <plugin-name> <hook-name>
```

### Example

```bash
han hook explain bun test
```

Output:

```
Hook: bun/test

Description: Run Bun tests

Command: bun test --only-failures

Directories: Runs in directories containing:
  - bun.lock
  - bun.lockb

File Patterns: Triggers when these files change:
  - **/*.ts
  - **/*.test.ts

Cache: Enabled by default (use --no-cache to disable)

Usage:
  han hook run bun test
  han hook run bun test --no-cache
  han hook run bun test --verbose --directory packages/core
```

## `han hook verify`

Verify hook configuration for all installed plugins.

### Usage

```bash
# Verify all plugin hooks
han hook verify

# Verify specific plugin
han hook verify --plugin bun
```

### Options

| Option | Description |
|--------|-------------|
| `--plugin <name>` | Verify hooks for specific plugin only |
| `--fix` | Attempt to fix common issues |

Checks for:

- Valid hook configuration syntax
- Command executability
- File pattern validity
- Directory detection logic

## `han hook test`

Test a hook configuration without actually running it.

### Usage

```bash
# Test which directories a hook would run in
han hook test <plugin-name> <hook-name>

# Test with specific directory
han hook test <plugin-name> <hook-name> --directory packages/core
```

Shows:

- Detected directories
- Files that would trigger execution (if `ifChanged` is set)
- Checkpoint status (if `--cache` would skip)

## Environment Variables

Hook execution respects these environment variables:

| Variable | Description |
|----------|-------------|
| `HAN_DISABLE_HOOKS` | Set to `1` or `true` to disable all hooks |
| `HAN_HOOK_RUN_VERBOSE` | Set to `1` or `true` to enable verbose output globally |
| `HAN_MCP_TIMEOUT` | Hook timeout in milliseconds (default: 600000 = 10 minutes) |

### Example

```bash
# Disable all hooks temporarily
export HAN_DISABLE_HOOKS=1
han hook run bun test  # Exits immediately without running

# Enable verbose output globally
export HAN_HOOK_RUN_VERBOSE=1
han hook run bun test  # Always shows full output
```

## Integration with Claude Code

Hooks run automatically at lifecycle events when configured in plugin `hooks.json`:

```json
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          { "type": "command", "command": "han hook run bun test" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "han hook run typescript typecheck" }
        ]
      }
    ]
  }
}
```

### Hook Lifecycle

Han supports these Claude Code hook points:

| Hook | When It Fires | Checkpoint Type | Purpose |
|------|---------------|-----------------|---------|
| `SessionStart` | Claude Code session begins | Creates session checkpoint | Initialize session state |
| `SubagentStart` | Subagent is spawned | Creates agent checkpoint | Capture pre-subagent state |
| `UserPromptSubmit` | User submits a prompt | N/A | Pre-process user input |
| `PreToolUse` | Before each tool call | N/A | Validate tool usage |
| `PermissionRequest` | Permission dialog appears | N/A | Audit/auto-approve permissions |
| `PostToolUse` | After each tool call | N/A | Post-process tool results |
| `PostToolUseFailure` | Tool execution fails | N/A | Error tracking, recovery |
| `Stop` | Agent about to respond | Validates using session checkpoint | Validate all session changes |
| `SubagentStop` | Subagent completes | Validates using agent checkpoint | Validate subagent changes |
| `Notification` | Notification event | N/A | Custom notification handling |
| `PreCompact` | Before context compaction | N/A | Save state before compaction |
| `SessionEnd` | Session ends | N/A | Cleanup session state |
| `ConfigChange` | Configuration modified (2.1.49+) | N/A | Audit trails, config monitoring |
| `TeammateIdle` | Teammate goes idle (2.1.33+) | N/A | Team coordination |
| `TaskCompleted` | Task completed (2.1.33+) | N/A | Task tracking, workflows |
| `WorktreeCreate` | Worktree created (2.1.50+) | N/A | Agent isolation tracking, custom VCS |
| `WorktreeRemove` | Worktree removed (2.1.50+) | N/A | Cleanup automation |

### New Hook Events

#### PermissionRequest (~2.1.50+)

Fired when a permission dialog appears. Supports matcher on tool name. The input includes a `permission_suggestions` array. The hook can respond with `behavior` (`allow`/`deny`), `updatedInput`, `updatedPermissions`, `message`, or `interrupt`:

```json
{
  "hook_event_name": "PermissionRequest",
  "session_id": "abc123",
  "cwd": "/project/path",
  "tool_name": "Bash",
  "permission_suggestions": [...]
}
```

Useful for automated permission policies, security auditing, and CI/CD environments.

#### PostToolUseFailure (~2.1.50+)

Fired when a tool execution fails. The input includes `error` string and `is_interrupt` boolean. The hook can return `additionalContext` to help Claude recover:

```json
{
  "hook_event_name": "PostToolUseFailure",
  "session_id": "abc123",
  "cwd": "/project/path",
  "tool_name": "Bash",
  "error": "Command exited with code 1",
  "is_interrupt": false
}
```

Useful for error tracking, automatic recovery suggestions, and operational monitoring.

#### PreCompact (~2.1.50+)

Fired before context compaction. Supports matcher for `manual` vs `auto` compaction:

```json
{
  "hook_event_name": "PreCompact",
  "session_id": "abc123",
  "cwd": "/project/path"
}
```

Useful for saving state, injecting critical context to preserve, or logging compaction events.

#### ConfigChange (2.1.49+)

Fired when Claude Code configuration is modified. The stdin payload includes the changed configuration keys:

```json
{
  "hook_event_name": "ConfigChange",
  "session_id": "abc123",
  "cwd": "/project/path"
}
```

Useful for audit trails, configuration drift detection, and enforcing settings policies across teams.

#### TeammateIdle (2.1.33+)

Fired when a teammate agent goes idle between turns in multi-agent sessions:

```json
{
  "hook_event_name": "TeammateIdle",
  "session_id": "abc123",
  "cwd": "/project/path"
}
```

Enables team coordination, load balancing, and monitoring agent activity.

#### TaskCompleted (2.1.33+)

Fired when a task is marked as completed via `TaskUpdate`:

```json
{
  "hook_event_name": "TaskCompleted",
  "session_id": "abc123",
  "cwd": "/project/path"
}
```

Useful for task tracking dashboards, triggering follow-up workflows, and team notifications.

#### WorktreeCreate (2.1.50+)

Fired when a worktree is being created via `--worktree` flag or `isolation: "worktree"` in an agent definition. When a WorktreeCreate hook is configured, it **replaces the default git worktree behavior**, enabling support for non-git VCS systems (SVN, Perforce, Mercurial, etc.).

The hook receives a `name` slug and **must print the absolute path** to the created worktree directory on stdout. A non-zero exit code blocks worktree creation.

```json
{
  "hook_event_name": "WorktreeCreate",
  "session_id": "abc123",
  "cwd": "/project/path",
  "name": "feature-auth"
}
```

Only `type: "command"` hooks are supported. No matcher support.

#### WorktreeRemove (2.1.50+)

Fired when a worktree is being removed (session exit or subagent completion). The hook receives the `worktree_path` that was created. WorktreeRemove hooks **cannot block** removal — failures are logged in debug mode only.

```json
{
  "hook_event_name": "WorktreeRemove",
  "session_id": "abc123",
  "cwd": "/project/path",
  "worktree_path": "/project/.claude/worktrees/feature-auth"
}
```

Only `type: "command"` hooks are supported. No matcher support.

### Stop/SubagentStop: `last_assistant_message` (2.1.47+)

The `Stop` and `SubagentStop` hook inputs now include a `last_assistant_message` field containing the final assistant message text. This allows hooks to inspect what the agent is about to respond with:

```json
{
  "hook_event_name": "Stop",
  "session_id": "abc123",
  "cwd": "/project/path",
  "last_assistant_message": "I've completed the refactoring of the auth module..."
}
```

This is useful for content-aware validation, sentiment analysis, or logging the agent's final output.

### Checkpoint Filtering

Hooks automatically filter what files they check based on when they run:

- **Stop hooks** validate against the session checkpoint (created at `SessionStart`)
  - Only checks files modified during the entire session
  - Use for session-wide validations (tests, builds, linting)

- **SubagentStop hooks** validate against the agent checkpoint (created at `SubagentStart`)
  - Only checks files modified by that specific subagent
  - Use for focused validations (type checking, unit tests)

This ensures hooks only validate relevant changes and skip unchanged files automatically.

## Learn More

- [Plugin Commands](/docs/cli/plugins) - Managing plugin installation
- [Configuration](/docs/configuration) - Configuring hook behavior
- [MCP Integrations](/docs/integrations) - How hooks integrate with MCP
