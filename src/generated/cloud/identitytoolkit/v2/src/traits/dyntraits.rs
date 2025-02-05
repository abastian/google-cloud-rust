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

/// A dyn-compatible, crate-private version of `AccountManagementService`.
#[async_trait::async_trait]
pub trait AccountManagementService: std::fmt::Debug + Send + Sync {
    async fn finalize_mfa_enrollment(
        &self,
        req: crate::model::FinalizeMfaEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FinalizeMfaEnrollmentResponse>;

    async fn start_mfa_enrollment(
        &self,
        req: crate::model::StartMfaEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StartMfaEnrollmentResponse>;

    async fn withdraw_mfa(
        &self,
        req: crate::model::WithdrawMfaRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::WithdrawMfaResponse>;
}

/// All implementations of [crate::traits::AccountManagementService] also implement [AccountManagementService].
#[async_trait::async_trait]
impl<T: crate::traits::AccountManagementService> AccountManagementService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn finalize_mfa_enrollment(
        &self,
        req: crate::model::FinalizeMfaEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FinalizeMfaEnrollmentResponse> {
        T::finalize_mfa_enrollment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn start_mfa_enrollment(
        &self,
        req: crate::model::StartMfaEnrollmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StartMfaEnrollmentResponse> {
        T::start_mfa_enrollment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn withdraw_mfa(
        &self,
        req: crate::model::WithdrawMfaRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::WithdrawMfaResponse> {
        T::withdraw_mfa(self, req, options).await
    }
}

/// A dyn-compatible, crate-private version of `AuthenticationService`.
#[async_trait::async_trait]
pub trait AuthenticationService: std::fmt::Debug + Send + Sync {
    async fn finalize_mfa_sign_in(
        &self,
        req: crate::model::FinalizeMfaSignInRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FinalizeMfaSignInResponse>;

    async fn start_mfa_sign_in(
        &self,
        req: crate::model::StartMfaSignInRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StartMfaSignInResponse>;
}

/// All implementations of [crate::traits::AuthenticationService] also implement [AuthenticationService].
#[async_trait::async_trait]
impl<T: crate::traits::AuthenticationService> AuthenticationService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn finalize_mfa_sign_in(
        &self,
        req: crate::model::FinalizeMfaSignInRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::FinalizeMfaSignInResponse> {
        T::finalize_mfa_sign_in(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn start_mfa_sign_in(
        &self,
        req: crate::model::StartMfaSignInRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::StartMfaSignInResponse> {
        T::start_mfa_sign_in(self, req, options).await
    }
}
