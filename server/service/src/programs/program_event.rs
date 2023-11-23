use std::collections::HashMap;

use chrono::{Duration, NaiveDate, NaiveDateTime};
use repository::{
    DatetimeFilter, EqualFilter, Pagination, PaginationOption, ProgramEventFilter,
    ProgramEventRepository, ProgramEventRow, ProgramEventRowRepository, ProgramEventSort,
    ProgramEventSortField, RepositoryError, StorageConnection,
};
use util::uuid::uuid;

use crate::{
    get_default_pagination, i64_to_u32, service_provider::ServiceContext, ListError, ListResult,
};

pub struct EventInput {
    pub active_start_datetime: NaiveDateTime,
    pub document_type: String,
    pub document_name: Option<String>,
    pub r#type: String,
    pub name: Option<String>,
}

pub const MAX_LIMIT: u32 = 2000;
pub const MIN_LIMIT: u32 = 1;

/// Events are grouped and updated by the fields in this struct
#[derive(Hash, PartialEq, Eq)]
struct EventTarget {
    pub patient_id: String,
    pub document_type: String,
    pub document_name: Option<String>,
    pub r#type: String,
}

struct StackEvent {
    pub active_start_datetime: NaiveDateTime,
    pub name: Option<String>,
}

/// NaiveDateTime::MAX doesn't serializes well in sqlite (+262143-12-31 23:59:59.999999999 is
/// smaller than any other datetime)
fn max_datetime() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(9999, 9, 9)
        .unwrap()
        .and_hms_opt(9, 9, 9)
        .unwrap()
}

fn event_target_filter(target: &EventTarget) -> ProgramEventFilter {
    let mut filter = ProgramEventFilter::new()
        .patient_id(EqualFilter::equal_to(&target.patient_id))
        .document_type(EqualFilter::equal_to(&target.document_type))
        .r#type(EqualFilter::equal_to(&target.r#type));
    if let Some(document_name) = &target.document_name {
        filter = filter.document_name(EqualFilter::equal_to(&document_name));
    }
    filter
}

fn remove_event_stack(
    datetime: NaiveDateTime,
    event_target: &EventTarget,
    connection: &StorageConnection,
) -> Result<(), RepositoryError> {
    let repo = ProgramEventRepository::new(connection);
    // get the longest active_end_datetime from the stack that is being removed
    let stack_events = repo.query(
        Pagination::one(),
        Some(event_target_filter(event_target).datetime(DatetimeFilter::equal_to(datetime))),
        Some(ProgramEventSort {
            key: ProgramEventSortField::ActiveStartDatetime,
            desc: Some(true),
        }),
    )?;
    let Some(longest) = stack_events.get(0) else {
        // no stack found -> done
        return Ok(());
    };
    // delete the stack
    repo.delete(event_target_filter(event_target).datetime(DatetimeFilter::equal_to(datetime)))?;

    // TODO: Below is some room for optimization. We might update the same events later...
    // For simplicity we cleanly remove the whole stack though.

    // update active_end_datetime of the latest event from the previous stack
    let previous_stack = repo.query(
        Pagination::all(),
        Some(
            event_target_filter(event_target)
                .active_end_datetime(DatetimeFilter::equal_to(datetime)),
        ),
        Some(ProgramEventSort {
            key: ProgramEventSortField::ActiveStartDatetime,
            desc: Some(true),
        }),
    )?;

    let mut current_end_datetime = longest.active_end_datetime;
    for mut current in previous_stack {
        current.active_end_datetime = current_end_datetime;
        current_end_datetime = current.active_start_datetime;
        ProgramEventRowRepository::new(connection).upsert_one(&current)?;
    }

    Ok(())
}

pub trait ProgramEventServiceTrait: Sync + Send {
    fn events(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<ProgramEventFilter>,
        sort: Option<ProgramEventSort>,
    ) -> Result<ListResult<ProgramEventRow>, ListError> {
        let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
        let repository = ProgramEventRepository::new(&ctx.connection);
        Ok(ListResult {
            rows: repository.query(pagination, filter.clone(), sort)?,
            count: i64_to_u32(repository.count(filter)?),
        })
    }

    fn active_events(
        &self,
        ctx: &ServiceContext,
        at: NaiveDateTime,
        pagination: Option<PaginationOption>,
        filter: Option<ProgramEventFilter>,
        sort: Option<ProgramEventSort>,
    ) -> Result<ListResult<ProgramEventRow>, ListError> {
        let filter = filter
            .unwrap_or(ProgramEventFilter::new())
            .active_start_datetime(DatetimeFilter::before_or_equal_to(at))
            .active_end_datetime(DatetimeFilter::after_or_equal_to(
                // TODO: add an `after` filter
                at.checked_add_signed(Duration::nanoseconds(1))
                    .unwrap_or(at),
            ));
        self.events(ctx, pagination, Some(filter), sort)
    }

    /// Upserts all events of a patient with a given datetime, i.e. it removes exiting events and
    /// inserts the provided list.
    /// For example, all events of a single document with the timestamp `datetime` can be updated
    /// using the method.
    ///
    /// Note, this assumes that no two documents for a single patient are changed at exactly the
    /// same time.
    fn upsert_events(
        &self,
        connection: &StorageConnection,
        patient_id: String,
        datetime: NaiveDateTime,
        context_id: &str,
        events: Vec<EventInput>,
    ) -> Result<(), RepositoryError> {
        let result = connection
            .transaction_sync(|con| -> Result<(), RepositoryError> {
                // TODO do we need to lock rows in case events are updated concurrently?

                let repo = ProgramEventRepository::new(con);
                let targets = if events.is_empty() {
                    // We need to clear all events. For this we still need to find all targets.
                    // To do this load all existing events and proceed.

                    repo.query_by_filter(
                        ProgramEventFilter::new()
                            .patient_id(EqualFilter::equal_to(&patient_id))
                            .datetime(DatetimeFilter::equal_to(datetime)),
                    )?
                    .into_iter()
                    .fold(
                        HashMap::<EventTarget, Vec<StackEvent>>::new(),
                        |mut map, it| {
                            let target = EventTarget {
                                patient_id: patient_id.clone(),
                                document_type: it.document_type,
                                document_name: it.document_name,
                                r#type: it.r#type,
                            };

                            map.entry(target).or_insert(vec![]);
                            map
                        },
                    )
                } else {
                    events.into_iter().fold(
                        HashMap::<EventTarget, Vec<StackEvent>>::new(),
                        |mut map, it| {
                            let target = EventTarget {
                                patient_id: patient_id.clone(),
                                document_type: it.document_type,
                                document_name: it.document_name,
                                r#type: it.r#type,
                            };

                            map.entry(target).or_insert(vec![]).push(StackEvent {
                                // sanitise active_start_datetime to not be small than datetime
                                active_start_datetime: datetime.max(it.active_start_datetime),
                                name: it.name,
                            });
                            map
                        },
                    )
                };

                for (target, mut events) in targets {
                    // remove existing stack, if there is any
                    remove_event_stack(datetime, &target, con)?;
                    if events.is_empty() {
                        continue;
                    }
                    // find events that need to be adjusted
                    let overlaps = repo.query(
                        Pagination::all(),
                        Some(
                            event_target_filter(&target)
                                .datetime(DatetimeFilter::before_or_equal_to(datetime))
                                .active_end_datetime(DatetimeFilter::after_or_equal_to(datetime)),
                        ),
                        Some(ProgramEventSort {
                            key: ProgramEventSortField::ActiveEndDatetime,
                            desc: Some(true),
                        }),
                    )?;

                    let active_end_datetime = if let Some(active_end_datetime) =
                        overlaps.get(0).map(|it| it.active_end_datetime)
                    {
                        active_end_datetime
                    } else {
                        // We inserting either before the first event or we inserting the very first
                        // event.
                        // First test if there is a next event:
                        let next = repo
                            .query(
                                Pagination::one(),
                                Some(
                                    event_target_filter(&target)
                                        .datetime(DatetimeFilter::after_or_equal_to(datetime)),
                                ),
                                Some(ProgramEventSort {
                                    key: ProgramEventSortField::Datetime,
                                    desc: Some(false),
                                }),
                            )?
                            .pop()
                            .map(|e| e.datetime);
                        // If there is no next event we are inserting the very first event, thus
                        // use max_datetime()
                        next.unwrap_or(max_datetime())
                    };

                    for mut overlap in overlaps {
                        overlap.active_end_datetime = datetime;
                        ProgramEventRowRepository::new(con).upsert_one(&overlap)?;
                    }

                    events.sort_by(|a, b| b.active_start_datetime.cmp(&a.active_start_datetime));
                    let mut events = events
                        .into_iter()
                        .map(|it| ProgramEventRow {
                            id: uuid(),
                            datetime,
                            active_start_datetime: it.active_start_datetime,
                            active_end_datetime,
                            patient_id: Some(patient_id.clone()),
                            document_type: target.document_type.clone(),
                            document_name: target.document_name.clone(),
                            context_id: context_id.to_string(),
                            r#type: target.r#type.clone(),
                            data: it.name,
                        })
                        .collect::<Vec<_>>();
                    // adjust end times within the stack
                    for n in 0..events.len() - 1 {
                        let active_start_datetime = events[n].active_start_datetime;
                        let active_end_datetime = events[n].active_end_datetime;
                        // End time of the whole stack might be already before the start time of an
                        // individual event.
                        // Thus take the min(start, end):
                        events[n + 1].active_end_datetime =
                            std::cmp::min(active_start_datetime, active_end_datetime);
                    }
                    for event in events {
                        ProgramEventRowRepository::new(con).upsert_one(&event)?;
                    }
                }

                Ok(())
            })
            .map_err(|err| err.to_inner_error())?;
        Ok(result)
    }
}

pub struct ProgramEventService {}
impl ProgramEventServiceTrait for ProgramEventService {}

#[cfg(test)]
mod test {
    use chrono::NaiveTime;
    use repository::{
        mock::{mock_program_a, MockDataInserts},
        test_db::setup_all,
    };

    use crate::service_provider::ServiceProvider;

    use super::*;

    // asserts an unfiltered active_events() with time `at` contains rows with the expected names
    macro_rules! assert_names {
        ($service:expr, $ctx:expr, $at:expr, $expected:expr) => {{
            let events = $service
                .active_events(
                    &$ctx,
                    NaiveDateTime::from_timestamp_opt($at, 0).unwrap(),
                    None,
                    Some(ProgramEventFilter::new()),
                    None,
                )
                .unwrap();
            let mut expected: Vec<&str> = $expected;
            expected.sort();
            let expected = expected
                .into_iter()
                .map(|it| it.to_string())
                .collect::<Vec<_>>();
            let mut actual_names = events
                .rows
                .iter()
                .map(|it| it.data.clone().unwrap())
                .collect::<Vec<_>>();
            actual_names.sort();
            assert_eq!(expected, actual_names);
        }};
    }

    #[actix_rt::test]
    async fn test_basic_program_events() {
        let (_, _, connection_manager, _) = setup_all(
            "test_basic_program_events",
            MockDataInserts::none().names().contexts(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        let service = service_provider.program_event_service;

        // try to insert a single event
        // ----------x
        //          10
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                    document_type: "DocType".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("data1".to_string()),
                }],
            )
            .unwrap();
        assert_names!(service, ctx, 5, vec![]);
        assert_names!(service, ctx, 15, vec!["data1"]);

        // insert later event with different active_start_datetime
        // ----------x----------x
        //          10         20-------->30
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(20, 0).unwrap(),
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: NaiveDateTime::from_timestamp_opt(30, 0).unwrap(),
                    document_type: "DocType".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("data2".to_string()),
                }],
            )
            .unwrap();
        assert_names!(service, ctx, 19, vec!["data1"]);
        assert_names!(service, ctx, 25, vec![]);
        assert_names!(service, ctx, 30, vec!["data2"]);

        // terminate the second event
        // ----------x----------x-----x
        //          10         20--------->30
        //                           25-------->35
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(25, 0).unwrap(),
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: NaiveDateTime::from_timestamp_opt(35, 0).unwrap(),
                    document_type: "DocType".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("data3".to_string()),
                }],
            )
            .unwrap();
        assert_names!(service, ctx, 31, vec![]);
        assert_names!(service, ctx, 40, vec!["data3"]);

        // add an event for different type (should show up in results together with existing)
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(40, 0).unwrap(),
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: NaiveDateTime::from_timestamp_opt(40, 0).unwrap(),
                    document_type: "DocType2".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("data4".to_string()),
                }],
            )
            .unwrap();
        assert_names!(service, ctx, 50, vec!["data3", "data4"]);
    }

    #[actix_rt::test]
    async fn test_program_reverse_order_events() {
        let (_, _, connection_manager, _) = setup_all(
            "test_program_reverse_order_events",
            MockDataInserts::none().names().contexts(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        let service = service_provider.program_event_service;

        // Test special case where the inserted event is the very first event

        // try to insert a single event
        // ----------x----------
        //          10
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                    document_type: "DocType".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("data1".to_string()),
                }],
            )
            .unwrap();
        assert_names!(service, ctx, 5, vec![]);
        assert_names!(service, ctx, 15, vec!["data1"]);

        // insert earlier event
        // ----x------x--
        //     5     10
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(5, 0).unwrap(),
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: NaiveDateTime::from_timestamp_opt(5, 0).unwrap(),
                    document_type: "DocType".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("data2".to_string()),
                }],
            )
            .unwrap();
        assert_names!(service, ctx, 6, vec!["data2"]);
        assert_names!(service, ctx, 15, vec!["data1"]);
    }

    #[actix_rt::test]
    async fn test_event_stacks() {
        let (_, _, connection_manager, _) = setup_all(
            "test_event_stacks",
            MockDataInserts::none().names().contexts(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        let service = service_provider.program_event_service;

        // add stack s1 and test that status changes over time
        // ----------s1
        //           10->10
        //           10------->20
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(20, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_2".to_string()),
                    },
                ],
            )
            .unwrap();
        assert_names!(service, ctx, 5, vec![]);
        assert_names!(service, ctx, 10, vec!["G1_1"]);
        assert_names!(service, ctx, 25, vec!["G1_2"]);

        // replace stack s1 and test that status changes over time
        // ----------s1
        //           10---->15
        //           10--------------->30
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(15, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_3".to_string()),
                    },
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(30, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_4".to_string()),
                    },
                ],
            )
            .unwrap();
        assert_names!(service, ctx, 5, vec![]);
        assert_names!(service, ctx, 15, vec!["G1_3"]);
        assert_names!(service, ctx, 30, vec!["G1_4"]);
        assert_names!(service, ctx, 35, vec!["G1_4"]);

        // test "cutting" of an event from one stack when inserting a later stack
        // ----------g1-----------g2
        //           10---->15
        //           10--------------->30
        //                        25------->35
        //                        25------------->40
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(25, 0).unwrap(),
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(35, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G2_1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(40, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G2_2".to_string()),
                    },
                ],
            )
            .unwrap();
        assert_names!(service, ctx, 26, vec![]);
        assert_names!(service, ctx, 31, vec![]);
        assert_names!(service, ctx, 35, vec!["G2_1"]);
        assert_names!(service, ctx, 40, vec!["G2_2"]);
    }

    #[actix_rt::test]
    async fn test_program_events_remove_stacks() {
        let (_, _, connection_manager, _) = setup_all(
            "test_program_events_remove_stacks",
            MockDataInserts::none().names().contexts(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        let service = service_provider.program_event_service;

        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(5, 0).unwrap(),
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(5, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("data1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(5, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status2".to_string(),
                        name: Some("data2".to_string()),
                    },
                ],
            )
            .unwrap();
        assert_names!(service, ctx, 10, vec!["data1", "data2"]);

        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(5, 0).unwrap(),
                &mock_program_a().context_id,
                vec![],
            )
            .unwrap();
        assert_names!(service, ctx, 10, vec![]);
    }

    #[actix_rt::test]
    async fn test_program_events_remove_stacks2() {
        let (_, _, connection_manager, _) = setup_all(
            "test_program_events_remove_stacks2",
            MockDataInserts::none().names().contexts(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        let service = service_provider.program_event_service;

        // setup: g2 is overwriting g1
        // ----------g1-----------g2
        //           10---->15
        //           10--------------->30
        //                        25------->35
        //                        25------------->40
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(10, 0).unwrap(),
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(15, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(30, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_2".to_string()),
                    },
                ],
            )
            .unwrap();
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(25, 0).unwrap(),
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(35, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G2_1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: NaiveDateTime::from_timestamp_opt(40, 0).unwrap(),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G2_2".to_string()),
                    },
                ],
            )
            .unwrap();
        assert_names!(service, ctx, 26, vec![]);
        assert_names!(service, ctx, 31, vec![]);
        assert_names!(service, ctx, 35, vec!["G2_1"]);
        assert_names!(service, ctx, 40, vec!["G2_2"]);

        // remove G2 -> G2 should become active again
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                NaiveDateTime::from_timestamp_opt(25, 0).unwrap(),
                &mock_program_a().context_id,
                vec![],
            )
            .unwrap();
        assert_names!(service, ctx, 26, vec!["G1_1"]);
        assert_names!(service, ctx, 31, vec!["G1_2"]);
    }

    #[actix_rt::test]
    async fn test_program_events_bug() {
        let (_, _, connection_manager, _) = setup_all(
            "test_program_events_bug",
            MockDataInserts::none().names().contexts(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        let service = service_provider.program_event_service;

        let datetime_from_date = |year: i32, month: u32, day: u32| {
            NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        };

        // An earlier stack is inserted after a later stack.
        // When inserting the earlier stack all events of the earlier stack should finish before
        // the datetime of the later stack.

        // Final target setup with g2 inserted first:
        // ---g1-------------g2-----------
        //                  18/09---->16/12
        //                  18/09--------------->13/1
        //                  18/09--------------------------->15/3
        //   17/6-->14/09--|
        //   17/6---------->16/09          // end datetime = 18/09 (previously set to 13/12)
        //   17/6------------|----->13/12  // end datetime = 18/09

        let later_stack_datetime = datetime_from_date(2023, 09, 18);
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                later_stack_datetime,
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: datetime_from_date(2023, 12, 16),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G2_1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: datetime_from_date(2024, 01, 13),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G2_2".to_string()),
                    },
                    EventInput {
                        active_start_datetime: datetime_from_date(2024, 03, 15),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G2_3".to_string()),
                    },
                ],
            )
            .unwrap();

        let earlier_stack_datetime = datetime_from_date(2023, 06, 17);
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                earlier_stack_datetime,
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: datetime_from_date(2023, 09, 14),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: datetime_from_date(2023, 09, 16),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_2".to_string()),
                    },
                    EventInput {
                        active_start_datetime: datetime_from_date(2023, 12, 13),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_3".to_string()),
                    },
                ],
            )
            .unwrap();

        let events = service
            .active_events(
                &ctx,
                datetime_from_date(2023, 10, 5),
                None,
                Some(ProgramEventFilter::new()),
                None,
            )
            .unwrap();
        assert_eq!(events.count, 0);

        // check end times for the earlier group
        let event_end_datetimes = service
            .events(
                &ctx,
                None,
                Some(
                    ProgramEventFilter::new()
                        .datetime(DatetimeFilter::equal_to(earlier_stack_datetime)),
                ),
                Some(ProgramEventSort {
                    key: ProgramEventSortField::ActiveStartDatetime,
                    desc: Some(false),
                }),
            )
            .unwrap()
            .rows
            .into_iter()
            .map(|e| e.active_end_datetime)
            .collect::<Vec<_>>();
        assert_eq!(
            event_end_datetimes,
            vec![
                datetime_from_date(2023, 09, 16),
                later_stack_datetime,
                later_stack_datetime
            ]
        )
    }

    fn check_integrity(mut events: Vec<ProgramEventRow>) {
        events.sort_by(|a, b| {
            a.datetime
                .cmp(&b.datetime)
                .then_with(|| a.active_start_datetime.cmp(&b.active_start_datetime))
        });
        if events.is_empty() {
            return;
        }

        // init with first datetime
        let mut prev_event_end = events[0].datetime;
        while !events.is_empty() {
            let cur_stack_time = events[0].datetime;
            // remove first stack, i.e., events with the same datetime
            let mut stack = vec![];
            while !events.is_empty() && events[0].datetime == cur_stack_time {
                let e = events.remove(0);
                stack.push(e);
            }

            // validate stack integrity
            let stack_end = stack.last().unwrap().active_end_datetime;
            let mut stack_end_time_reached = false;
            for (i, event) in stack.iter().enumerate() {
                assert!(event.datetime <= event.active_start_datetime);
                assert!(event.datetime <= event.active_end_datetime);

                if !stack_end_time_reached && event.active_end_datetime == stack_end {
                    stack_end_time_reached = true;
                }
                if i == 0 {
                    assert_eq!(
                        event.datetime, prev_event_end,
                        "Event {:?} must start where previous event ended",
                        event.data
                    );
                } else if !stack_end_time_reached {
                    assert_eq!(
                        event.active_start_datetime, prev_event_end,
                        "Event {:?} must start where previous event ended",
                        event.data
                    );
                } else {
                    assert_eq!(
                        event.active_end_datetime, stack_end,
                        "End of stack changed in {:?}",
                        event.data
                    );
                }
                assert!(event.active_end_datetime >= prev_event_end);
                prev_event_end = event.active_end_datetime;
            }
        }
    }

    #[actix_rt::test]
    async fn test_program_events_bug_2() {
        let (_, _, connection_manager, _) = setup_all(
            "test_program_events_bug_2",
            MockDataInserts::none().names().contexts(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        let service = service_provider.program_event_service;

        let datetime_from_date = |year: i32, month: u32, day: u32| {
            NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        };

        let stack_3_datetime = datetime_from_date(2012, 05, 01);
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                stack_3_datetime,
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: stack_3_datetime,
                    document_type: "DocType".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("G3_1".to_string()),
                }],
            )
            .unwrap();

        let stack_2_datetime = datetime_from_date(2012, 04, 03);
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                stack_2_datetime,
                &mock_program_a().context_id,
                vec![EventInput {
                    active_start_datetime: datetime_from_date(2012, 05, 10),
                    document_type: "DocType".to_string(),
                    document_name: None,
                    r#type: "status".to_string(),
                    name: Some("G2_1".to_string()),
                }],
            )
            .unwrap();

        let stack_1_datetime = datetime_from_date(2011, 11, 29);
        service
            .upsert_events(
                &ctx.connection,
                "patient2".to_string(),
                stack_1_datetime,
                &mock_program_a().context_id,
                vec![
                    EventInput {
                        active_start_datetime: datetime_from_date(2011, 12, 31),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_1".to_string()),
                    },
                    EventInput {
                        active_start_datetime: datetime_from_date(2012, 01, 28),
                        document_type: "DocType".to_string(),
                        document_name: None,
                        r#type: "status".to_string(),
                        name: Some("G1_2".to_string()),
                    },
                ],
            )
            .unwrap();

        // Expect the following events:
        // G3_1: datetime: 2012-05-01T00:00:00, start: 2012-05-01T00:00:00, end: 9999-09-09T09:09:09
        // G2_1: datetime: 2012-04-03T00:00:00, start: 2012-05-10T00:00:00, end: 2012-05-01T00:00:00
        // G1_2: datetime: 2011-11-29T00:00:00, start: 2012-01-28T00:00:00, end: 2012-04-03T00:00:00
        // G1_1: datetime: 2011-11-29T00:00:00, start: 2011-12-31T00:00:00, end: 2012-01-28T00:00:00
        let result = service.events(&ctx, None, None, None).unwrap();
        check_integrity(result.rows);
    }
}
