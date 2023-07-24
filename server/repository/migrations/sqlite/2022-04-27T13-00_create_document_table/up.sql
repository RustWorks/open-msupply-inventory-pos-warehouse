CREATE TABLE document (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    parent_ids TEXT NOT NULL,
    user_id TEXT NOT NULL,
    datetime TIMESTAMP NOT NULL,
    type TEXT NOT NULL,
    data TEXT NOT NULL,
    form_schema_id TEXT REFERENCES form_schema(id),
    status TEXT NOT NULL,
    owner_name_id TEXT REFERENCES name (id),
    context TEXT,
    is_sync_update BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX ix_document_name_unique ON document(name);

CREATE VIEW latest_document AS
SELECT d.*
FROM (
      SELECT name, MAX(datetime) AS datetime
      FROM document
      GROUP BY name
) grouped
INNER JOIN document d
ON d.name = grouped.name AND d.datetime = grouped.datetime;
