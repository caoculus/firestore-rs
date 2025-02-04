use crate::db::safe_document_path;
use crate::{FirestoreDb, FirestoreError, FirestoreResult};
use async_trait::async_trait;
use chrono::prelude::*;
use futures::future::{BoxFuture, FutureExt};
use futures::stream::BoxStream;
use futures::TryFutureExt;
use futures::TryStreamExt;
use futures::{future, StreamExt};
use gcloud_sdk::google::firestore::v1::*;
use serde::Deserialize;
use tracing::*;

#[async_trait]
pub trait FirestoreGetByIdSupport {
    async fn get_doc<S>(
        &self,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Document>
    where
        S: AsRef<str> + Send;

    async fn get_doc_at<S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Document>
    where
        S: AsRef<str> + Send;

    async fn get_obj<T, S>(&self, collection_id: &str, document_id: S) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send;

    async fn get_obj_return_fields<T, S>(
        &self,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send;

    async fn get_obj_at<T, S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
    ) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send;

    async fn get_obj_at_return_fields<T, S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send;

    async fn get_obj_if_exists<T, S>(
        &self,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Option<T>>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send;

    async fn get_obj_at_if_exists<T, S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Option<T>>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send;

    async fn batch_stream_get_docs<S, I>(
        &self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<(String, Option<Document>)>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;

    async fn batch_stream_get_docs_with_errors<S, I>(
        &self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<FirestoreResult<(String, Option<Document>)>>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;

    async fn batch_stream_get_docs_at<S, I>(
        &self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<(String, Option<Document>)>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;

    async fn batch_stream_get_docs_at_with_errors<S, I>(
        &self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<FirestoreResult<(String, Option<Document>)>>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;

    async fn batch_stream_get_objects<'a, T, S, I>(
        &'a self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'a, (String, Option<T>)>>
    where
        for<'de> T: Deserialize<'de> + Send + 'a,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;

    async fn batch_stream_get_objects_with_errors<'a, T, S, I>(
        &'a self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'a, FirestoreResult<(String, Option<T>)>>>
    where
        for<'de> T: Deserialize<'de> + Send + 'a,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;

    async fn batch_stream_get_objects_at<'a, T, S, I>(
        &'a self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'a, (String, Option<T>)>>
    where
        for<'de> T: Deserialize<'de> + Send + 'a,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;

    async fn batch_stream_get_objects_at_with_errors<'a, T, S, I>(
        &'a self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'a, FirestoreResult<(String, Option<T>)>>>
    where
        for<'de> T: Deserialize<'de> + Send + 'a,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send;
}

#[async_trait]
impl FirestoreGetByIdSupport for FirestoreDb {
    async fn get_doc_at<S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Document>
    where
        S: AsRef<str> + Send,
    {
        let document_path = safe_document_path(parent, collection_id, document_id.as_ref())?;
        self.get_doc_by_path(document_path, return_only_fields, 0)
            .await
    }

    async fn get_doc<S>(
        &self,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Document>
    where
        S: AsRef<str> + Send,
    {
        self.get_doc_at(
            self.get_documents_path().as_str(),
            collection_id,
            document_id,
            return_only_fields,
        )
        .await
    }

    async fn get_obj<T, S>(&self, collection_id: &str, document_id: S) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send,
    {
        self.get_obj_at(
            self.get_documents_path().as_str(),
            collection_id,
            document_id,
        )
        .await
    }

    async fn get_obj_return_fields<T, S>(
        &self,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send,
    {
        self.get_obj_at_return_fields(
            self.get_documents_path().as_str(),
            collection_id,
            document_id,
            return_only_fields,
        )
        .await
    }

    async fn get_obj_at<T, S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
    ) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send,
    {
        let doc: Document = self
            .get_doc_at(parent, collection_id, document_id, None)
            .await?;

        let obj: T = Self::deserialize_doc_to(&doc)?;
        Ok(obj)
    }

    async fn get_obj_at_return_fields<T, S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<T>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send,
    {
        let doc: Document = self
            .get_doc_at(parent, collection_id, document_id, return_only_fields)
            .await?;

        let obj: T = Self::deserialize_doc_to(&doc)?;
        Ok(obj)
    }

    async fn get_obj_if_exists<T, S>(
        &self,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Option<T>>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send,
    {
        self.get_obj_at_if_exists(
            self.get_documents_path().as_str(),
            collection_id,
            document_id,
            return_only_fields,
        )
        .await
    }

    async fn get_obj_at_if_exists<T, S>(
        &self,
        parent: &str,
        collection_id: &str,
        document_id: S,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<Option<T>>
    where
        for<'de> T: Deserialize<'de>,
        S: AsRef<str> + Send,
    {
        match self
            .get_obj_at_return_fields::<T, S>(
                parent,
                collection_id,
                document_id,
                return_only_fields,
            )
            .await
        {
            Ok(obj) => Ok(Some(obj)),
            Err(err) => match err {
                FirestoreError::DataNotFoundError(_) => Ok(None),
                _ => Err(err),
            },
        }
    }

    async fn batch_stream_get_docs_at_with_errors<S, I>(
        &self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<FirestoreResult<(String, Option<Document>)>>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        let full_doc_ids: Vec<String> = document_ids
            .into_iter()
            .map(|document_id| safe_document_path(parent, collection_id, document_id.as_ref()))
            .collect::<FirestoreResult<Vec<String>>>()?;

        let span = span!(
            Level::DEBUG,
            "Firestore Batch Get",
            "/firestore/collection_name" = collection_id,
            "/firestore/ids_count" = full_doc_ids.len()
        );

        let request = tonic::Request::new(BatchGetDocumentsRequest {
            database: self.get_database_path().clone(),
            documents: full_doc_ids,
            consistency_selector: self
                .session_params
                .consistency_selector
                .as_ref()
                .map(|selector| selector.try_into())
                .transpose()?,
            mask: return_only_fields.map({
                |vf| gcloud_sdk::google::firestore::v1::DocumentMask {
                    field_paths: vf.iter().map(|f| f.to_string()).collect(),
                }
            }),
        });
        match self.client().get().batch_get_documents(request).await {
            Ok(response) => {
                span.in_scope(|| debug!("Start consuming a batch of documents by ids"));
                let stream = response
                    .into_inner()
                    .filter_map(|r| {
                        future::ready(match r {
                            Ok(doc_response) => doc_response.result.map(|doc_res| match doc_res {
                                batch_get_documents_response::Result::Found(document) => {
                                    let doc_id = document
                                        .name
                                        .split('/')
                                        .last()
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| document.name.clone());
                                    Ok((doc_id, Some(document)))
                                }
                                batch_get_documents_response::Result::Missing(full_doc_id) => {
                                    let doc_id = full_doc_id
                                        .split('/')
                                        .last()
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| full_doc_id);
                                    Ok((doc_id, None))
                                }
                            }),
                            Err(err) => Some(Err(err.into())),
                        })
                    })
                    .boxed();
                Ok(stream)
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn batch_stream_get_docs_at<S, I>(
        &self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<(String, Option<Document>)>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        let doc_stream = self
            .batch_stream_get_docs_at_with_errors(
                parent,
                collection_id,
                document_ids,
                return_only_fields,
            )
            .await?;

        Ok(Box::pin(doc_stream.filter_map(|doc_res| {
            future::ready(match doc_res {
                Ok(doc_pair) => Some(doc_pair),
                Err(err) => {
                    error!(
                        "[DB] Error occurred while consuming batch get as a stream: {}",
                        err
                    );
                    None
                }
            })
        })))
    }

    async fn batch_stream_get_docs<S, I>(
        &self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<(String, Option<Document>)>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        self.batch_stream_get_docs_at(
            self.get_documents_path(),
            collection_id,
            document_ids,
            return_only_fields,
        )
        .await
    }

    async fn batch_stream_get_docs_with_errors<S, I>(
        &self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<FirestoreResult<(String, Option<Document>)>>>
    where
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        self.batch_stream_get_docs_at_with_errors(
            self.get_documents_path(),
            collection_id,
            document_ids,
            return_only_fields,
        )
        .await
    }

    async fn batch_stream_get_objects<'b, T, S, I>(
        &'b self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'b, (String, Option<T>)>>
    where
        for<'de> T: Deserialize<'de> + Send + 'b,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        self.batch_stream_get_objects_at(
            self.get_documents_path(),
            collection_id,
            document_ids,
            return_only_fields,
        )
        .await
    }

    async fn batch_stream_get_objects_at<'a, T, S, I>(
        &'a self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'a, (String, Option<T>)>>
    where
        for<'de> T: Deserialize<'de> + Send + 'a,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        let doc_stream = self
            .batch_stream_get_docs_at(parent, collection_id, document_ids, return_only_fields)
            .await?;

        Ok(Box::pin(doc_stream.filter_map(|(doc_id,maybe_doc)| async move {
            match maybe_doc {
                Some(doc) => {
                    match Self::deserialize_doc_to(&doc) {
                        Ok(obj) => Some((doc_id, Some(obj))),
                        Err(err) => {
                            error!(
                                "[DB] Error occurred while consuming batch documents as a stream: {}",
                                err
                            );
                            None
                        }
                    }
                },
                None => Some((doc_id, None))
            }
        })))
    }

    async fn batch_stream_get_objects_at_with_errors<'a, T, S, I>(
        &'a self,
        parent: &str,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'a, FirestoreResult<(String, Option<T>)>>>
    where
        for<'de> T: Deserialize<'de> + Send + 'a,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        let doc_stream = self
            .batch_stream_get_docs_at_with_errors(
                parent,
                collection_id,
                document_ids,
                return_only_fields,
            )
            .await?;

        Ok(Box::pin(doc_stream.and_then(|(doc_id, maybe_doc)| {
            future::ready({
                maybe_doc
                    .map(|doc| Self::deserialize_doc_to::<T>(&doc))
                    .transpose()
                    .map(|obj| (doc_id, obj))
            })
        })))
    }

    async fn batch_stream_get_objects_with_errors<'a, T, S, I>(
        &'a self,
        collection_id: &str,
        document_ids: I,
        return_only_fields: Option<Vec<String>>,
    ) -> FirestoreResult<BoxStream<'a, FirestoreResult<(String, Option<T>)>>>
    where
        for<'de> T: Deserialize<'de> + Send + 'a,
        S: AsRef<str> + Send,
        I: IntoIterator<Item = S> + Send,
    {
        let doc_stream = self
            .batch_stream_get_docs_at_with_errors(
                self.get_documents_path(),
                collection_id,
                document_ids,
                return_only_fields,
            )
            .await?;

        Ok(Box::pin(doc_stream.and_then(|(doc_id, maybe_doc)| {
            future::ready({
                maybe_doc
                    .map(|doc| Self::deserialize_doc_to::<T>(&doc))
                    .transpose()
                    .map(|obj| (doc_id, obj))
            })
        })))
    }
}

impl FirestoreDb {
    pub(crate) fn get_doc_by_path(
        &self,
        document_path: String,
        return_only_fields: Option<Vec<String>>,
        retries: usize,
    ) -> BoxFuture<FirestoreResult<Document>> {
        async move {
            let begin_query_utc: DateTime<Utc> = Utc::now();

            let request = tonic::Request::new(GetDocumentRequest {
                name: document_path.clone(),
                consistency_selector: self
                    .session_params
                    .consistency_selector
                    .as_ref()
                    .map(|selector| selector.try_into())
                    .transpose()?,
                mask: return_only_fields.map({
                    |vf| gcloud_sdk::google::firestore::v1::DocumentMask {
                        field_paths: vf.iter().map(|f| f.to_string()).collect(),
                    }
                }),
            });

            match self
                .client()
                .get()
                .get_document(request)
                .map_err(|e| e.into())
                .await
            {
                Ok(doc_response) => {
                    let end_query_utc: DateTime<Utc> = Utc::now();
                    let query_duration = end_query_utc.signed_duration_since(begin_query_utc);

                    debug!(
                        "[DB]: Reading document {} took {}ms",
                        document_path,
                        query_duration.num_milliseconds()
                    );

                    Ok(doc_response.into_inner())
                }
                Err(err) => match err {
                    FirestoreError::DatabaseError(ref db_err)
                        if db_err.retry_possible && retries < self.get_options().max_retries =>
                    {
                        warn!(
                            "[DB]: Failed with {}. Retrying: {}/{}",
                            db_err,
                            retries + 1,
                            self.get_options().max_retries
                        );
                        self.get_doc_by_path(document_path, None, retries + 1).await
                    }
                    _ => Err(err),
                },
            }
        }
        .boxed()
    }
}
