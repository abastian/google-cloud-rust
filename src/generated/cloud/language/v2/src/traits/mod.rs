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

/// Provides text analysis operations such as sentiment analysis and entity
/// recognition.
///
/// # Mocking
///
/// Application developers may use this trait to mock the language clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait LanguageService: std::fmt::Debug + Send + Sync {
    /// Analyzes the sentiment of the provided text.
    fn analyze_sentiment(
        &self,
        _req: crate::model::AnalyzeSentimentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AnalyzeSentimentResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AnalyzeSentimentResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Finds named entities (currently proper names and common nouns) in the text
    /// along with entity types, probability, mentions for each entity, and
    /// other properties.
    fn analyze_entities(
        &self,
        _req: crate::model::AnalyzeEntitiesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AnalyzeEntitiesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AnalyzeEntitiesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Classifies a document into categories.
    fn classify_text(
        &self,
        _req: crate::model::ClassifyTextRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ClassifyTextResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ClassifyTextResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Moderates a document for harmful and sensitive categories.
    fn moderate_text(
        &self,
        _req: crate::model::ModerateTextRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ModerateTextResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ModerateTextResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// A convenience method that provides all features in one call.
    fn annotate_text(
        &self,
        _req: crate::model::AnnotateTextRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AnnotateTextResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AnnotateTextResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
