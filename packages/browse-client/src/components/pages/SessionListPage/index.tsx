/**
 * Session List Page
 *
 * Displays Claude Code sessions with filtering and virtualized scrolling.
 * Uses Relay for data fetching with proper pagination via usePaginationFragment.
 */

import type React from "react";
import { graphql } from "react-relay";
import { useParams } from "react-router-dom";
import { PageLoader } from "@/components/helpers";
import type { SessionListPageQuery as SessionListPageQueryType } from "./__generated__/SessionListPageQuery.graphql.ts";
import { SessionsContent } from "./SessionsContent.tsx";

/**
 * Top-level query that spreads the pagination fragment
 */
export const SessionListPageQuery = graphql`
  query SessionListPageQuery(
    $first: Int
    $filter: SessionFilter
  ) {
    ...SessionsContent_query
      @arguments(
        first: $first
        filter: $filter
      )
  }
`;

/**
 * Session list page component with PageLoader for query preloading
 *
 * Supports multiple route contexts:
 * - /sessions (global)
 * - /repos/:repoId/sessions (repo-scoped via association filter)
 * - /projects/:projectId/sessions (project-scoped)
 * - /repos/:repoId/worktrees/:worktreeName/sessions (worktree-scoped)
 */
export default function SessionListPage(): React.ReactElement {
	const { projectId, repoId, worktreeName } = useParams<{
		projectId?: string;
		repoId?: string;
		worktreeName?: string;
	}>();

	// Build filter using GreenFairy pattern:
	// - repoId → association filter through project.repoId
	// - projectId → direct field filter on session.projectId
	const filter = repoId
		? { project: { repoId: { _eq: repoId } } }
		: projectId
			? { projectId: { _eq: projectId } }
			: undefined;

	return (
		<PageLoader<SessionListPageQueryType>
			query={SessionListPageQuery}
			variables={{
				first: 50,
				filter: filter ?? null,
			}}
			loadingMessage="Loading sessions..."
		>
			{(queryRef) => (
				<SessionsContent
					queryRef={queryRef}
					projectId={projectId ?? repoId ?? null}
					worktreeName={worktreeName ?? null}
				/>
			)}
		</PageLoader>
	);
}
