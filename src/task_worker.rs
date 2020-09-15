use super::blender;
use super::{dav, joinact, workman};
use actix::prelude::*;
use futures::prelude::*;
use golem_gw_api::models::Subtask;
use gu_client::NodeId;
use std::rc::Rc;

pub struct TaskWorker {
    gw_url: String,
    api: Rc<dyn golem_gw_api::apis::DefaultApi>,
    hub_session: gu_client::r#async::HubSession,
    deployment: Option<gu_client::r#async::PeerSession>,
    spec: Option<blender::BlenderSubtaskSpec>,
    task: golem_gw_api::models::Task,
    subtask_id: Option<String>,
    node_id: String,
    peer_id: Option<NodeId>,
    state: State,
    output_uri: String,
    cnt: Counters,
}

#[derive(Default)]
struct Counters {
    subtasks_cnt: u64,
    subtasks_done_cnt: u64,
    subtasks_fail_cnt: u64,
}

pub struct DoSubTask(pub Subtask);

impl Message for DoSubTask {
    type Result = Result<(), gu_client::error::Error>;
}

pub struct DoResource(pub golem_gw_api::models::Resource);

impl Message for DoResource {
    type Result = Result<(), gu_client::error::Error>;
}

pub struct DoSubtaskVerification(pub golem_gw_api::models::SubtaskVerification);

impl Message for DoSubtaskVerification {
    type Result = Result<(), gu_client::error::Error>;
}

impl TaskWorker {
    pub fn new(
        gw_url: String,
        api: &Rc<dyn golem_gw_api::apis::DefaultApi>,
        hub_session: gu_client::r#async::HubSession,
        node_id: &str,
        task: &golem_gw_api::models::Task,
    ) -> Self {
        TaskWorker {
            gw_url,
            api: api.clone(),
            hub_session,
            node_id: node_id.to_owned(),
            task: task.clone(),
            peer_id: None,
            deployment: None,
            state: State::default(),
            output_uri: String::default(),
            spec: None,
            subtask_id: None,
            cnt: Counters::default(),
        }
    }

    fn resource_ready(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.state.resource_ready = true;
        self.start_processing(ctx)
    }

    fn spec_ready(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.state.spec_ready = true;
        self.start_processing(ctx)
    }

    fn start_processing(&mut self, ctx: &mut <Self as Actor>::Context) {
        use gu_client::model::envman::{Command, ResourceFormat};

        if !self.state.is_ready() {
            return;
        }

        let deployment = match self.deployment.as_ref() {
            Some(d) => d.clone(),
            None => {
                log::error!("!!! deployment not ready !!!");
                return;
            }
        };

        let output_file_name = self.spec.as_ref().unwrap().expected_output_file_name();
        let output_path = format!("/golem/output/{}", output_file_name);
        let output_uri = format!("{}/{}", self.output_uri, output_file_name);
        let result_path = format!("{}/output", self.task.task_id());

        self.state.mark_subtask_start();
        log::info!(
            "\n\nstarting blendering!!\n  subtask={}\n  out_file={}\n",
            self.subtask_id.clone().unwrap(),
            output_path,
        );

        let compute = self.hub_session.new_blob().and_then(move |b| {
            deployment.update(vec![
                Command::Open,
                Command::Wait,
                Command::UploadFile {
                    uri: b.uri(),
                    file_path: output_path.clone(),
                    format: ResourceFormat::Raw,
                },
                Command::UploadFile {
                    uri: output_uri,
                    file_path: output_path,
                    format: ResourceFormat::Raw,
                },
            ])
        });

        ctx.spawn(
            compute
                .into_actor(self)
                .map_err(|e, _, _| log::error!("\n\nblendering failed!!\n  err: {}", e))
                .and_then(|r, act: &mut TaskWorker, _ctx| {
                    log::info!(
                        "\n\nblendering done!!\n  results in: {}\n  {:?}",
                        result_path,
                        r
                    );

                    act.api
                        .subtask_result(
                            &act.node_id,
                            act.subtask_id.as_ref().unwrap(),
                            golem_gw_api::models::SubtaskResult::new(
                                "succeeded".into(),
                                result_path,
                            ),
                        )
                        .map_err(|e| log::error!("fail send result: {}", e))
                        .and_then(|_r| Ok(log::info!("sending results done")))
                        .into_actor(act)
                }),
        );
    }
}

#[derive(Default, Debug)]
struct State {
    resource_ready: bool,
    spec_ready: bool,
}

impl State {
    #[inline]
    fn is_ready(&self) -> bool {
        self.resource_ready && self.spec_ready
    }

    fn mark_subtask_start(&mut self) {
        self.spec_ready = false;
    }
}

impl Handler<DoSubTask> for TaskWorker {
    type Result = ActorResponse<TaskWorker, (), gu_client::error::Error>;

    fn handle(&mut self, msg: DoSubTask, ctx: &mut Self::Context) -> Self::Result {
        use gu_client::model::envman::Command;

        let mut subtask_spec: blender::BlenderSubtaskSpec =
            blender::decode(msg.0.extra_data().clone()).unwrap();

        subtask_spec.normalize_path();
        log::info!("got subtask {}; {}", msg.0.subtask_id(), subtask_spec);

        self.spec = Some(subtask_spec.clone());
        self.subtask_id = Some(msg.0.subtask_id().clone());

        let deployment = match self.deployment.as_ref() {
            Some(d) => d,
            None => {
                return ActorResponse::reply(Err(gu_client::error::Error::Other(
                    "deployment not ready".into(),
                )));
            }
        };

        self.cnt.subtasks_cnt += 1;
        let _ = ctx.spawn(
            self.api
                .confirm_subtask(&self.node_id, self.subtask_id.as_ref().unwrap())
                .into_actor(self)
                .map_err(|e, act, _| {
                    log::warn!("subtask {:?} confirmation failure: {}", act.subtask_id, e)
                })
                .and_then(|_r, act, _| {
                    log::info!("subtask {:?} confirmed", act.subtask_id);
                    fut::ok(())
                }),
        );

        let upload_spec = deployment.update(vec![Command::WriteFile {
            file_path: "golem/resources/spec.json".to_string(),
            content: serde_json::to_string(&subtask_spec).unwrap(),
        }]);

        ActorResponse::r#async(upload_spec.into_actor(self).and_then(
            |_r, act: &mut TaskWorker, ctx| {
                act.spec_ready(ctx);
                fut::ok(())
            },
        ))
    }
}

impl Handler<DoResource> for TaskWorker {
    type Result = ActorResponse<TaskWorker, (), gu_client::error::Error>;

    fn handle(&mut self, msg: DoResource, ctx: &mut Self::Context) -> Self::Result {
        use gu_client::model::envman::{Command, ResourceFormat};

        if self.state.resource_ready {
            return ActorResponse::reply(Ok(()));
        }

        let r = &msg.0;
        let zip_uri = format!("{}/{}/{}", self.gw_url, r.path(), r.task_id());
        let task_uri = format!("{}/{}", self.gw_url, r.task_id());

        self.subtask_id = Some(r.subtask_id().clone());
        log::info!("got resource for subtask {}", r.subtask_id());

        let deployment = match self.deployment.as_ref() {
            Some(d) => d,
            None => {
                return ActorResponse::reply(Err(gu_client::error::Error::Other(
                    "deployment not ready".into(),
                )));
            }
        };

        let upload_zip = deployment.update(vec![Command::DownloadFile {
            uri: zip_uri.clone(),
            file_path: "/golem/resources/gu.zip".to_string(),
            format: ResourceFormat::Raw,
        }]);

        let create_output = dav::DavPath::new(task_uri.parse().unwrap())
            .mkdir("output")
            .into_actor(self)
            .map_err(|e, _, _| log::warn!("unable to create output dir at {:?}", e))
            .and_then(|r, act: &mut TaskWorker, _| {
                act.output_uri = r.to_string();
                fut::ok(())
            });

        let _ = ctx.spawn(create_output);

        log::info!("got resource; path: {}", r.path());
        ActorResponse::r#async(upload_zip.into_actor(self).and_then(
            move |r, act: &mut TaskWorker, ctx| {
                act.resource_ready(ctx);
                fut::ok(log::info!(
                    "resource downloaded for {}: {:?}",
                    act.subtask_id.as_ref().unwrap_or(&"unknown subtask".into()),
                    r
                ))
            },
        ))
    }
}

impl Handler<DoSubtaskVerification> for TaskWorker {
    type Result = ActorResponse<TaskWorker, (), gu_client::error::Error>;

    fn handle(&mut self, msg: DoSubtaskVerification, _ctx: &mut Self::Context) -> Self::Result {
        let s_v = &msg.0;
        let subtask_id = s_v.subtask_id();

        if self.subtask_id.as_ref().unwrap() != subtask_id {
            log::warn!(
                "verification of {} but for {} needed",
                subtask_id,
                self.subtask_id.as_ref().unwrap()
            );
        }

        if s_v.verification_result() != "OK" {
            let reason = s_v
                .reason()
                .expect("negative verification should have reason");
            log::warn!("verification of {} failure : {:?}", subtask_id, reason);
            return ActorResponse::reply(Err(gu_client::error::Error::Other(format!(
                "subtask {} result not accepted: {}",
                subtask_id, reason
            ))));
        }

        self.cnt.subtasks_done_cnt += 1;

        log::info!("subtask {} verified successfully", s_v.subtask_id());
        ActorResponse::r#async(
            self.api
                .want_to_compute_task(&self.node_id, self.task.task_id())
                .into_actor(self)
                .and_then(|m, _, _| {
                    fut::ok(log::info!("want to compute (next) task send: {:?}", m))
                })
                .map_err(|e, act, _| {
                    let msg = format!("{:?}", e);
                    let task_not_found = format!("{} not found", act.task.task_id());
                    if msg.contains(task_not_found.as_str()) {
                        // TODO: clean-up after last subtask, use task deadline
                        // TODO: check if requestor sends NO_MORE_SUBTASKS to gw and pass it as an event
                        log::info!("task {} has finished", act.task.task_id());
                        gu_client::error::Error::Other("task finshed".into())
                    } else {
                        log::error!("want to compute (next) task failed: {:?}", e);
                        gu_client::error::Error::Other(e.to_string())
                    }
                }),
        )
    }
}

impl TaskWorker {
    fn create_deployment(&self) -> Box<dyn ActorFuture<Actor = TaskWorker, Item = (), Error = ()>> {
        Box::new(
            workman::reserve_for_session(
                self.hub_session.id(),
                self.task.task_id(),
                (*self.task.deadline()) as u64,
            )
            .map_err(|_| {
                /*TODO*/
                ()
            })
            .into_actor(self)
            .and_then(|peer_id, act: &mut TaskWorker, _| {
                act.peer_id = Some(peer_id);
                act.hub_session
                    .add_peers(vec![peer_id])
                    .into_actor(act)
                    .map_err(|e, act, _| {
                        log::error!("fail to add peer {:?}: {}", act.peer_id.unwrap(), e)
                    })
                    .and_then(|_, act: &mut TaskWorker, _| {
                        blender::blender_deployment_spec(
                            act.hub_session.peer(act.peer_id.unwrap()),
                            true,
                        )
                        .into_actor(act)
                        .map_err(|e, act, _| {
                            log::warn!(
                                "unable to create deployment @ peer: {:?}, err: {}",
                                act.peer_id.unwrap(),
                                e
                            )
                        })
                        .and_then(|deployment, act: &mut TaskWorker, _| {
                            act.deployment = Some(deployment);
                            fut::ok(())
                        })
                    })
            }),
        )
    }

    fn create_deployment_with_retry(
        &self,
        retry_cnt: u32,
    ) -> Box<dyn ActorFuture<Actor = TaskWorker, Item = (), Error = ()>> {
        Box::new(self.create_deployment().then(move |r, act, _| match r {
            Ok(v) => actix::fut::Either::A(fut::ok(v)),
            Err(e) => {
                if retry_cnt > 0 {
                    actix::fut::Either::B(act.create_deployment_with_retry(retry_cnt - 1))
                } else {
                    act.cnt.subtasks_fail_cnt += 1;
                    actix::fut::Either::A(fut::err(e))
                }
            }
        }))
    }
}

impl Actor for TaskWorker {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let get_subtask = self
            .api
            .want_to_compute_task(&self.node_id, self.task.task_id())
            .into_actor(self)
            .and_then(|m, _, _| fut::ok(log::info!("want to compute (first) task send: {:?}", m)))
            .map_err(|e, _, _| log::error!("want to compute (first) task failed: {:?}", e));

        ctx.wait(
            joinact::join_act_fut(get_subtask, self.create_deployment_with_retry(5))
                .and_then(|_, _, _| fut::ok(())),
        )
    }
}

use super::error::Error;
use super::gateway::{Stats, StatsData};
use crate::workman::reserve_for_session;

impl Handler<Stats> for TaskWorker {
    type Result = Result<StatsData, Error>;

    fn handle(&mut self, msg: Stats, ctx: &mut Self::Context) -> Self::Result {
        let result = StatsData {
            tasks: 0,
            subtasks: self.cnt.subtasks_cnt,
            subtasks_done: self.cnt.subtasks_done_cnt,
            fails: self.cnt.subtasks_fail_cnt,
        };
        self.cnt.subtasks_cnt = 0;
        self.cnt.subtasks_done_cnt = 0;
        self.cnt.subtasks_fail_cnt = 0;

        Ok(result)
    }
}
