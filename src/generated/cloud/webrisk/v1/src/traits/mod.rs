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

/// Web Risk API defines an interface to detect malicious URLs on your
/// website and in client applications.
///
/// # Mocking
///
/// Application developers may use this trait to mock the webrisk clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait WebRiskService: std::fmt::Debug + Send + Sync {
    /// Gets the most recent threat list diffs. These diffs should be applied to
    /// a local database of hashes to keep it up-to-date. If the local database is
    /// empty or excessively out-of-date, a complete snapshot of the database will
    /// be returned. This Method only updates a single ThreatList at a time. To
    /// update multiple ThreatList databases, this method needs to be called once
    /// for each list.
    fn compute_threat_list_diff(
        &self,
        _req: crate::model::ComputeThreatListDiffRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ComputeThreatListDiffResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ComputeThreatListDiffResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// This method is used to check whether a URI is on a given threatList.
    /// Multiple threatLists may be searched in a single query.
    /// The response will list all requested threatLists the URI was found to
    /// match. If the URI is not found on any of the requested ThreatList an
    /// empty response will be returned.
    fn search_uris(
        &self,
        _req: crate::model::SearchUrisRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SearchUrisResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SearchUrisResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the full hashes that match the requested hash prefix.
    /// This is used after a hash prefix is looked up in a threatList
    /// and there is a match. The client side threatList only holds partial hashes
    /// so the client must query this method to determine if there is a full
    /// hash match of a threat.
    fn search_hashes(
        &self,
        _req: crate::model::SearchHashesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SearchHashesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SearchHashesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a Submission of a URI suspected of containing phishing content to
    /// be reviewed. If the result verifies the existence of malicious phishing
    /// content, the site will be added to the [Google's Social Engineering
    /// lists](https://support.google.com/webmasters/answer/6350487/) in order to
    /// protect users that could get exposed to this threat in the future. Only
    /// allowlisted projects can use this method during Early Access. Please reach
    /// out to Sales or your customer engineer to obtain access.
    fn create_submission(
        &self,
        _req: crate::model::CreateSubmissionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Submission>> + Send {
        std::future::ready::<crate::Result<crate::model::Submission>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Submits a URI suspected of containing malicious content to be reviewed.
    /// Returns a google.longrunning.Operation which, once the review is complete,
    /// is updated with its result. You can use the [Pub/Sub API]
    /// (<https://cloud.google.com/pubsub>) to receive notifications for the returned
    /// Operation. If the result verifies the existence of malicious content, the
    /// site will be added to the [Google's Social Engineering lists]
    /// (<https://support.google.com/webmasters/answer/6350487/>) in order to
    /// protect users that could get exposed to this threat in the future. Only
    /// allowlisted projects can use this method during Early Access. Please reach
    /// out to Sales or your customer engineer to obtain access.
    fn submit_uri(
        &self,
        _req: crate::model::SubmitUriRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}
