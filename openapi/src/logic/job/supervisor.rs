use crate::{
    entity::{executor, job, job_exec_history, job_supervisor, prelude::*, tag_resource, team},
    logic::types::ResourceType,
};
use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait, Set,
};
use sea_query::Query;

use super::{
    types::JobSupervisorRelatedJobModel, Executor, Job, JobExecHistory, JobLogic, JobSupervisor,
    Team,
};

impl<'a> JobLogic<'a> {
    pub async fn query_job_supervisor(
        &self,
        created_user: Option<&String>,
        name: Option<String>,
        eid: Option<String>,
        team_id: Option<u64>,
        updated_time_range: Option<(String, String)>,
        tag_ids: Option<Vec<u64>>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<JobSupervisorRelatedJobModel>, u64)> {
        let mut select = job_supervisor::Entity::find()
            .column_as(job::Column::Name, "job_name")
            .column_as(executor::Column::Name, "executor_name")
            .column_as(executor::Column::Platform, "executor_platform")
            .column_as(team::Column::Name, "team_name")
            .column_as(team::Column::Id, "team_id")
            .column(job::Column::ExecutorId)
            .join_rev(
                JoinType::LeftJoin,
                Job::belongs_to(JobSupervisor)
                    .from(job::Column::Eid)
                    .to(job_supervisor::Column::Eid)
                    .into(),
            )
            .join_rev(
                JoinType::LeftJoin,
                Executor::belongs_to(Job)
                    .from(executor::Column::Id)
                    .to(job::Column::ExecutorId)
                    .into(),
            )
            .join_rev(
                JoinType::LeftJoin,
                Team::belongs_to(Job)
                    .from(team::Column::Id)
                    .to(job::Column::TeamId)
                    .into(),
            )
            .apply_if(name, |query, v| {
                query.filter(job_supervisor::Column::Name.contains(v))
            })
            .apply_if(created_user, |query, v| {
                query.filter(job_supervisor::Column::CreatedUser.eq(v))
            })
            .apply_if(team_id, |q, v| q.filter(job::Column::TeamId.eq(v)))
            .apply_if(eid, |q, v| q.filter(job_supervisor::Column::Eid.eq(v)))
            .apply_if(updated_time_range, |query, v| {
                query.filter(
                    job_supervisor::Column::UpdatedTime
                        .gt(v.0)
                        .and(job_supervisor::Column::UpdatedTime.lt(v.1)),
                )
            });

        match tag_ids {
            Some(v) if v.len() > 0 => {
                select = select.filter(
                    Condition::any().add(
                        job::Column::Id.in_subquery(
                            Query::select()
                                .column(tag_resource::Column::ResourceId)
                                .and_where(
                                    tag_resource::Column::ResourceType
                                        .eq(ResourceType::Job.to_string())
                                        .and(tag_resource::Column::TagId.is_in(v)),
                                )
                                .from(TagResource)
                                .to_owned(),
                        ),
                    ),
                );
            }
            _ => {}
        };

        let total = select.clone().count(&self.ctx.db).await?;

        let list = select
            .order_by_desc(job_supervisor::Column::Id)
            .into_model()
            .paginate(&self.ctx.db, page_size)
            .fetch_page(page)
            .await?;

        Ok((list, total))
    }

    pub async fn save_job_supervisor(
        &self,
        active_model: job_supervisor::ActiveModel,
    ) -> Result<job_supervisor::ActiveModel> {
        Ok(active_model.save(&self.ctx.db).await?)
    }

    pub async fn delete_job_supervisor(&self, eid: String) -> Result<u64> {
        let record = JobExecHistory::find()
            .filter(job_exec_history::Column::Eid.eq(&eid))
            .one(&self.ctx.db)
            .await?;
        if record.is_some() {
            anyhow::bail!("forbidden to delete the executed jobs")
        }
        let ret = JobSupervisor::delete(job_supervisor::ActiveModel {
            eid: Set(eid),
            ..Default::default()
        })
        .exec(&self.ctx.db)
        .await?;
        Ok(ret.rows_affected)
    }
}
