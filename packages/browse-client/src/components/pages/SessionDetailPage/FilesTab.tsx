/**
 * Files Tab
 *
 * File changes grouped by action with validation status and pagination.
 */

import type React from "react";
import { type ReactElement, Suspense } from "react";
import { graphql, usePaginationFragment } from "react-relay";
import { Box } from "@/components/atoms/Box.tsx";
import { Button } from "@/components/atoms/Button.tsx";
import { HStack } from "@/components/atoms/HStack.tsx";
import { Spinner } from "@/components/atoms/Spinner.tsx";
import { Text } from "@/components/atoms/Text.tsx";
import { VStack } from "@/components/atoms/VStack.tsx";
import { colors, spacing } from "@/theme.ts";
import type { FilesTab_session$key } from "./__generated__/FilesTab_session.graphql.ts";
import type { FilesTabRefetchQuery } from "./__generated__/FilesTabRefetchQuery.graphql.ts";
import { FileChangeCard } from "./components.ts";

/**
 * Pagination fragment for file changes
 */
const FilesTabFragment = graphql`
  fragment FilesTab_session on Session
  @refetchable(queryName: "FilesTabRefetchQuery")
  @argumentDefinitions(
    first: { type: "Int", defaultValue: 50 }
    after: { type: "String" }
  ) {
    fileChanges(first: $first, after: $after)
      @connection(key: "FilesTab_fileChanges") {
      totalCount
      edges {
        node {
          id
          filePath
          action
          toolName
          recordedAt
          isValidated
          validations {
            pluginName
            hookName
            validatedAt
          }
          missingValidations {
            pluginName
            hookName
          }
        }
      }
      pageInfo {
        hasNextPage
        endCursor
      }
    }
    fileChangeCount
  }
`;

interface FilesTabProps {
	fragmentRef: FilesTab_session$key;
}

function FilesTabContent({ fragmentRef }: FilesTabProps): ReactElement {
	const { data, loadNext, hasNext, isLoadingNext } = usePaginationFragment<
		FilesTabRefetchQuery,
		FilesTab_session$key
	>(FilesTabFragment, fragmentRef);

	const allFileChanges = (data.fileChanges?.edges ?? [])
		.map((edge) => edge?.node)
		.filter(
			(node): node is NonNullable<typeof node> & { id: string } =>
				node != null && !!node.id,
		);

	// Deduplicate by filePath — keep the latest entry per unique path
	const filesByPath = new Map<string, (typeof allFileChanges)[number]>();
	for (const fc of allFileChanges) {
		const path = fc.filePath ?? "";
		const existing = filesByPath.get(path);
		if (!existing || (fc.recordedAt ?? "") > (existing.recordedAt ?? "")) {
			filesByPath.set(path, fc);
		}
	}
	const fileChanges = Array.from(filesByPath.values());

	const createdFiles = fileChanges.filter((f) => f.action === "created");
	const modifiedFiles = fileChanges.filter((f) => f.action === "modified");
	const deletedFiles = fileChanges.filter((f) => f.action === "deleted");

	if (fileChanges.length === 0) {
		return (
			<VStack align="center" gap="sm" style={{ padding: spacing.lg }}>
				<Text color="muted">No file changes in this session.</Text>
			</VStack>
		);
	}

	const mapAction = (
		action: string | null | undefined,
	): "CREATED" | "MODIFIED" | "DELETED" => {
		switch (action) {
			case "created":
				return "CREATED";
			case "deleted":
				return "DELETED";
			default:
				return "MODIFIED";
		}
	};

	const renderFileCard = (fileChange: (typeof fileChanges)[number]) => (
		<FileChangeCard
			key={fileChange.id}
			fileChange={{
				id: fileChange.id,
				filePath: fileChange.filePath ?? "",
				action: mapAction(fileChange.action),
				toolName: fileChange.toolName ?? null,
				recordedAt: fileChange.recordedAt ?? null,
				isValidated: fileChange.isValidated ?? false,
				validations: fileChange.validations ?? [],
				missingValidations: fileChange.missingValidations ?? [],
			}}
		/>
	);

	return (
		<Box
			style={{
				flex: 1,
				overflowY: "auto",
				padding: spacing.lg,
			}}
		>
			<VStack gap="lg">
				{/* Summary row */}
				<HStack gap="lg" style={{ flexWrap: "wrap" }}>
					{createdFiles.length > 0 && (
						<Text size="sm" style={{ color: colors.success }}>
							+{createdFiles.length} created
						</Text>
					)}
					{modifiedFiles.length > 0 && (
						<Text size="sm" style={{ color: colors.warning }}>
							~{modifiedFiles.length} modified
						</Text>
					)}
					{deletedFiles.length > 0 && (
						<Text size="sm" style={{ color: colors.danger }}>
							-{deletedFiles.length} deleted
						</Text>
					)}
				</HStack>

				{/* Created files group */}
				{createdFiles.length > 0 && (
					<VStack gap="sm">
						<Text size="sm" weight="medium" style={{ color: colors.success }}>
							Created ({createdFiles.length})
						</Text>
						<VStack gap="xs">{createdFiles.map(renderFileCard)}</VStack>
					</VStack>
				)}

				{/* Modified files group */}
				{modifiedFiles.length > 0 && (
					<VStack gap="sm">
						<Text size="sm" weight="medium" style={{ color: colors.warning }}>
							Modified ({modifiedFiles.length})
						</Text>
						<VStack gap="xs">{modifiedFiles.map(renderFileCard)}</VStack>
					</VStack>
				)}

				{/* Deleted files group */}
				{deletedFiles.length > 0 && (
					<VStack gap="sm">
						<Text size="sm" weight="medium" style={{ color: colors.danger }}>
							Deleted ({deletedFiles.length})
						</Text>
						<VStack gap="xs">{deletedFiles.map(renderFileCard)}</VStack>
					</VStack>
				)}

				{/* Load more */}
				{hasNext && (
					<Button
						variant="secondary"
						size="sm"
						onClick={() => loadNext(50)}
						disabled={isLoadingNext}
						style={{ alignSelf: "center" }}
					>
						{isLoadingNext ? "Loading..." : "Load More Files"}
					</Button>
				)}
			</VStack>
		</Box>
	);
}

export function FilesTab(props: FilesTabProps): React.ReactElement {
	return (
		<Suspense
			fallback={
				<VStack align="center" gap="sm" style={{ padding: spacing.lg }}>
					<Spinner size="sm" />
					<Text color="muted" size="sm">
						Loading files...
					</Text>
				</VStack>
			}
		>
			<FilesTabContent {...props} />
		</Suspense>
	);
}
