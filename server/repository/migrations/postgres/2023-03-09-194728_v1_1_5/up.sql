ALTER TABLE name ADD is_sync_update BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE changelog ADD is_sync_update BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE clinician ADD is_sync_update BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE clinician_store_join ADD is_sync_update BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE name_store_join ADD is_sync_update BOOLEAN NOT NULL DEFAULT FALSE;

/* View wasn't being dropped in down.sql, so have moved it here to recreate view */
DROP VIEW IF EXISTS changelog_deduped;
CREATE VIEW changelog_deduped AS
    SELECT t1.cursor,
        t1.table_name,
        t1.record_id,
        t1.row_action,
        t1.name_id,
        t1.store_id,
        t1.is_sync_update
    FROM changelog t1
    WHERE t1.cursor = (SELECT max(t2.cursor) 
                    from changelog t2
                    where t2.record_id = t1.record_id)
    ORDER BY t1.cursor;