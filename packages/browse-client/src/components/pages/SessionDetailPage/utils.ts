/**
 * Utility functions for session detail components
 */

export { formatRelativeTime } from "@/components/helpers/formatters.ts";

import { formatAbsoluteDate } from "@/components/helpers/formatters.ts";

/**
 * Format a date for display (delegates to shared formatAbsoluteDate)
 */
export function formatDate(dateStr: string | null): string {
	return formatAbsoluteDate(dateStr, "-");
}

/**
 * Format time duration between two dates
 */
export function formatDuration(
	startedAt: string | null,
	updatedAt: string | null,
): string {
	if (!startedAt || !updatedAt) return "-";

	const start = new Date(startedAt);
	const end = new Date(updatedAt);
	const diffMs = end.getTime() - start.getTime();
	const diffMins = Math.floor(diffMs / 60000);
	const diffHours = Math.floor(diffMs / 3600000);

	if (diffMins < 1) return "< 1m";
	if (diffMins < 60) return `${diffMins}m`;
	if (diffHours < 24) return `${diffHours}h ${diffMins % 60}m`;
	return `${Math.floor(diffHours / 24)}d ${diffHours % 24}h`;
}

/**
 * Format task duration in seconds
 */
export function formatTaskDuration(seconds: number | null): string {
	if (seconds === null) return "-";
	if (seconds < 60) return `${seconds}s`;
	const mins = Math.floor(seconds / 60);
	const secs = seconds % 60;
	if (mins < 60) return `${mins}m ${secs}s`;
	const hours = Math.floor(mins / 60);
	return `${hours}h ${mins % 60}m`;
}

/**
 * Format milliseconds as human-readable duration
 */
export function formatMs(ms: number): string {
	if (ms < 1000) return `${ms}ms`;

	const seconds = Math.floor(ms / 1000);
	if (seconds < 60) return `${seconds}s`;

	const minutes = Math.floor(seconds / 60);
	const remainingSecs = seconds % 60;
	if (minutes < 60) {
		return remainingSecs > 0 ? `${minutes}m ${remainingSecs}s` : `${minutes}m`;
	}

	const hours = Math.floor(minutes / 60);
	const remainingMins = minutes % 60;
	if (hours < 24) {
		return remainingMins > 0 ? `${hours}h ${remainingMins}m` : `${hours}h`;
	}

	const days = Math.floor(hours / 24);
	const remainingHours = hours % 24;
	return remainingHours > 0 ? `${days}d ${remainingHours}h` : `${days}d`;
}
