/**
 * Repo Card Component
 *
 * Displays a git repository with session counts and navigation.
 */

import type React from "react";
import { Link } from "react-router-dom";
import { Badge } from "@/components/atoms/Badge.tsx";
import { Card } from "@/components/atoms/Card.tsx";
import { Heading } from "@/components/atoms/Heading.tsx";
import { HStack } from "@/components/atoms/HStack.tsx";
import { Text } from "@/components/atoms/Text.tsx";
import { VStack } from "@/components/atoms/VStack.tsx";
import {
	formatRelativeTime,
	pluralize,
} from "@/components/helpers/formatters.ts";
import type { Repo } from "./types.ts";
import { formatRepoUrl } from "./utils.ts";

interface RepoCardProps {
	repo: Repo;
}

export function RepoCard({ repo }: RepoCardProps): React.ReactElement {
	// Build repo detail URL using git remote-based repoId
	const repoUrl = `/repos/${repo.repoId}`;

	return (
		<Link
			to={repoUrl}
			style={{
				textDecoration: "none",
				color: "inherit",
				display: "block",
				height: "100%",
			}}
		>
			<Card hoverable style={{ height: "100%" }}>
				<VStack gap="sm" style={{ height: "100%" }}>
					{/* Header with name and activity time */}
					<HStack justify="space-between" align="flex-start" gap="sm">
						<VStack gap="xs" style={{ flex: 1, minWidth: 0 }}>
							<Heading size="sm">{repo.name}</Heading>
							<Text size="xs" color="muted" truncate>
								{formatRepoUrl(repo.path)}
							</Text>
						</VStack>
						<Text size="xs" color="secondary">
							{formatRelativeTime(repo.lastActivity)}
						</Text>
					</HStack>

					{/* Stats badges */}
					<HStack gap="sm" wrap style={{ marginTop: "auto" }}>
						<Badge variant="default">
							{pluralize(repo.totalSessions, "session")}
						</Badge>
					</HStack>
				</VStack>
			</Card>
		</Link>
	);
}
