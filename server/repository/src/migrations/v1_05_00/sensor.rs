use crate::migrations::{DATETIME, DOUBLE};
use crate::{migrations::sql, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    #[cfg(not(feature = "postgres"))]
    const SENSOR_TYPE: &'static str = "TEXT";
    #[cfg(feature = "postgres")]
    const SENSOR_TYPE: &'static str = "sensor_type";
    #[cfg(feature = "postgres")]
    sql!(
        connection,
        r#"
            CREATE TYPE {TEST_ENUM_TYPE} AS ENUM (
                'BLUE_MAESTRO',
                'LAIRD'
            );
        "#
    )?;

    sql!(
        connection,
        r#"
            CREATE TABLE sensor (
                id TEXT NOT NULL PRIMARY KEY,
                serial TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                is_active BOOLEAN,
                store_id TEXT REFERENCES store(id),
                location_id TEXT REFERENCES location(id),
                battery_level INTEGER,
                log_interval INTEGER,
                last_connection_datetime {DATETIME},
                type {SENSOR_TYPE}
            );

            CREATE TABLE temperature_breach_config (
                id TEXT NOT NULL PRIMARY KEY,
                duration INTEGER NOT NULL,
                type TEXT NOT NULL,
                description TEXT NOT NULL UNIQUE,
                is_active BOOLEAN,
                store_id TEXT REFERENCES store(id),
                minimum_temperature {DOUBLE} NOT NULL,
                maximum_temperature {DOUBLE} NOT NULL
            );

            CREATE TABLE temperature_breach (
                id TEXT NOT NULL PRIMARY KEY,
                duration INTEGER NOT NULL,
                type TEXT NOT NULL,
                sensor_id TEXT NOT NULL REFERENCES sensor(id),
                store_id TEXT REFERENCES store(id),
                location_id TEXT REFERENCES location(id),
                start_datetime {DATETIME} NOT NULL,
                end_datetime {DATETIME},
                acknowledged BOOLEAN,
                threshold_minimum {DOUBLE} NOT NULL,
                threshold_maximum {DOUBLE} NOT NULL,
                threshold_duration INTEGER NOT NULL
            );

            CREATE TABLE temperature_log (
                id TEXT NOT NULL PRIMARY KEY,
                temperature {DOUBLE} NOT NULL,
                sensor_id TEXT NOT NULL REFERENCES sensor(id),
                store_id TEXT REFERENCES store(id),
                location_id TEXT REFERENCES location(id),
                datetime {DATETIME} NOT NULL,
                temperature_breach_id TEXT REFERENCES temperature_breach(id)
            );      
            "#
    )?;

    if cfg!(feature = "postgres") {
        sql!(
            connection,
            r#"
                ALTER TYPE changelog_table_name ADD VALUE IF NOT EXISTS 'sensor';
                ALTER TYPE changelog_table_name ADD VALUE IF NOT EXISTS 'temperature_breach';
                ALTER TYPE changelog_table_name ADD VALUE IF NOT EXISTS 'temperature_breach_config';
                ALTER TYPE changelog_table_name ADD VALUE IF NOT EXISTS 'temperature_log';


                CREATE TRIGGER sensor_trigger
                AFTER INSERT OR UPDATE OR DELETE ON sensor
                FOR EACH ROW EXECUTE PROCEDURE update_changelog();

                CREATE TRIGGER temperature_breach_trigger
                AFTER INSERT OR UPDATE OR DELETE ON temperature_breach
                FOR EACH ROW EXECUTE PROCEDURE update_changelog();

                CREATE TRIGGER temperature_breach_config_trigger
                AFTER INSERT OR UPDATE OR DELETE ON temperature_breach_config
                FOR EACH ROW EXECUTE PROCEDURE update_changelog();
                
                CREATE TRIGGER temperature_log_trigger
                AFTER INSERT OR UPDATE OR DELETE ON temperature_log
                FOR EACH ROW EXECUTE PROCEDURE update_changelog();
            "#
        )?;
    } else {
        sql!(
            connection,
            r#"
                CREATE TRIGGER sensor_insert_trigger
                AFTER INSERT ON sensor
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ("sensor", NEW.id, "UPSERT");
                END;

                CREATE TRIGGER temperature_breach_insert_trigger
                AFTER INSERT ON temperature_breach
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ("temperature_breach", NEW.id, "UPSERT");
                END;

                CREATE TRIGGER temperature_breach_config_insert_trigger
                AFTER INSERT ON temperature_breach_config
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ("temperature_breach_config", NEW.id, "UPSERT");
                END;

                CREATE TRIGGER temperature_log_insert_trigger
                AFTER INSERT ON temperature_log
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ("temperature_log", NEW.id, "UPSERT");
                END;
            "#
        )?;

        sql!(
            connection,
            r#"
                CREATE TRIGGER sensor_update_trigger
                AFTER UPDATE ON sensor
                BEGIN
                INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('sensor', NEW.id, 'UPSERT');
                END;

                CREATE TRIGGER temperature_breach_update_trigger
                AFTER UPDATE ON temperature_breach
                BEGIN
                INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('temperature_breach', NEW.id, 'UPSERT');
                END;

                CREATE TRIGGER temperature_breach_config_update_trigger
                AFTER UPDATE ON temperature_breach_config
                BEGIN
                INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('temperature_breach_config', NEW.id, 'UPSERT');
                END;

                CREATE TRIGGER temperature_log_update_trigger
                AFTER UPDATE ON temperature_log
                BEGIN
                INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('temperature_log', NEW.id, 'UPSERT');
                END;          
            "#
        )?;

        sql!(
            connection,
            r#"
                CREATE TRIGGER sensor_delete_trigger
                AFTER DELETE ON sensor
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('sensor', OLD.id, 'DELETE');
                END;

                CREATE TRIGGER temperature_breach_delete_trigger
                AFTER DELETE ON temperature_breach
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('temperature_breach', OLD.id, 'DELETE');
                END;

                CREATE TRIGGER temperature_breach_config_delete_trigger
                AFTER DELETE ON temperature_breach_config
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('temperature_breach_config', OLD.id, 'DELETE');
                END;

                CREATE TRIGGER temperature_log_delete_trigger
                AFTER DELETE ON temperature_log
                BEGIN
                    INSERT INTO changelog (table_name, record_id, row_action)
                    VALUES ('temperature_log', OLD.id, 'DELETE');
                END;
            "#
        )?;
    }
    Ok(())
}