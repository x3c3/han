/**
 * @generated SignedSource<<305056f5075c22d327eb98c1a7875b90>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ReaderFragment } from 'relay-runtime';
export type FileChangeAction = "created" | "deleted" | "modified" | "%future added value";
import { FragmentRefs } from "relay-runtime";
export type FilesTab_session$data = {
  readonly fileChangeCount: number | null | undefined;
  readonly fileChanges: {
    readonly edges: ReadonlyArray<{
      readonly node: {
        readonly action: FileChangeAction | null | undefined;
        readonly filePath: string | null | undefined;
        readonly id: string | null | undefined;
        readonly isValidated: boolean | null | undefined;
        readonly missingValidations: ReadonlyArray<{
          readonly hookName: string | null | undefined;
          readonly pluginName: string | null | undefined;
        }> | null | undefined;
        readonly recordedAt: string | null | undefined;
        readonly toolName: string | null | undefined;
        readonly validations: ReadonlyArray<{
          readonly hookName: string | null | undefined;
          readonly pluginName: string | null | undefined;
          readonly validatedAt: string | null | undefined;
        }> | null | undefined;
      };
    }>;
    readonly pageInfo: {
      readonly endCursor: string | null | undefined;
      readonly hasNextPage: boolean;
    };
    readonly totalCount: number;
  } | null | undefined;
  readonly id: string;
  readonly " $fragmentType": "FilesTab_session";
};
export type FilesTab_session$key = {
  readonly " $data"?: FilesTab_session$data;
  readonly " $fragmentSpreads": FragmentRefs<"FilesTab_session">;
};

import FilesTabRefetchQuery_graphql from './FilesTabRefetchQuery.graphql';

const node: ReaderFragment = (function(){
var v0 = [
  "fileChanges"
],
v1 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
},
v2 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "pluginName",
  "storageKey": null
},
v3 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "hookName",
  "storageKey": null
};
return {
  "argumentDefinitions": [
    {
      "defaultValue": null,
      "kind": "LocalArgument",
      "name": "after"
    },
    {
      "defaultValue": 50,
      "kind": "LocalArgument",
      "name": "first"
    }
  ],
  "kind": "Fragment",
  "metadata": {
    "connection": [
      {
        "count": "first",
        "cursor": "after",
        "direction": "forward",
        "path": (v0/*: any*/)
      }
    ],
    "refetch": {
      "connection": {
        "forward": {
          "count": "first",
          "cursor": "after"
        },
        "backward": null,
        "path": (v0/*: any*/)
      },
      "fragmentPathInResult": [
        "node"
      ],
      "operation": FilesTabRefetchQuery_graphql,
      "identifierInfo": {
        "identifierField": "id",
        "identifierQueryVariableName": "id"
      }
    }
  },
  "name": "FilesTab_session",
  "selections": [
    {
      "alias": "fileChanges",
      "args": null,
      "concreteType": "FileChangeConnection",
      "kind": "LinkedField",
      "name": "__FilesTab_fileChanges_connection",
      "plural": false,
      "selections": [
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "totalCount",
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "concreteType": "FileChangeEdge",
          "kind": "LinkedField",
          "name": "edges",
          "plural": true,
          "selections": [
            {
              "alias": null,
              "args": null,
              "concreteType": "FileChange",
              "kind": "LinkedField",
              "name": "node",
              "plural": false,
              "selections": [
                (v1/*: any*/),
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "filePath",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "action",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "toolName",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "recordedAt",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "isValidated",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "concreteType": "FileValidation",
                  "kind": "LinkedField",
                  "name": "validations",
                  "plural": true,
                  "selections": [
                    (v2/*: any*/),
                    (v3/*: any*/),
                    {
                      "alias": null,
                      "args": null,
                      "kind": "ScalarField",
                      "name": "validatedAt",
                      "storageKey": null
                    }
                  ],
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "concreteType": "FileValidation",
                  "kind": "LinkedField",
                  "name": "missingValidations",
                  "plural": true,
                  "selections": [
                    (v2/*: any*/),
                    (v3/*: any*/)
                  ],
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "__typename",
                  "storageKey": null
                }
              ],
              "storageKey": null
            },
            {
              "alias": null,
              "args": null,
              "kind": "ScalarField",
              "name": "cursor",
              "storageKey": null
            }
          ],
          "storageKey": null
        },
        {
          "alias": null,
          "args": null,
          "concreteType": "PageInfo",
          "kind": "LinkedField",
          "name": "pageInfo",
          "plural": false,
          "selections": [
            {
              "alias": null,
              "args": null,
              "kind": "ScalarField",
              "name": "hasNextPage",
              "storageKey": null
            },
            {
              "alias": null,
              "args": null,
              "kind": "ScalarField",
              "name": "endCursor",
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
      "kind": "ScalarField",
      "name": "fileChangeCount",
      "storageKey": null
    },
    (v1/*: any*/)
  ],
  "type": "Session",
  "abstractKey": null
};
})();

(node as any).hash = "c3edd05bd0cf237a75a21b7f41efdf19";

export default node;
