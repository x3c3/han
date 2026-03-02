/**
 * Task Type Badge Molecule
 *
 * Displays a badge for task type with appropriate color.
 * Maps IMPLEMENTATION/FIX/REFACTOR/RESEARCH to badge variants.
 */

import type React from "react";
import { Badge } from "../atoms/index.ts";

interface TaskTypeBadgeProps {
	type: string;
}

export function TaskTypeBadge({
	type,
}: TaskTypeBadgeProps): React.ReactElement {
	const variants: Record<
		string,
		"default" | "success" | "warning" | "danger" | "purple"
	> = {
		IMPLEMENTATION: "default",
		FIX: "danger",
		REFACTOR: "purple",
		RESEARCH: "success",
	};
	return (
		<Badge variant={variants[type] || "default"}>{type.toLowerCase()}</Badge>
	);
}
