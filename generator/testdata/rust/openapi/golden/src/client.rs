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

use crate::Result;
use std::sync::Arc;

/// An implementation of [crate::traits::SecretManagerService] to make requests with.
///
/// `SecretManagerService` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `SecretManagerService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `SecretManagerService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// Stores sensitive data such as API keys, passwords, and certificates.
/// Provides convenience while improving security.
#[derive(Clone, Debug)]
pub struct SecretManagerService {
    inner: Arc<dyn crate::traits::dyntraits::SecretManagerService>,
}

impl SecretManagerService {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner }) 
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where T: crate::traits::SecretManagerService + 'static {
        Self { inner: Arc::new(stub) }
    }

    async fn build_inner(conf: gax::options::ClientConfig) -> Result<Arc<dyn crate::traits::dyntraits::SecretManagerService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gax::options::ClientConfig) -> Result<impl crate::traits::SecretManagerService> {
        crate::transport::SecretManagerService::new(conf).await
    }

    async fn build_with_tracing(conf: gax::options::ClientConfig) -> Result<impl crate::traits::SecretManagerService> {
        Self::build_transport(conf).await.map(crate::tracing::SecretManagerService::new)
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        project: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::ListLocations
    {
        crate::builders::secret_manager_service::ListLocations::new(self.inner.clone())
            .set_project ( project.into() )
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::GetLocation
    {
        crate::builders::secret_manager_service::GetLocation::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
    }

    /// Lists Secrets.
    pub fn list_secrets(
        &self,
        project: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::ListSecrets
    {
        crate::builders::secret_manager_service::ListSecrets::new(self.inner.clone())
            .set_project ( project.into() )
    }

    /// Creates a new Secret containing no SecretVersions.
    pub fn create_secret(
        &self,
        project: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::CreateSecret
    {
        crate::builders::secret_manager_service::CreateSecret::new(self.inner.clone())
            .set_project ( project.into() )
    }

    /// Lists Secrets.
    pub fn list_secrets_by_project_and_location(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::ListSecretsByProjectAndLocation
    {
        crate::builders::secret_manager_service::ListSecretsByProjectAndLocation::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
    }

    /// Creates a new Secret containing no SecretVersions.
    pub fn create_secret_by_project_and_location(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::CreateSecretByProjectAndLocation
    {
        crate::builders::secret_manager_service::CreateSecretByProjectAndLocation::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    pub fn add_secret_version(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::AddSecretVersion
    {
        crate::builders::secret_manager_service::AddSecretVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    pub fn add_secret_version_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::AddSecretVersionByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::AddSecretVersionByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

    /// Gets metadata for a given Secret.
    pub fn get_secret(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::GetSecret
    {
        crate::builders::secret_manager_service::GetSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Deletes a Secret.
    pub fn delete_secret(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::DeleteSecret
    {
        crate::builders::secret_manager_service::DeleteSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Updates metadata of an existing Secret.
    pub fn update_secret(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::UpdateSecret
    {
        crate::builders::secret_manager_service::UpdateSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Gets metadata for a given Secret.
    pub fn get_secret_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::GetSecretByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::GetSecretByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

    /// Deletes a Secret.
    pub fn delete_secret_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::DeleteSecretByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::DeleteSecretByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

    /// Updates metadata of an existing Secret.
    pub fn update_secret_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::UpdateSecretByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::UpdateSecretByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    pub fn list_secret_versions(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::ListSecretVersions
    {
        crate::builders::secret_manager_service::ListSecretVersions::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    pub fn list_secret_versions_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::ListSecretVersionsByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::ListSecretVersionsByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn get_secret_version(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::GetSecretVersion
    {
        crate::builders::secret_manager_service::GetSecretVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn get_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::GetSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        crate::builders::secret_manager_service::GetSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn access_secret_version(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::AccessSecretVersion
    {
        crate::builders::secret_manager_service::AccessSecretVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn access_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::AccessSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        crate::builders::secret_manager_service::AccessSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    pub fn disable_secret_version(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::DisableSecretVersion
    {
        crate::builders::secret_manager_service::DisableSecretVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    pub fn disable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::DisableSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        crate::builders::secret_manager_service::DisableSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    pub fn enable_secret_version(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::EnableSecretVersion
    {
        crate::builders::secret_manager_service::EnableSecretVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    pub fn enable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::EnableSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        crate::builders::secret_manager_service::EnableSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    pub fn destroy_secret_version(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::DestroySecretVersion
    {
        crate::builders::secret_manager_service::DestroySecretVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    pub fn destroy_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
        version: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::DestroySecretVersionByProjectAndLocationAndSecretAndVersion
    {
        crate::builders::secret_manager_service::DestroySecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
            .set_version ( version.into() )
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    pub fn set_iam_policy(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::SetIamPolicy
    {
        crate::builders::secret_manager_service::SetIamPolicy::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    pub fn set_iam_policy_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::SetIamPolicyByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::SetIamPolicyByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::GetIamPolicy
    {
        crate::builders::secret_manager_service::GetIamPolicy::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    pub fn get_iam_policy_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::GetIamPolicyByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::GetIamPolicyByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        project: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::TestIamPermissions
    {
        crate::builders::secret_manager_service::TestIamPermissions::new(self.inner.clone())
            .set_project ( project.into() )
            .set_secret ( secret.into() )
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub fn test_iam_permissions_by_project_and_location_and_secret(
        &self,
        project: impl Into<std::string::String>,
        location: impl Into<std::string::String>,
        secret: impl Into<std::string::String>,
    ) -> crate::builders::secret_manager_service::TestIamPermissionsByProjectAndLocationAndSecret
    {
        crate::builders::secret_manager_service::TestIamPermissionsByProjectAndLocationAndSecret::new(self.inner.clone())
            .set_project ( project.into() )
            .set_location ( location.into() )
            .set_secret ( secret.into() )
    }

}
