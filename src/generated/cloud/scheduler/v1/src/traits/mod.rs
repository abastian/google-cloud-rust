// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;

pub(crate) mod dyntraits;

/// The Cloud Scheduler API allows external entities to reliably
/// schedule asynchronous jobs.
///
/// # Mocking
///
/// Application developers may use this trait to mock the cloudscheduler clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait CloudScheduler: std::fmt::Debug + Send + Sync {
    /// Lists jobs.
    fn list_jobs(
        &self,
        _req: crate::model::ListJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListJobsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListJobsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets a job.
    fn get_job(
        &self,
        _req: crate::model::GetJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Creates a job.
    fn create_job(
        &self,
        _req: crate::model::CreateJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Updates a job.
    ///
    /// If successful, the updated [Job][google.cloud.scheduler.v1.Job] is
    /// returned. If the job does not exist, `NOT_FOUND` is returned.
    ///
    /// If UpdateJob does not successfully return, it is possible for the
    /// job to be in an
    /// [Job.State.UPDATE_FAILED][google.cloud.scheduler.v1.Job.State.UPDATE_FAILED]
    /// state. A job in this state may not be executed. If this happens, retry the
    /// UpdateJob request until a successful response is received.
    ///
    /// [google.cloud.scheduler.v1.Job]: crate::model::Job
    /// [google.cloud.scheduler.v1.Job.State.UPDATE_FAILED]: crate::model::job::state::UPDATE_FAILED
    fn update_job(
        &self,
        _req: crate::model::UpdateJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Deletes a job.
    fn delete_job(
        &self,
        _req: crate::model::DeleteJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Pauses a job.
    ///
    /// If a job is paused then the system will stop executing the job
    /// until it is re-enabled via
    /// [ResumeJob][google.cloud.scheduler.v1.CloudScheduler.ResumeJob]. The state
    /// of the job is stored in [state][google.cloud.scheduler.v1.Job.state]; if
    /// paused it will be set to
    /// [Job.State.PAUSED][google.cloud.scheduler.v1.Job.State.PAUSED]. A job must
    /// be in [Job.State.ENABLED][google.cloud.scheduler.v1.Job.State.ENABLED] to
    /// be paused.
    ///
    /// [google.cloud.scheduler.v1.CloudScheduler.ResumeJob]: crate::client::CloudScheduler::resume_job
    /// [google.cloud.scheduler.v1.Job.State.ENABLED]: crate::model::job::state::ENABLED
    /// [google.cloud.scheduler.v1.Job.State.PAUSED]: crate::model::job::state::PAUSED
    /// [google.cloud.scheduler.v1.Job.state]: crate::model::Job::state
    fn pause_job(
        &self,
        _req: crate::model::PauseJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Resume a job.
    ///
    /// This method reenables a job after it has been
    /// [Job.State.PAUSED][google.cloud.scheduler.v1.Job.State.PAUSED]. The state
    /// of a job is stored in [Job.state][google.cloud.scheduler.v1.Job.state];
    /// after calling this method it will be set to
    /// [Job.State.ENABLED][google.cloud.scheduler.v1.Job.State.ENABLED]. A job
    /// must be in [Job.State.PAUSED][google.cloud.scheduler.v1.Job.State.PAUSED]
    /// to be resumed.
    ///
    /// [google.cloud.scheduler.v1.Job.State.ENABLED]: crate::model::job::state::ENABLED
    /// [google.cloud.scheduler.v1.Job.State.PAUSED]: crate::model::job::state::PAUSED
    /// [google.cloud.scheduler.v1.Job.state]: crate::model::Job::state
    fn resume_job(
        &self,
        _req: crate::model::ResumeJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Forces a job to run now.
    ///
    /// When this method is called, Cloud Scheduler will dispatch the job, even
    /// if the job is already running.
    fn run_job(
        &self,
        _req: crate::model::RunJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Job>> + Send {
        std::future::ready::<crate::Result<crate::model::Job>>(Err(Error::other("unimplemented")))
    }

    /// Lists information about the supported locations for this service.
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Gets information about a location.
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
