-- Add migration script here
create table profile_about_me (id serial primary key, about_me text not null);

create table profile_skills_languages (id serial primary key, languages text not null);

create table profile_skills_tools (id serial primary key, tools text not null);

create table profile_skills_frameworks (id serial primary key, frameworks text not null);

create table profile_skills_others (id serial primary key, others text not null);

create table profile_projects (
  id serial primary key,
  project_name text not null,
  project_url text not null,
  description text[] not null
);

create table profile_jobs (
  id serial primary key,
  company_name text not null,
  company_url text not null,
  job_title text not null,
  time_span text not null,
  description text[]
);
