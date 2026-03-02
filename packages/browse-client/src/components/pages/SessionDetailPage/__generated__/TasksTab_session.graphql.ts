/**
 * @generated SignedSource<<1bd0745cad1772793e1b3f9da10f1923>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
export type TaskStatus = "ACTIVE" | "COMPLETED" | "FAILED" | "%future added value";
export type TaskType = "FIX" | "IMPLEMENTATION" | "REFACTOR" | "RESEARCH" | "%future added value";
export type TodoStatus = "completed" | "in_progress" | "pending" | "%future added value";
import { FragmentRefs } from "relay-runtime";
export type TasksTab_session$data = {
  readonly nativeTasks: ReadonlyArray<{
    readonly activeForm: string | null | undefined;
    readonly blockedBy: ReadonlyArray<string> | null | undefined;
    readonly blocks: ReadonlyArray<string> | null | undefined;
    readonly completedAt: string | null | undefined;
    readonly createdAt: string;
    readonly description: string | null | undefined;
    readonly id: string;
    readonly messageId: string;
    readonly owner: string | null | undefined;
    readonly sessionId: string;
    readonly status: string;
    readonly subject: string;
    readonly updatedAt: string;
  }>;
  readonly tasks: {
    readonly edges: ReadonlyArray<{
      readonly node: {
        readonly completedAt: string | null | undefined;
        readonly confidence: number | null | undefined;
        readonly description: string;
        readonly durationSeconds: number | null | undefined;
        readonly id: string;
        readonly outcome: string | null | undefined;
        readonly startedAt: string;
        readonly status: TaskStatus;
        readonly taskId: string;
        readonly type: TaskType | null | undefined;
      };
    }>;
    readonly totalCount: number;
  } | null | undefined;
  readonly todoCounts: {
    readonly completed: number | null | undefined;
    readonly inProgress: number | null | undefined;
    readonly pending: number | null | undefined;
    readonly total: number | null | undefined;
  } | null | undefined;
  readonly todos: {
    readonly edges: ReadonlyArray<{
      readonly node: {
        readonly activeForm: string | null | undefined;
        readonly content: string | null | undefined;
        readonly id: string | null | undefined;
        readonly status: TodoStatus | null | undefined;
      };
    }>;
    readonly totalCount: number;
  } | null | undefined;
  readonly " $fragmentType": "TasksTab_session";
};
export type TasksTab_session$key = {
  readonly " $data"?: TasksTab_session$data;
  readonly " $fragmentSpreads": FragmentRefs<"TasksTab_session">;
};

const node: ReaderFragment = (function(){
var v0 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
},
v1 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "description",
  "storageKey": null
},
v2 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "status",
  "storageKey": null
},
v3 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "activeForm",
  "storageKey": null
},
v4 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "completedAt",
  "storageKey": null
},
v5 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "totalCount",
  "storageKey": null
};
return {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "TasksTab_session",
  "selections": [
    {
      "alias": null,
      "args": null,
      "concreteType": "NativeTask",
      "kind": "LinkedField",
      "name": "nativeTasks",
      "plural": true,
      "selections": [
        (v0/*: any*/),
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "sessionId",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "messageId",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "subject",
          "storageKey": null
        },
        (v1/*: any*/),
        (v2/*: any*/),
        (v3/*: any*/),
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "owner",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "blocks",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "blockedBy",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "createdAt",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "updatedAt",
          "storageKey": null
        },
        (v4/*: any*/)
      ],
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "concreteType": "TodoConnection",
      "kind": "LinkedField",
      "name": "todos",
      "plural": false,
      "selections": [
        (v5/*: any*/),
        {
          "alias": null,
          "args": null,
          "concreteType": "TodoEdge",
          "kind": "LinkedField",
          "name": "edges",
          "plural": true,
          "selections": [
            {
              "alias": null,
              "args": null,
              "concreteType": "Todo",
              "kind": "LinkedField",
              "name": "node",
              "plural": false,
              "selections": [
                (v0/*: any*/),
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "content",
                  "storageKey": null
                },
                (v2/*: any*/),
                (v3/*: any*/)
              ],
              "storageKey": null
            }
          ],
          "storageKey": null
        }
      ],
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "concreteType": "TodoCounts",
      "kind": "LinkedField",
      "name": "todoCounts",
      "plural": false,
      "selections": [
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "total",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "pending",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "inProgress",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "completed",
          "storageKey": null
        }
      ],
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "concreteType": "TaskConnection",
      "kind": "LinkedField",
      "name": "tasks",
      "plural": false,
      "selections": [
        (v5/*: any*/),
        {
          "alias": null,
          "args": null,
          "concreteType": "TaskEdge",
          "kind": "LinkedField",
          "name": "edges",
          "plural": true,
          "selections": [
            {
              "alias": null,
              "args": null,
              "concreteType": "Task",
              "kind": "LinkedField",
              "name": "node",
              "plural": false,
              "selections": [
                (v0/*: any*/),
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "taskId",
                  "storageKey": null
                },
                (v1/*: any*/),
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "type",
                  "storageKey": null
                },
                (v2/*: any*/),
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "outcome",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "confidence",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "startedAt",
                  "storageKey": null
                },
                (v4/*: any*/),
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "durationSeconds",
                  "storageKey": null
                }
              ],
              "storageKey": null
            }
          ],
          "storageKey": null
        }
      ],
      "storageKey": null
    }
  ],
  "type": "Session",
  "abstractKey": null
};
})();

(node as any).hash = "01f7c6f229402b435c0c0bcc0fb20ea0";

export default node;
