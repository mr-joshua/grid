// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use super::StoreFactory;

/// A `StoryFactory` backed by a PostgreSQL database.
pub struct PgStoreFactory {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgStoreFactory {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl StoreFactory for PgStoreFactory {
    fn get_grid_agent_store(&self) -> Box<dyn crate::grid_db::AgentStore> {
        Box::new(crate::grid_db::DieselAgentStore::new(self.pool.clone()))
    }

    fn get_grid_commit_store(&self) -> Box<dyn crate::grid_db::CommitStore> {
        Box::new(crate::grid_db::DieselCommitStore::new(self.pool.clone()))
    }

    fn get_grid_organization_store(&self) -> Box<dyn crate::grid_db::OrganizationStore> {
        Box::new(crate::grid_db::DieselOrganizationStore::new(
            self.pool.clone(),
        ))
    }

    fn get_grid_location_store(&self) -> Box<dyn crate::grid_db::LocationStore> {
        Box::new(crate::grid_db::DieselLocationStore::new(self.pool.clone()))
    }

    fn get_grid_product_store(&self) -> Box<dyn crate::grid_db::ProductStore> {
        Box::new(crate::grid_db::DieselProductStore::new(self.pool.clone()))
    }

    fn get_grid_schema_store(&self) -> Box<dyn crate::grid_db::SchemaStore> {
        Box::new(crate::grid_db::DieselSchemaStore::new(self.pool.clone()))
    }

    fn get_grid_track_and_trace_store(&self) -> Box<dyn crate::grid_db::TrackAndTraceStore> {
        Box::new(crate::grid_db::DieselTrackAndTraceStore::new(
            self.pool.clone(),
        ))
    }
}
