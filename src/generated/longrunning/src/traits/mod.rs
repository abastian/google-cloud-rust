// Copyright 2024 Google LLC
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

/// Manages long-running operations with an API service.
///
/// When an API method normally takes long time to complete, it can be designed
/// to return [Operation][google.longrunning.Operation] to the client, and the
/// client can use this interface to receive the real response asynchronously by
/// polling the operation resource, or pass the operation resource to another API
/// (such as Pub/Sub API) to receive the response.  Any API service that returns
/// long-running operations should implement the `Operations` interface so
/// developers can have a consistent client experience.
///
/// [google.longrunning.Operation]: crate::model::Operation
///
/// # Mocking
///
/// Application developers may use this trait to mock the longrunning clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Operations: std::fmt::Debug + Send + Sync {
    /// Lists operations that match the specified filter in the request. If the
    /// server doesn't support this method, it returns `UNIMPLEMENTED`.
    fn list_operations(
        &self,
        _req: crate::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListOperationsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    fn get_operation(
        &self,
        _req: crate::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a long-running operation. This method indicates that the client is
    /// no longer interested in the operation result. It does not cancel the
    /// operation. If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.
    fn delete_operation(
        &self,
        _req: crate::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Starts asynchronous cancellation on a long-running operation.  The server
    /// makes a best effort to cancel the operation, but success is not
    /// guaranteed.  If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
    /// [Operations.GetOperation][google.longrunning.Operations.GetOperation] or
    /// other methods to check whether the cancellation succeeded or whether the
    /// operation completed despite cancellation. On successful cancellation,
    /// the operation is not deleted; instead, it becomes an operation with
    /// an [Operation.error][google.longrunning.Operation.error] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of `1`, corresponding to
    /// `Code.CANCELLED`.
    ///
    /// [google.longrunning.Operation.error]: crate::model::Operation::result
    /// [google.longrunning.Operations.GetOperation]: crate::client::Operations::get_operation
    /// [google.rpc.Status.code]: rpc::model::Status::code
    fn cancel_operation(
        &self,
        _req: crate::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }
}
