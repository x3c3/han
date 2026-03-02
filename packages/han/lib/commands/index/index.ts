/**
 * Han Reindex Command
 *
 * Clears all derived database tables and re-indexes from JSONL logs.
 * All data is event-sourced from logs, so this is a safe operation.
 *
 * Usage:
 *   han reindex              # Clear database and reindex
 *   han reindex -v           # With verbose output
 */

import type { Command } from 'commander';
import { type IndexLayer, searchAll } from '../../memory/indexer.ts';
import { getGitRemote } from '../../memory/paths.ts';

/**
 * Register the reindex command
 */
export function registerReindexCommand(program: Command): void {
  const reindexCommand = program
    .command('reindex')
    .description('Clear database and reindex from JSONL logs');

  // Main reindex action (default)
  reindexCommand
    .command('run', { isDefault: true })
    .description('Clear database and reindex from logs')
    .option('-v, --verbose', 'Show detailed progress')
    .action(async (options: { verbose?: boolean }) => {
      try {
        const { existsSync, unlinkSync, statSync } = await import('node:fs');
        const { join } = await import('node:path');
        const { getHanDataDir } = await import(
          '../../config/claude-settings.ts'
        );

        const dataDir = getHanDataDir();
        const dbPath = join(dataDir, 'han.db');
        const dbWalPath = `${dbPath}-wal`;
        const dbShmPath = `${dbPath}-shm`;

        // Step 1: Stop the coordinator if running
        if (options.verbose) {
          console.log('Stopping coordinator...');
        }
        try {
          const { stopDaemon } = await import('../coordinator/daemon.ts');
          await stopDaemon();
        } catch {
          if (options.verbose) {
            console.log('  Coordinator not running or already stopped');
          }
        }

        // Step 2: Delete the database file (all data is derived from JSONL)
        if (existsSync(dbPath)) {
          const stats = statSync(dbPath);
          const sizeMb = (stats.size / 1024 / 1024).toFixed(2);

          unlinkSync(dbPath);
          // Clean up WAL and SHM files if present
          try {
            if (existsSync(dbWalPath)) unlinkSync(dbWalPath);
          } catch {
            /* ignore */
          }
          try {
            if (existsSync(dbShmPath)) unlinkSync(dbShmPath);
          } catch {
            /* ignore */
          }

          console.log(`Deleted database (${sizeMb} MB): ${dbPath}`);
        } else {
          console.log('No database found - nothing to clear');
        }

        // Step 3: Re-index memory store
        const { runIndex } = await import('../../memory/indexer.ts');
        type IndexOptions = import('../../memory/indexer.ts').IndexOptions;
        const gitRemote = getGitRemote() || undefined;

        const indexOptions: IndexOptions = {
          gitRemote,
          verbose: options.verbose,
        };

        if (options.verbose) {
          console.log('\nRe-indexing memory store...');
          if (gitRemote) {
            console.log(`Project: ${gitRemote}`);
          }
        }

        const results = await runIndex(indexOptions);

        const totalIndexed =
          results.observations +
          results.summaries +
          results.team +
          results.transcripts;

        if (options.verbose || totalIndexed > 0) {
          console.log('\nMemory indexing complete:');
          if (results.observations > 0) {
            console.log(`  Observations: ${results.observations} documents`);
          }
          if (results.summaries > 0) {
            console.log(`  Summaries: ${results.summaries} documents`);
          }
          if (results.team > 0) {
            console.log(`  Team memory: ${results.team} documents`);
          }
          if (results.transcripts > 0) {
            console.log(`  Transcripts: ${results.transcripts} documents`);
          }
          if (totalIndexed === 0) {
            console.log('  No new documents to index');
          }
        }

        console.log(
          '\nDone. The coordinator will rebuild session data from JSONL on next start.'
        );
        console.log('Run `han browse` to start the coordinator and re-index.');

        process.exit(0);
      } catch (error: unknown) {
        console.error(
          'Error during reindex:',
          error instanceof Error ? error.message : error
        );
        process.exit(1);
      }
    });

  // Search command for testing the index
  reindexCommand
    .command('search <query>')
    .description('Search indexed content (for testing)')
    .option(
      '--layer <layer>',
      'Search specific layer (observations, summaries, transcripts, team)'
    )
    .option('--limit <n>', 'Maximum results', '10')
    .action(
      async (query: string, options: { layer?: string; limit: string }) => {
        try {
          const gitRemote = getGitRemote() || undefined;
          const limit = Number.parseInt(options.limit, 10);

          const layers = options.layer
            ? [options.layer as IndexLayer]
            : undefined;

          const results = await searchAll(query, {
            layers,
            gitRemote,
            limit,
          });

          if (results.length === 0) {
            console.log('No results found');
            process.exit(0);
          }

          console.log(`Found ${results.length} results:\n`);

          for (const result of results) {
            const meta = result.metadata || {};
            const layer = meta.layer || 'unknown';
            const score = result.score.toFixed(3);

            console.log(`[${layer}] Score: ${score}`);
            console.log(`  ID: ${result.id}`);

            // Truncate content for display
            const content =
              result.content.length > 200
                ? `${result.content.slice(0, 200)}...`
                : result.content;
            console.log(`  ${content.replace(/\n/g, '\n  ')}`);
            console.log();
          }

          process.exit(0);
        } catch (error: unknown) {
          console.error(
            'Error searching:',
            error instanceof Error ? error.message : error
          );
          process.exit(1);
        }
      }
    );

  // Status command to check index health
  reindexCommand
    .command('status')
    .description('Show index status')
    .action(async () => {
      try {
        const { getIndexDbPath } = await import('../../memory/indexer.ts');
        const { existsSync, statSync } = await import('node:fs');

        const dbPath = getIndexDbPath();
        const exists = existsSync(dbPath);

        console.log('Index Status:');
        console.log(`  Database: ${dbPath}`);
        console.log(`  Exists: ${exists ? 'Yes' : 'No'}`);

        if (exists) {
          const stats = statSync(dbPath);
          const sizeMb = (stats.size / 1024 / 1024).toFixed(2);
          console.log(`  Size: ${sizeMb} MB`);
          console.log(`  Modified: ${stats.mtime.toISOString()}`);
        }

        const gitRemote = getGitRemote();
        console.log(`  Git remote: ${gitRemote || 'Not in a git repo'}`);

        process.exit(0);
      } catch (error: unknown) {
        console.error(
          'Error getting status:',
          error instanceof Error ? error.message : error
        );
        process.exit(1);
      }
    });
}
