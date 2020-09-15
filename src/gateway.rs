use super::task_worker::{DoResource, DoSubTask, DoSubtaskVerification, TaskWorker};
use actix::prelude::*;
/** Module responsible for signle HUB session.

Traces given hub session.

**/
use futures::prelude::*;
use gu_actix::flatten::FlattenFuture;
use serde_derive::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

pub struct Gateway {
    gw_url: String,
    base_url: String,
    api: Option<std::rc::Rc<dyn golem_gw_api::apis::DefaultApi>>,
    hub_session: Option<gu_client::r#async::HubSession>,
    session_id: Option<u64>,
    last_event_id: i64,
    tasks: HashMap<String, Addr<TaskWorker>>,
    stats: StatsData,
}

pub struct Stats;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatsData {
    pub tasks: u64,
    pub subtasks: u64,
    pub subtasks_done: u64,
    pub fails: u64,
}

impl Message for Stats {
    type Result = Result<StatsData, super::error::Error>;
}

impl Gateway {
    pub fn new(session_id: Option<u64>, gw_url: String, base_url: String) -> Gateway {
        Gateway {
            gw_url,
            base_url,
            api: None,
            last_event_id: -1,
            session_id,
            tasks: HashMap::new(),
            hub_session: None,
            stats: StatsData::default(),
        }
    }

    fn api(&self) -> &golem_gw_api::apis::DefaultApi {
        self.api.as_ref().unwrap().as_ref()
    }

    fn set_status(&mut self, msg: &str, ctx: &mut <Self as Actor>::Context) {
        let hub_session = match &self.hub_session {
            Some(s) => s.clone(),
            None => return,
        };

        let config = hub_session.config();
        let status = match serde_json::to_value(msg) {
            Ok(s) => s,
            Err(e) => return,
        };
        ctx.spawn(
            config
                .and_then(move |mut c: gu_client::model::session::Metadata| {
                    c.entry.insert("status".to_owned(), status);
                    hub_session.set_config(c)
                })
                .map_err(|e| log::error!("update config {}", e))
                .and_then(|_| Ok(()))
                .into_actor(self),
        );
    }

    fn init_api(&mut self) -> &golem_gw_api::apis::DefaultApi {
        let http_client = hyper::client::Client::new();
        let mut api_configuration =
            golem_gw_api::apis::configuration::Configuration::new(http_client);
        log::info!("Brass Gateway url={}", self.base_url);
        api_configuration.base_path = self.base_url.clone();
        let api = Rc::new(golem_gw_api::apis::DefaultApiClient::new(Rc::new(
            api_configuration,
        )));

        self.api = Some(api);
        self.api.as_ref().unwrap().as_ref()
    }

    fn name(&self) -> &str {
        "gu-mediator blendering"
    }

    fn node_id(&self) -> &str {
        "0xb2bbb75241939e50b5ba6f698415bbb5ca54610d"
    }

    fn eth_public_key(&self) -> &str {
        "bf1abe57ba441ba1b3a6ee433cf1fd6028fec6061db84272a20beb2e760314162ad00451cd84584eaed4f1fc38b394e35c36d3e54925ac13e3a751fae3a66e0e"
    }

    fn task_type(&self) -> &str {
        "Blender"
    }

    fn new_subscription(&self) -> impl Future<Item = (), Error = failure::Error> {
        self.api()
            .subscribe(
                self.node_id(),
                self.task_type(),
                golem_gw_api::models::Subscription::new(
                    1f64,
                    6,
                    3 * 1024 * 1024 * 512,
                    3 * 1024 * 1024 * 512,
                )
                .with_name(self.name().into())
                .with_performance(1000f32)
                .with_eth_pub_key(self.eth_public_key().into()),
            )
            .and_then(|s| Ok(log::info!("status: {}", serde_json::to_string_pretty(&s)?)))
            .from_err()
    }

    fn poll_events(
        &self,
    ) -> impl Future<Item = Vec<golem_gw_api::models::Event>, Error = failure::Error> {
        self.api()
            .fetch_events(self.node_id(), self.task_type(), self.last_event_id)
            .from_err()
    }

    fn ack_event(&mut self, event_id: i64) {
        log::info!(
            "[ -[_]- ] event processed: {}/{}",
            event_id,
            self.last_event_id
        );
        if self.last_event_id < event_id {
            self.last_event_id = event_id;
        }
    }

    fn process_event(&mut self, ev: &golem_gw_api::models::Event) {
        if let Some(task) = ev.task() {
            let worker = TaskWorker::new(
                self.gw_url.clone(),
                self.api.as_ref().unwrap(),
                self.hub_session.clone().unwrap(),
                self.node_id(),
                task,
            )
            .start();
            self.stats.tasks += 1;
            self.tasks.insert(task.task_id().to_owned(), worker);
        } else if let Some(subtask) = ev.subtask() {
            if let Some(worker) = self.tasks.get(subtask.task_id()) {
                worker.do_send(DoSubTask(subtask.clone()))
            } else {
                log::warn!("no worker for: {}", subtask.task_id());
            }
        } else if let Some(resource) = ev.resource() {
            if let Some(worker) = self.tasks.get(resource.task_id()) {
                worker.do_send(DoResource(resource.clone()))
            } else {
                log::warn!("no worker for: {}", resource.task_id());
            }
        } else if let Some(subtask_verification) = ev.subtask_verification() {
            if let Some(worker) = self.tasks.get(subtask_verification.task_id()) {
                worker.do_send(DoSubtaskVerification(subtask_verification.clone()))
            } else {
                log::warn!("no worker for: {}", subtask_verification.task_id());
            }
        } else {
            log::warn!("invalid event={:?}", ev);
            return;
        }
        self.ack_event(ev.event_id());
    }

    fn pump_events(
        &self,
        ctx: &mut <Self as Actor>::Context,
    ) -> impl ActorFuture<Actor = Self, Item = (), Error = ()> {
        ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            let f = act
                .poll_events()
                .map_err(|e| log::error!("polling events failed: {}", e))
                .into_actor(act)
                .and_then(|events, act, _| {
                    for ev in events {
                        act.process_event(&ev)
                    }
                    fut::ok(())
                });
            ctx.spawn(f);
        });
        fut::ok(())
    }
}

impl Actor for Gateway {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let _api = self.init_api();

        let hub_connection = gu_client::r#async::HubConnection::default();

        if let Some(session_id) = self.session_id {
            self.hub_session = Some(hub_connection.hub_session(session_id));
        } else {
            let create_hub_session = hub_connection
                .new_session(gu_client::model::session::HubSessionSpec {
                    expires: None,
                    allocation: gu_client::model::session::AllocationMode::AUTO,
                    name: Some(self.name().into()),
                    tags: std::collections::BTreeSet::new(),
                })
                .into_actor(self)
                .map_err(|e, act, ctx| {
                    log::error!("failed to create hub session {:?}: {}", act.hub_session, e);
                    ctx.stop()
                })
                .and_then(|h, mut act, _| {
                    let hub_session: gu_client::r#async::HubSession = h.into_inner().unwrap();
                    act.session_id = Some(hub_session.id());
                    act.hub_session = Some(hub_session);
                    fut::ok(())
                });

            ctx.spawn(create_hub_session);
        }

        let f = self
            .new_subscription()
            .into_actor(self)
            .map_err(|e, act: &mut Gateway, ctx| {
                log::error!("Unable to update subscription: {}", e);
                act.set_status(&format!("error: {}", e), ctx);
                ctx.stop()
            })
            .and_then(|_, act, ctx| fut::ok(act.set_status("working", ctx)));
        ctx.spawn(f.and_then(|_, act, ctx| act.pump_events(ctx)));
    }
}

impl Handler<Stats> for Gateway {
    type Result = ActorResponse<Self, StatsData, super::error::Error>;

    fn handle(&mut self, msg: Stats, ctx: &mut Self::Context) -> Self::Result {
        use gu_actix::prelude::*;
        let tasks = self.tasks.len();

        let e: Vec<String> = self
            .tasks
            .iter()
            .filter_map(|(k, v)| {
                if !v.connected() {
                    Some(k.to_owned())
                } else {
                    None
                }
            })
            .collect();

        for k in e {
            self.tasks.remove(&k);
        }

        let init = self.stats.clone();

        ActorResponse::r#async(
            futures::future::join_all(
                self.tasks
                    .values()
                    .map(|t| {
                        t.send(Stats).flatten_fut().then(|r| match r {
                            Ok(r) => Ok(r),
                            Err(e) => {
                                log::warn!("get stats err: {}", e);
                                Ok(StatsData::default())
                            }
                        })
                    })
                    .collect::<Vec<_>>(),
            )
            .and_then(move |r: Vec<StatsData>| {
                let agg = r.into_iter().fold(init, |a, r| StatsData {
                    tasks: a.tasks + r.tasks,
                    subtasks_done: a.subtasks_done + r.subtasks_done,
                    subtasks: a.subtasks + r.subtasks,
                    fails: a.fails + r.fails,
                });

                Ok(agg)
            })
            .into_actor(self)
            .and_then(|r, mut act, _| {
                act.stats = r.clone();
                fut::ok(r)
            }),
        )
    }
}
