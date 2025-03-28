use crate::entity::executor;
use crate::entity::job;
use crate::entity::job_bundle_script;
use crate::entity::prelude::*;
use crate::entity::team;
use anyhow::Result;
use sea_orm::Condition;
use sea_orm::JoinType;
use sea_orm::QuerySelect;
use sea_orm::QueryTrait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use sea_query::Expr;

use super::types;
use super::JobLogic;

impl<'a> JobLogic<'a> {
    pub async fn save_job_bundle_script(
        &self,
        active_model: job_bundle_script::ActiveModel,
    ) -> Result<job_bundle_script::ActiveModel> {
        let active_model = active_model.save(&self.ctx.db).await?;
        Ok(active_model)
    }

    pub async fn query_bundle_script(
        &self,
        username: Option<String>,
        team_id: Option<u64>,
        default_eid: Option<String>,
        name: Option<String>,
        updated_time_range: Option<(String, String)>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<types::BundleScriptRelatedExecutorModel>, u64)> {
        let model = JobBundleScript::find()
            .column_as(executor::Column::Name, "executor_name")
            .column_as(team::Column::Name, "team_name")
            .join_rev(
                JoinType::LeftJoin,
                Executor::belongs_to(JobBundleScript)
                    .from(executor::Column::Id)
                    .to(job_bundle_script::Column::ExecutorId)
                    .into(),
            )
            .join_rev(
                JoinType::LeftJoin,
                Team::belongs_to(JobBundleScript)
                    .from(team::Column::Id)
                    .to(job_bundle_script::Column::TeamId)
                    .into(),
            )
            .apply_if(username, |q, v| {
                q.filter(job_bundle_script::Column::CreatedUser.eq(v))
            })
            .apply_if(name, |query, v| {
                query.filter(job_bundle_script::Column::Name.contains(v))
            })
            .apply_if(updated_time_range, |query, v| {
                query.filter(
                    job_bundle_script::Column::UpdatedTime
                        .gt(v.0)
                        .and(job_bundle_script::Column::UpdatedTime.lt(v.1)),
                )
            })
            .apply_if(team_id, |q, v| {
                q.filter(job_bundle_script::Column::TeamId.eq(v))
            });

        let total = model.clone().count(&self.ctx.db).await?;
        let list = model
            .apply_if(default_eid, |query, v| {
                query.order_by_desc(Expr::expr(job_bundle_script::Column::Eid.eq(v)))
            })
            .order_by_desc(job_bundle_script::Column::Id)
            .into_model()
            .paginate(&self.ctx.db, page_size)
            .fetch_page(page)
            .await?;
        Ok((list, total))
    }

    pub async fn delete_bundle_script(&self, username: Option<String>, eid: String) -> Result<u64> {
        let cond = Condition::all().add(Expr::cust_with_values(
            "JSON_CONTAIN(bunle_script, ?)",
            vec![serde_json::json!({ "eid": eid.clone() })],
        ));

        let has = Job::find()
            .filter(cond)
            .filter(job::Column::JobType.eq("bundle"))
            .one(&self.ctx.db)
            .await?;
        if has.is_some() {
            anyhow::bail!("this bundle script is used by job");
        }

        let ret = JobBundleScript::delete_many()
            .apply_if(username, |q, v| {
                q.filter(job_bundle_script::Column::CreatedUser.eq(v))
            })
            .filter(job_bundle_script::Column::Eid.eq(eid))
            .exec(&self.ctx.db)
            .await?;
        Ok(ret.rows_affected)
    }
}
