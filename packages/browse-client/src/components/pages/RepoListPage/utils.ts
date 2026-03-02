/**
 * Utility functions for RepoListPage
 */

/**
 * Format a git remote URL for display.
 * e.g., "git@github.com:TheBushidoCollective/han.git" -> "github.com/TheBushidoCollective/han"
 *        "https://github.com/org/repo.git" -> "github.com/org/repo"
 */
export function formatRepoUrl(remote: string): string {
	let url = remote;
	// Strip trailing .git
	url = url.replace(/\.git$/, "");
	// Handle SSH format: git@host:path
	const sshMatch = url.match(/^git@([^:]+):(.+)$/);
	if (sshMatch) {
		return `${sshMatch[1]}/${sshMatch[2]}`;
	}
	// Handle HTTPS format: https://host/path
	const httpsMatch = url.match(/^https?:\/\/(.+)$/);
	if (httpsMatch) {
		return httpsMatch[1];
	}
	return url;
}
