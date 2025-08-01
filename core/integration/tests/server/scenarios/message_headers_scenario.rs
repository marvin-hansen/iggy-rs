/* Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use crate::server::scenarios::{
    MESSAGES_COUNT, PARTITION_ID, PARTITIONS_COUNT, STREAM_ID, STREAM_NAME, TOPIC_ID, TOPIC_NAME,
    cleanup, create_client,
};
use bytes::Bytes;
use iggy::prelude::*;
use integration::test_server::{ClientFactory, assert_clean_system, login_root};
use std::collections::HashMap;
use std::str::FromStr;

pub async fn run(client_factory: &dyn ClientFactory) {
    let client = create_client(client_factory).await;
    login_root(&client).await;
    init_system(&client).await;

    // 1. Send messages with the included headers
    let mut messages = Vec::new();
    for offset in 0..MESSAGES_COUNT {
        let id = (offset + 1) as u128;
        let payload = create_message_payload(offset as u64);
        let headers = create_message_headers();
        messages.push(
            IggyMessage::builder()
                .id(id)
                .payload(payload)
                .user_headers(headers)
                .build()
                .expect("Failed to create message with headers"),
        );
    }

    client
        .send_messages(
            &Identifier::numeric(STREAM_ID).unwrap(),
            &Identifier::numeric(TOPIC_ID).unwrap(),
            &Partitioning::partition_id(PARTITION_ID),
            &mut messages,
        )
        .await
        .unwrap();

    // 2. Poll messages and validate the headers
    let consumer = Consumer::default();
    let polled_messages = client
        .poll_messages(
            &Identifier::numeric(STREAM_ID).unwrap(),
            &Identifier::numeric(TOPIC_ID).unwrap(),
            Some(PARTITION_ID),
            &consumer,
            &PollingStrategy::offset(0),
            MESSAGES_COUNT,
            false,
        )
        .await
        .unwrap();

    assert_eq!(polled_messages.messages.len() as u32, MESSAGES_COUNT);
    for i in 0..MESSAGES_COUNT {
        let message = polled_messages.messages.get(i as usize).unwrap();
        assert!(message.user_headers.is_some());
        let headers = message.user_headers_map().unwrap().unwrap();
        assert_eq!(headers.len(), 3);
        assert_eq!(
            headers
                .get(&HeaderKey::new("key_1").unwrap())
                .unwrap()
                .as_str()
                .unwrap(),
            "Value 1"
        );
        assert!(
            headers
                .get(&HeaderKey::new("key 2").unwrap())
                .unwrap()
                .as_bool()
                .unwrap(),
        );
        assert_eq!(
            headers
                .get(&HeaderKey::new("key-3").unwrap())
                .unwrap()
                .as_uint64()
                .unwrap(),
            123456
        );
    }
    cleanup(&client, false).await;
    assert_clean_system(&client).await;
}

async fn init_system(client: &IggyClient) {
    // 1. Create the stream
    client
        .create_stream(STREAM_NAME, Some(STREAM_ID))
        .await
        .unwrap();

    // 2. Create the topic
    client
        .create_topic(
            &Identifier::numeric(STREAM_ID).unwrap(),
            TOPIC_NAME,
            PARTITIONS_COUNT,
            CompressionAlgorithm::default(),
            None,
            Some(TOPIC_ID),
            IggyExpiry::NeverExpire,
            MaxTopicSize::ServerDefault,
        )
        .await
        .unwrap();
}

fn create_message_payload(offset: u64) -> Bytes {
    Bytes::from(format!("message {offset}"))
}

fn create_message_headers() -> HashMap<HeaderKey, HeaderValue> {
    let mut headers = HashMap::new();
    headers.insert(
        HeaderKey::new("key_1").unwrap(),
        HeaderValue::from_str("Value 1").unwrap(),
    );
    headers.insert(
        HeaderKey::new("key 2").unwrap(),
        HeaderValue::from_bool(true).unwrap(),
    );
    headers.insert(
        HeaderKey::new("key-3").unwrap(),
        HeaderValue::from_uint64(123456).unwrap(),
    );
    headers
}
