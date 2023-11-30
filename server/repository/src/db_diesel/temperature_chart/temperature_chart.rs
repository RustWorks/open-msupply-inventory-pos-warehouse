use crate::{
    db_diesel::temperature_log_row::temperature_log::dsl as temperature_log_dsl, DBType,
    RepositoryError, StorageConnection, TemperatureLogFilter, TemperatureLogRepository,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::temperature_chart_row::{Interval, *};

pub struct TemperatureChartRepository<'a> {
    connection: &'a StorageConnection,
}

type QueryResult = (NaiveDateTime, NaiveDateTime, f64, String, String);

impl<'a> TemperatureChartRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        TemperatureChartRepository { connection }
    }

    /// Result is sorted by sensor and then by datetime
    pub fn query(
        &self,
        intervals: Vec<Interval>,
        temperature_log_filter: Option<TemperatureLogFilter>,
    ) -> Result<Vec<TemperatureChartRow>, RepositoryError> {
        let mut query = TemperatureChart {
            intervals: intervals.clone(),
        }
        .into_boxed::<DBType>();

        if temperature_log_filter.is_some() {
            let temperature_log_ids =
                TemperatureLogRepository::create_filtered_query(temperature_log_filter)
                    .select(temperature_log_dsl::id);
            query = query.filter(TemperatureLogId.eq_any(temperature_log_ids));
        };

        let query = query
            .select((
                FromDatetime,
                ToDatetime,
                AverageTemperature,
                SensorId,
                BreachIds,
            ))
            .group_by((FromDatetime, ToDatetime, SensorId));

        // First by sensor then by datetime (so should be sorted by sensor and then by datetime)
        let query = query.order_by((SensorId.asc(), FromDatetime.asc()));

        // Debug diesel query
        // println!("{}", diesel::debug_query::<DBType, _>(&query).to_string());

        let chart_data = query
            .load::<QueryResult>(&self.connection.connection)?
            .into_iter()
            .map(TemperatureChartRow::from)
            .collect::<Result<_, _>>()?;

        Ok(chart_data)
    }
}

impl TemperatureChartRow {
    fn from(
        (from_datetime, to_datetime, average_temperature, sensor_id, breach_ids): QueryResult,
    ) -> Result<Self, RepositoryError> {
        // breach_ids at this stage is a stringified JSON array of string or null values
        // i.e. '[null, "first_breach"]
        let breach_ids_with_nulls: Vec<Option<String>> = serde_json::from_str(&breach_ids)
            .map_err(|e| RepositoryError::DBError {
                msg: e.to_string(),
                extra: breach_ids,
            })?;

        Ok(Self {
            from_datetime,
            to_datetime,
            average_temperature,
            sensor_id,
            breach_ids: breach_ids_with_nulls
                .into_iter()
                .filter_map(|id_or_none| id_or_none)
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        location::LocationFilter,
        mock::{MockData, MockDataInserts},
        test_db::setup_all_with_data,
        EqualFilter, LocationRow, NameRow, SensorFilter, SensorRow, StoreRow, TemperatureBreachRow,
        TemperatureChartRepository, TemperatureChartRow, TemperatureLogRow,
    };

    use rand::{seq::SliceRandom, thread_rng};
    use util::create_datetime;

    #[actix_rt::test]
    async fn temperature_charts() {
        let name = NameRow {
            id: "name1".to_string(),
            ..Default::default()
        };

        let store = StoreRow {
            id: "store".to_string(),
            name_id: name.id.clone(),
            ..Default::default()
        };

        let location = LocationRow {
            id: "location".to_string(),
            store_id: store.id.clone(),
            ..Default::default()
        };

        let sensor1 = SensorRow {
            id: "sensor1".to_string(),
            serial: "sensor1".to_string(),
            store_id: store.id.clone(),
            ..Default::default()
        };

        let sensor2 = SensorRow {
            id: "sensor2".to_string(),
            serial: "sensor2".to_string(),
            store_id: store.id.clone(),
            ..Default::default()
        };

        let breach1 = TemperatureBreachRow {
            id: "breach1".to_string(),
            sensor_id: sensor1.id.clone(),
            store_id: store.id.clone(),
            ..Default::default()
        };

        let breach2 = TemperatureBreachRow {
            id: "breach2".to_string(),
            sensor_id: sensor1.id.clone(),
            store_id: store.id.clone(),
            ..Default::default()
        };

        // Test intervals will be
        let intervals = vec![
            Interval {
                // P1
                from_datetime: create_datetime(2021, 01, 01, 23, 59, 50).unwrap(),
                to_datetime: create_datetime(2021, 01, 01, 23, 59, 56).unwrap(),
            },
            Interval {
                // P2
                from_datetime: create_datetime(2021, 01, 01, 23, 59, 56).unwrap(),
                to_datetime: create_datetime(2021, 01, 02, 00, 00, 02).unwrap(),
            },
            Interval {
                // P3
                from_datetime: create_datetime(2021, 01, 02, 00, 00, 02).unwrap(),
                to_datetime: create_datetime(2021, 01, 02, 00, 00, 08).unwrap(),
            },
        ];

        // Want to test two sensors, with gap in data, and one location filter

        let s1 = &sensor1.id;
        let s2 = &sensor2.id;
        let l1 = Some(&location.id);
        let b1 = Some(&breach1.id);
        let b2 = Some(&breach2.id);

        // Sensor 1 (S1)
        let mut temperature_logs: Vec<TemperatureLogRow> = vec![
            ((2021, 01, 01), (23, 59, 49), 100.0, s1, None, None), // Not in period
            ((2021, 01, 01), (23, 59, 50), 10.0, s1, None, b1),    // (P1-S1 no location, breach1)
            ((2021, 01, 01), (23, 59, 55), 5.0, s1, None, b2),     // (P1-S1 no location, breach2)
            ((2021, 01, 01), (23, 59, 56), 1.0, s1, l1, None),     // (P2-S1-L1)
            ((2021, 01, 02), (00, 00, 03), 0.0, s1, None, None),   // (P3-S1-L1)
            ((2021, 01, 02), (00, 00, 07), 5.0, s1, None, None),   // (P3-S1 no location)
            ((2021, 01, 02), (00, 00, 08), 100.0, s1, None, None), // Not in range
            ((2021, 01, 01), (23, 59, 49), 100.0, s2, None, None), // Not in period
            ((2021, 01, 01), (23, 59, 50), -10.0, s2, l1, None),   // (P1-S2-L1)
            ((2021, 01, 01), (23, 59, 55), -5.0, s2, l1, None),    // (P1-S2-L1)
            // (P2-S2) - No data
            ((2021, 01, 02), (00, 00, 03), 3.0, s2, None, None), // (P3-S2 no location)
            ((2021, 01, 02), (00, 00, 08), 100.0, s2, None, None), // Not in range
        ]
        .into_iter()
        .map(
            |(date, time, temperature, sensor_id, location, breach)| TemperatureLogRow {
                id: util::uuid::uuid(),
                temperature,
                sensor_id: sensor_id.clone(),
                store_id: store.id.clone(),
                datetime: create_datetime(date.0, date.1, date.2, time.0, time.1, time.2).unwrap(),
                location_id: location.map(ToString::to_string),
                temperature_breach_id: breach.map(ToString::to_string),
                ..Default::default()
            },
        )
        .collect();

        // This repository should return rows orderd by sensor and then by datetime
        // it's important to shuffle before inserting to test this
        temperature_logs.shuffle(&mut thread_rng());

        let (_, connection, _, _) = setup_all_with_data(
            "temperature_charts",
            MockDataInserts::none(),
            MockData {
                stores: vec![store],
                names: vec![name],
                sensors: vec![sensor1.clone(), sensor2.clone()],
                temperature_logs: temperature_logs,
                locations: vec![location.clone()],
                temperature_breaches: vec![breach1.clone(), breach2.clone()],
                ..Default::default()
            },
        )
        .await;

        let repo = TemperatureChartRepository::new(&connection);

        // Just date filter
        let mut result = repo.query(intervals.clone(), None).unwrap();
        // BreachIds are unordered in repository result, need to sort them to compare
        result[0].breach_ids.sort();
        assert_eq!(
            result,
            vec![
                TemperatureChartRow {
                    from_datetime: intervals[0].from_datetime,
                    to_datetime: intervals[0].to_datetime,
                    average_temperature: 7.5,
                    sensor_id: sensor1.id.clone(),
                    breach_ids: vec![breach1.id.clone(), breach2.id.clone()]
                },
                TemperatureChartRow {
                    from_datetime: intervals[1].from_datetime,
                    to_datetime: intervals[1].to_datetime,
                    average_temperature: 1.0,
                    sensor_id: sensor1.id.clone(),
                    breach_ids: Vec::new()
                },
                TemperatureChartRow {
                    from_datetime: intervals[2].from_datetime,
                    to_datetime: intervals[2].to_datetime,
                    average_temperature: 2.5,
                    sensor_id: sensor1.id.clone(),
                    breach_ids: Vec::new()
                },
                TemperatureChartRow {
                    from_datetime: intervals[0].from_datetime,
                    to_datetime: intervals[0].to_datetime,
                    average_temperature: -7.5,
                    sensor_id: sensor2.id.clone(),
                    breach_ids: Vec::new()
                },
                // Data point missing
                TemperatureChartRow {
                    from_datetime: intervals[2].from_datetime,
                    to_datetime: intervals[2].to_datetime,
                    average_temperature: 3.0,
                    sensor_id: sensor2.id.clone(),
                    breach_ids: Vec::new()
                }
            ],
        );

        // Filter by sensor 2
        let result = repo
            .query(
                intervals.clone(),
                Some(
                    TemperatureLogFilter::new()
                        .sensor(SensorFilter::new().id(EqualFilter::equal_to(&sensor2.id))),
                ),
            )
            .unwrap();

        assert_eq!(
            result,
            vec![
                TemperatureChartRow {
                    from_datetime: intervals[0].from_datetime,
                    to_datetime: intervals[0].to_datetime,
                    average_temperature: -7.5,
                    sensor_id: sensor2.id.clone(),
                    breach_ids: Vec::new()
                },
                // Data point missing
                TemperatureChartRow {
                    from_datetime: intervals[2].from_datetime,
                    to_datetime: intervals[2].to_datetime,
                    average_temperature: 3.0,
                    sensor_id: sensor2.id.clone(),
                    breach_ids: Vec::new()
                }
            ]
        );

        // Filter by location
        let result = repo
            .query(
                intervals.clone(),
                Some(
                    TemperatureLogFilter::new()
                        .location(LocationFilter::new().id(EqualFilter::equal_to(&location.id))),
                ),
            )
            .unwrap();

        assert_eq!(
            result,
            vec![
                TemperatureChartRow {
                    from_datetime: intervals[1].from_datetime,
                    to_datetime: intervals[1].to_datetime,
                    average_temperature: 1.0,
                    sensor_id: sensor1.id.clone(),
                    breach_ids: Vec::new()
                },
                TemperatureChartRow {
                    from_datetime: intervals[0].from_datetime,
                    to_datetime: intervals[0].to_datetime,
                    average_temperature: -7.5,
                    sensor_id: sensor2.id.clone(),
                    breach_ids: Vec::new()
                }
            ] // Missing data for location
        )
    }
}
