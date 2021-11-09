import { Dispatch } from 'react';

export interface Api<ServerData, Document> {
  onRead: (id: string) => Promise<ServerData>;
  onUpdate: (val: Document) => Promise<ServerData>;
}

export type ReducerCreator<ServerData, State, ActionSet> = (
  data: ServerData | undefined,
  dispatch: Dispatch<DocumentActionSet<ActionSet>> | null
) => (state: State | undefined, action: ActionSet) => State;

export interface DocumentState<Document, State, ActionSet> {
  draft: Document;
  state: State;
  dispatch: Dispatch<DocumentActionSet<ActionSet>>;
  save: () => Promise<void>;
}

export enum DocumentActionType {
  Init = 'Document/init',
  Merge = 'Document/merge',
}

export type DefaultDocumentAction =
  | { type: DocumentActionType.Init }
  | { type: DocumentActionType.Merge };

export type DocumentActionSet<T> = T | DefaultDocumentAction;
