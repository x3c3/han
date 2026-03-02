/**
 * Shared Formatting Utilities
 *
 * Consistent locale-aware formatting across the browse-client.
 */

const _numberFormatter = new Intl.NumberFormat();

/**
 * Format a number with abbreviated suffix (K, M, B) for compact display.
 */
export function formatCount(num: number): string {
	if (num >= 1_000_000_000) return `${(num / 1_000_000_000).toFixed(1)}B`;
	if (num >= 1_000_000) return `${(num / 1_000_000).toFixed(1)}M`;
	if (num >= 1_000) return `${(num / 1_000).toFixed(1)}K`;
	return num.toLocaleString();
}

/**
 * Format a number with locale-aware thousand separators (e.g., 108,407).
 * Use this for exact counts that should not be abbreviated.
 */
export function formatWholeNumber(num: number): string {
	return _numberFormatter.format(num);
}

/**
 * Format a duration in seconds to a human-readable string (e.g., "1h 23m", "45m", "2d 3h").
 */
export function formatDuration(seconds: number): string {
	if (seconds < 60) return `${seconds}s`;
	const minutes = Math.floor(seconds / 60);
	if (minutes < 60) return `${minutes}m`;
	const hours = Math.floor(minutes / 60);
	const remainingMinutes = minutes % 60;
	if (hours < 24) {
		return remainingMinutes > 0
			? `${hours}h ${remainingMinutes}m`
			: `${hours}h`;
	}
	const days = Math.floor(hours / 24);
	const remainingHours = hours % 24;
	return remainingHours > 0 ? `${days}d ${remainingHours}h` : `${days}d`;
}

/**
 * Format a duration in milliseconds to a human-readable string (e.g., "150ms", "1.2s", "3m 45s").
 */
export function formatDurationMs(ms: number | null | undefined): string {
	if (ms == null) return "N/A";
	if (ms < 1000) return `${ms}ms`;
	if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
	const mins = Math.floor(ms / 60000);
	const secs = Math.round((ms % 60000) / 1000);
	return `${mins}m ${secs}s`;
}

/**
 * Format relative time from a date string (e.g., "Just now", "5m ago", "3d ago").
 */
export function formatRelativeTime(dateStr: string | null | undefined): string {
	if (!dateStr) return "Never";
	const date = new Date(dateStr);
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffMins = Math.floor(diffMs / 60000);
	const diffHours = Math.floor(diffMs / 3600000);
	const diffDays = Math.floor(diffMs / 86400000);
	if (diffMins < 1) return "Just now";
	if (diffMins < 60) return `${diffMins}m ago`;
	if (diffHours < 24) return `${diffHours}h ago`;
	if (diffDays < 7) return `${diffDays}d ago`;
	return date.toLocaleDateString();
}

/**
 * Format an absolute date for display (e.g., "Jan 15, 2026, 02:30 PM").
 * Returns a fallback string for null/invalid input.
 */
export function formatAbsoluteDate(
	dateStr: string | null | undefined,
	fallback = "N/A",
): string {
	if (!dateStr) return fallback;
	const date = new Date(dateStr);
	if (Number.isNaN(date.getTime())) return fallback;
	return date.toLocaleDateString("en-US", {
		month: "short",
		day: "numeric",
		year: "numeric",
		hour: "2-digit",
		minute: "2-digit",
	});
}

/**
 * Pluralize a word based on count (e.g., `pluralize(1, "session")` → "1 session").
 * Optionally provide a custom plural form.
 */
export function pluralize(
	count: number,
	singular: string,
	plural?: string,
): string {
	const form = count === 1 ? singular : (plural ?? `${singular}s`);
	return `${count} ${form}`;
}

/**
 * Strip XML/HTML-like tags from text (e.g., `<command-name>/clear</command-name>` → `/clear`).
 * Also handles truncated tags (e.g., `</com...`) left by backend summary truncation.
 * Collapses whitespace and trims the result.
 */
export function stripXmlTags(text: string): string {
	return (
		text
			// Complete tags: <tag> or </tag>
			.replace(/<[^>]+>/g, "")
			// Truncated tags at end: </com... or <command-na...
			.replace(/<\/?[\w-]*\.{3}$/g, "")
			// Truncated tags mid-text: </com... followed by more content
			.replace(/<\/?[\w-]*\.{3}/g, "")
			.replace(/\s+/g, " ")
			.trim()
	);
}

/**
 * Clean a session summary for display. Returns null if the summary is not meaningful.
 *
 * Handles common artifacts:
 * - XML tags and truncated tags (stripped)
 * - Slash commands like "/clear clear" or "/mcp mcp" (removed)
 * - Command wrapper patterns like "green-man /green-man mine" (removed)
 * - Tool call IDs like "toolu_01XJh..." (removed)
 * - Very short remnants (< 8 chars after cleaning)
 */
export function cleanSessionSummary(
	summary: string | null | undefined,
): string | null {
	if (!summary) return null;

	let cleaned = stripXmlTags(summary);

	// Remove tool call IDs (toolu_xxxx patterns)
	cleaned = cleaned.replace(/\btoolu_\w+\b/g, "").trim();

	// Remove hex hash prefixes (e.g., "bb39204" at the start)
	cleaned = cleaned.replace(/^[0-9a-f]{7,40}\s+/i, "").trim();

	// If what's left is just a slash command (e.g., "/clear clear", "/mcp mcp"), discard
	if (/^\/\S+(\s+\S+)?\s*$/.test(cleaned)) return null;

	// Detect command wrapper: "word /word args" where the /word repeats the first word
	// e.g., "green-man /green-man mine" from <command-message>green-man</command-message><command-name>/green-man</command-name>
	if (/^\S+\s+\/\S+(\s+\S+)?\s*$/.test(cleaned)) return null;

	// Collapse whitespace again after removals
	cleaned = cleaned.replace(/\s+/g, " ").trim();

	// Too short to be meaningful
	if (cleaned.length < 8) return null;

	return cleaned;
}
