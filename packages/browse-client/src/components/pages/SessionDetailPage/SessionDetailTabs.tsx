/**
 * Session Detail Tabs
 *
 * Tab bar for switching between Overview, Messages, Tasks, and Files.
 * Active tab syncs with URL search params for deep-linking.
 */

import type React from "react";
import { Box } from "@/components/atoms/Box.tsx";
import { HStack } from "@/components/atoms/HStack.tsx";
import { Pressable } from "@/components/atoms/Pressable.tsx";
import { Text } from "@/components/atoms/Text.tsx";
import { colors, fontSizes, spacing } from "@/theme.ts";

export type SessionTab = "overview" | "messages" | "tasks" | "files";

interface TabDef {
	key: SessionTab;
	label: string;
	count?: number | null;
}

interface SessionDetailTabsProps {
	activeTab: SessionTab;
	onTabChange: (tab: SessionTab) => void;
	messageCount: number;
	taskCount: number;
	fileCount: number;
}

export function SessionDetailTabs({
	activeTab,
	onTabChange,
	messageCount,
	taskCount,
	fileCount,
}: SessionDetailTabsProps): React.ReactElement {
	const tabs: TabDef[] = [
		{ key: "overview", label: "Overview" },
		{ key: "messages", label: "Messages", count: messageCount },
		{ key: "tasks", label: "Tasks", count: taskCount },
		{ key: "files", label: "Files", count: fileCount },
	];

	return (
		<Box
			style={{
				flexShrink: 0,
				borderBottom: `1px solid ${colors.border.default}`,
				backgroundColor: colors.bg.primary,
			}}
		>
			<HStack style={{ paddingLeft: spacing.lg }}>
				{tabs.map((tab) => {
					const isActive = activeTab === tab.key;
					return (
						<Pressable
							key={tab.key}
							onPress={() => onTabChange(tab.key)}
							style={{
								padding: `${spacing.sm}px ${spacing.md}px`,
								cursor: "pointer",
								borderBottom: isActive
									? `2px solid ${colors.primary}`
									: "2px solid transparent",
								marginBottom: -1,
								transition: "color 0.15s, border-color 0.15s",
							}}
						>
							<Text
								size="sm"
								style={{
									fontSize: fontSizes.sm,
									fontWeight: isActive ? 500 : 400,
									color: isActive ? colors.text.primary : colors.text.muted,
								}}
							>
								{tab.label}
								{tab.count != null && tab.count > 0 ? ` (${tab.count})` : ""}
							</Text>
						</Pressable>
					);
				})}
			</HStack>
		</Box>
	);
}
