// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

package tcp_test

import (
	"github.com/onsi/ginkgo/v2"
)

var _ = ginkgo.Describe("PING FEATURE:", func() {
	ginkgo.When("User is logged in", func() {
		ginkgo.Context("and tries to ping server", func() {
			client := createAuthorizedConnection()
			err := client.Ping()

			itShouldNotReturnError(err)
		})
	})

	ginkgo.When("User is not logged in", func() {
		ginkgo.Context("and tries to ping server", func() {
			client := createClient()
			err := client.Ping()

			itShouldNotReturnError(err)
		})
	})
})
