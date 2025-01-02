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
/// Secret Manager Service
///
/// Manages secrets and operations using those secrets. Implements a REST
/// model with the following objects:
///
/// * [Secret][google.cloud.secretmanager.v1.Secret]
/// * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
///
/// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
/// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
#[derive(Clone, Debug)]
pub struct SecretManagerService {
    inner: Arc<dyn crate::traits::dyntraits::SecretManagerService>,
}

impl SecretManagerService {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::traits::SecretManagerService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: crate::ConfigBuilder,
    ) -> Result<Arc<dyn crate::traits::dyntraits::SecretManagerService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: crate::ConfigBuilder,
    ) -> Result<impl crate::traits::SecretManagerService> {
        crate::transport::SecretManagerService::new(conf).await
    }

    async fn build_with_tracing(
        conf: crate::ConfigBuilder,
    ) -> Result<impl crate::traits::SecretManagerService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::SecretManagerService::new)
    }

    /// Lists [Secrets][google.cloud.secretmanager.v1.Secret].
    ///
    /// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
    pub fn list_secrets(&self, parent: impl Into<String>) -> crate::builders::ListSecrets {
        crate::builders::ListSecrets::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    pub fn create_secret(&self, parent: impl Into<String>) -> crate::builders::CreateSecret {
        crate::builders::CreateSecret::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
    /// containing secret data and attaches it to an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    pub fn add_secret_version(
        &self,
        parent: impl Into<String>,
    ) -> crate::builders::AddSecretVersion {
        crate::builders::AddSecretVersion::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
    pub fn get_secret(&self, name: impl Into<String>) -> crate::builders::GetSecret {
        crate::builders::GetSecret::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates metadata of an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
    pub fn update_secret(
        &self,
        secret: impl Into<crate::model::Secret>,
    ) -> crate::builders::UpdateSecret {
        crate::builders::UpdateSecret::new(self.inner.clone()).set_secret(secret.into())
    }

    /// Deletes a [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
    pub fn delete_secret(&self, name: impl Into<String>) -> crate::builders::DeleteSecret {
        crate::builders::DeleteSecret::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This
    /// call does not return secret data.
    ///
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    pub fn list_secret_versions(
        &self,
        parent: impl Into<String>,
    ) -> crate::builders::ListSecretVersions {
        crate::builders::ListSecretVersions::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets metadata for a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    pub fn get_secret_version(&self, name: impl Into<String>) -> crate::builders::GetSecretVersion {
        crate::builders::GetSecretVersion::new(self.inner.clone()).set_name(name.into())
    }

    /// Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    /// This call returns the secret data.
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    pub fn access_secret_version(
        &self,
        name: impl Into<String>,
    ) -> crate::builders::AccessSecretVersion {
        crate::builders::AccessSecretVersion::new(self.inner.clone()).set_name(name.into())
    }

    /// Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED].
    ///
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    /// [google.cloud.secretmanager.v1.SecretVersion.State.DISABLED]: crate::model::secret_version::state::DISABLED
    /// [google.cloud.secretmanager.v1.SecretVersion.state]: crate::model::SecretVersion::state
    pub fn disable_secret_version(
        &self,
        name: impl Into<String>,
    ) -> crate::builders::DisableSecretVersion {
        crate::builders::DisableSecretVersion::new(self.inner.clone()).set_name(name.into())
    }

    /// Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED].
    ///
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    /// [google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]: crate::model::secret_version::state::ENABLED
    /// [google.cloud.secretmanager.v1.SecretVersion.state]: crate::model::SecretVersion::state
    pub fn enable_secret_version(
        &self,
        name: impl Into<String>,
    ) -> crate::builders::EnableSecretVersion {
        crate::builders::EnableSecretVersion::new(self.inner.clone()).set_name(name.into())
    }

    /// Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED]
    /// and irrevocably destroys the secret data.
    ///
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    /// [google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED]: crate::model::secret_version::state::DESTROYED
    /// [google.cloud.secretmanager.v1.SecretVersion.state]: crate::model::SecretVersion::state
    pub fn destroy_secret_version(
        &self,
        name: impl Into<String>,
    ) -> crate::builders::DestroySecretVersion {
        crate::builders::DestroySecretVersion::new(self.inner.clone()).set_name(name.into())
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced
    /// according to the policy set on the associated
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// [google.cloud.secretmanager.v1.Secret]: crate::model::Secret
    /// [google.cloud.secretmanager.v1.SecretVersion]: crate::model::SecretVersion
    pub fn set_iam_policy(&self, resource: impl Into<String>) -> crate::builders::SetIamPolicy {
        crate::builders::SetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    pub fn get_iam_policy(&self, resource: impl Into<String>) -> crate::builders::GetIamPolicy {
        crate::builders::GetIamPolicy::new(self.inner.clone()).set_resource(resource.into())
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
        resource: impl Into<String>,
    ) -> crate::builders::TestIamPermissions {
        crate::builders::TestIamPermissions::new(self.inner.clone()).set_resource(resource.into())
    }
}

/// An implementation of [crate::traits::Locations] to make requests with.
///
/// `Locations` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `Locations` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Locations` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// Manages location-related information with an API service.
#[derive(Clone, Debug)]
pub struct Locations {
    inner: Arc<dyn crate::traits::dyntraits::Locations>,
}

impl Locations {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::traits::Locations + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: crate::ConfigBuilder,
    ) -> Result<Arc<dyn crate::traits::dyntraits::Locations>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: crate::ConfigBuilder) -> Result<impl crate::traits::Locations> {
        crate::transport::Locations::new(conf).await
    }

    async fn build_with_tracing(
        conf: crate::ConfigBuilder,
    ) -> Result<impl crate::traits::Locations> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Locations::new)
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self, name: impl Into<String>) -> crate::builders::ListLocations {
        crate::builders::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(&self, name: impl Into<String>) -> crate::builders::GetLocation {
        crate::builders::GetLocation::new(self.inner.clone()).set_name(name.into())
    }
}
