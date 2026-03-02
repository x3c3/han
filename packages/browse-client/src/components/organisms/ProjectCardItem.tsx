/**
 * Project Card Item Organism
 *
 * Displays a project in a card format for grid/list views.
 */

import type { CSSProperties } from "react";
import { useNavigate } from "react-router-dom";
import { Badge, Card, Heading, HStack, Text, VStack } from "../atoms/index.ts";
import { formatRelativeTime } from "../helpers/formatters.ts";

interface Project {
	id: string;
	name: string;
	path: string;
	sessionCount: number;
	lastActivity?: string;
	worktreeCount?: number;
}

interface ProjectCardItemProps {
	project: Project;
	style?: CSSProperties;
}

export function ProjectCardItem({ project, style }: ProjectCardItemProps) {
	const navigate = useNavigate();

	const handleClick = () => {
		navigate(`/repos/${encodeURIComponent(project.id)}`);
	};

	return (
		<Card onClick={handleClick} hoverable style={{ height: "100%", ...style }}>
			<VStack gap="sm" style={{ height: "100%" }}>
				<Heading size="sm">{project.name}</Heading>
				<Text size="sm" color="secondary" truncate>
					{project.path}
				</Text>
				<HStack gap="sm" style={{ marginTop: "auto" }}>
					<Badge variant="default">{project.sessionCount} sessions</Badge>
					{project.worktreeCount && project.worktreeCount > 0 && (
						<Badge variant="purple">{project.worktreeCount} worktrees</Badge>
					)}
				</HStack>
				<Text size="xs" color="muted">
					{formatRelativeTime(project.lastActivity)}
				</Text>
			</VStack>
		</Card>
	);
}
