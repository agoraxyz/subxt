// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    RpcClientT,
    RpcFuture,
    RpcSubscription,
};
use crate::error::RpcError;
use futures::stream::{
    StreamExt,
    TryStreamExt,
};
use jsonrpsee::core::{
    client::{
        Client,
        ClientT,
        SubscriptionClientT,
    },
    traits::ToRpcParams,
    Error as JsonRpseeError,
};
use serde_json::value::RawValue;

struct Params(Option<Box<RawValue>>);

impl ToRpcParams for Params {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, JsonRpseeError> {
        Ok(self.0)
    }
}

impl RpcClientT for Client {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RpcFuture<'a, Box<RawValue>> {
        Box::pin(async move {
            let res = ClientT::request(self, method, Params(params))
                .await
                .map_err(|e| RpcError(e.to_string()))?;
            Ok(res)
        })
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription> {
        Box::pin(async move {
            let sub = SubscriptionClientT::subscribe::<Box<RawValue>, _>(
                self,
                sub,
                Params(params),
                unsub,
            )
            .await
            .map_err(|e| RpcError(e.to_string()))?
            .map_err(|e| RpcError(e.to_string()))
            .boxed();
            Ok(sub)
        })
    }
}
