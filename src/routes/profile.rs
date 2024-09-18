use std::time::Duration;

use aws_sdk_s3::{presigning::PresigningConfig, Client};
use axum::{extract::State, Json};
use indexmap::IndexMap;
use serde::Serialize;
use sqlx::PgPool;
use tokio::try_join;
use utoipa::ToSchema;

use crate::{
    errors::{s3::S3Error, AppError},
    AppState,
};

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    image_url: String,
    about_me: Vec<String>,
    skills: Skills,
    projects: Vec<Projects>,
    job_experience: Vec<JobExperience>,
}

#[derive(ToSchema, Serialize)]
pub struct Skills {
    languages: Vec<String>,
    tools: Vec<String>,
    frameworks: Vec<String>,
    others: Vec<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JobExperience {
    company_name: String,
    company_url: String,
    job_in_company: Vec<JobInCompany>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JobInCompany {
    job_title: String,
    time_span: String,
    description: Option<Vec<String>>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Projects {
    project_name: String,
    project_url: String,
    description: Vec<String>,
}

#[utoipa::path(
    get,
    tag="profile",
    path="/profile",
    responses(
        (status = 200, body = Profile),
        (status = 505, description = "internal server error")
    )
)]
#[tracing::instrument(skip_all)]
pub async fn get_profile(State(app): State<AppState>) -> Result<Json<Profile>, AppError> {
    let pool = app.pool;

    let (image_url, skills, about_me, projects, job_experience) = try_join!(
        get_profile_image(app.s3),
        get_profile_skills(&pool),
        get_profile_about_me(&pool),
        get_projects(&pool),
        get_jobs_in_company(&pool)
    )?;

    Ok(axum::Json(Profile {
        image_url,
        about_me,
        skills,
        projects,
        job_experience,
    }))
}

#[tracing::instrument(skip_all)]
async fn get_profile_image(s3_client: Client) -> Result<String, AppError> {
    let presign_config =
        PresigningConfig::expires_in(Duration::from_secs(60 * 60)).map_err(S3Error::Presign)?;

    let presigned = s3_client
        .get_object()
        .bucket("misc")
        .key("profile.jpg")
        .presigned(presign_config)
        .await
        .map_err(|e| {
            tracing::error!("{e:#?}");
            S3Error::SdkError("can't get presign".into())
        })?;

    Ok(presigned.uri().to_owned())
}

async fn get_profile_about_me(pool: &PgPool) -> Result<Vec<String>, AppError> {
    let records = sqlx::query!("select about_me from profile_about_me")
        .fetch_all(pool)
        .await?;
    Ok(records.into_iter().map(|x| x.about_me).collect())
}

async fn get_profile_skills(pool: &PgPool) -> Result<Skills, AppError> {
    let lang_query = sqlx::query!("select languages from profile_skills_languages").fetch_all(pool);
    let tools_query = sqlx::query!("select tools from profile_skills_tools").fetch_all(pool);
    let fw_query = sqlx::query!("select frameworks from profile_skills_frameworks").fetch_all(pool);
    let others_query = sqlx::query!("select others from profile_skills_others").fetch_all(pool);

    let (lang_rec, tool_rec, fw_rec, other_rec) =
        try_join!(lang_query, tools_query, fw_query, others_query)?;

    let languages = lang_rec.into_iter().map(|x| x.languages).collect();
    let tools = tool_rec.into_iter().map(|x| x.tools).collect();
    let frameworks = fw_rec.into_iter().map(|x| x.frameworks).collect();
    let others = other_rec.into_iter().map(|x| x.others).collect();

    Ok(Skills {
        languages,
        tools,
        frameworks,
        others,
    })
}

async fn get_projects(pool: &PgPool) -> Result<Vec<Projects>, AppError> {
    let projects = sqlx::query_as!(
        Projects,
        "select
        project_name, project_url, description
        from profile_projects
        order by id desc
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

struct JobInfo {
    company_name: String,
    company_url: String,
    job_title: String,
    time_span: String,
    description: Option<Vec<String>>,
}

async fn get_jobs_in_company(pool: &PgPool) -> Result<Vec<JobExperience>, AppError> {
    let job_info: Vec<JobInfo> = sqlx::query_as!(
        JobInfo,
        "select company_name, company_url, job_title , time_span, description 
        from profile_jobs
        order by id desc"
    )
    .fetch_all(pool)
    .await?;

    let mut job_info_hashmap: IndexMap<String, JobExperience> = IndexMap::new();

    for info in job_info {
        job_info_hashmap
            .entry(info.company_name.clone())
            .and_modify(|e| {
                e.job_in_company.push(JobInCompany {
                    job_title: info.job_title.clone(),
                    time_span: info.time_span.clone(),
                    description: info.description.clone(),
                });
            })
            .or_insert(JobExperience {
                company_name: info.company_name,
                company_url: info.company_url,
                job_in_company: vec![JobInCompany {
                    job_title: info.job_title,
                    time_span: info.time_span,
                    description: info.description,
                }],
            });
    }
    Ok(job_info_hashmap.into_values().collect())
}
