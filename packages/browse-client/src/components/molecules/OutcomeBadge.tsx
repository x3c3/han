/**
 * Outcome Badge Molecule
 *
 * Displays a badge for task outcome with appropriate color.
 * Maps SUCCESS/PARTIAL/FAILURE to success/warning/danger badge variants.
 */

import type React from "react";
import { Badge } from "../atoms/index.ts";

interface OutcomeBadgeProps {
	outcome: string | null | undefined;
}

export function OutcomeBadge({
	outcome,
}: OutcomeBadgeProps): React.ReactElement {
	if (!outcome) return <Badge variant="default">-</Badge>;

	const variants: Record<
		string,
		"default" | "success" | "warning" | "danger" | "purple"
	> = {
		SUCCESS: "success",
		PARTIAL: "warning",
		FAILURE: "danger",
	};
	return (
		<Badge variant={variants[outcome] || "default"}>
			{outcome.toLowerCase()}
		</Badge>
	);
}
