#!/usr/bin/env bash
# Consolidated session reference output.
# Replaces 8 separate `han hook reference` calls with a single process.
set -euo pipefail

PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-}"

if [ -z "$PLUGIN_ROOT" ]; then
  echo "CLAUDE_PLUGIN_ROOT not set" >&2
  exit 1
fi

# Each entry: relative_path|reason
references=(
  "hooks/no-time-estimates.md|no time estimates policy"
  "hooks/professional-honesty.md|epistemic rigor and professional honesty"
  "hooks/ensure-subagent.md|subagent delegation rules"
  "hooks/bash-output-capture.md|bash output capture best practices"
  "hooks/ensure-skill-use.md|skill selection and transparency"
  "hooks/date-handling.md|date handling best practices"
  "hooks/prefer-mcp-tools.md|prefer MCP tools over CLI tools"
  "hooks/warn-untested-modification.md|untested code modification safety"
)

for entry in "${references[@]}"; do
  rel_path="${entry%%|*}"
  reason="${entry#*|}"
  full_path="${PLUGIN_ROOT}/${rel_path}"
  if [ -f "$full_path" ]; then
    echo "<must-read-first reason=\"${reason}\">${full_path}</must-read-first>"
  fi
done
