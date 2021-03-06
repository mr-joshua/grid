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

use super::LocationStoreOperations;
use crate::grid_db::locations::store::diesel::{
    schema::{location, location_attribute},
    LocationStoreError,
};

use crate::grid_db::commits::MAX_COMMIT_NUM;
use crate::grid_db::locations::store::diesel::models::{
    LocationModel, NewLocationAttributeModel, NewLocationModel,
};
use diesel::{
    dsl::{insert_into, update},
    prelude::*,
    result::Error::NotFound,
};

pub(in crate::grid_db::locations::store::diesel) trait LocationStoreUpdateLocationOperation {
    fn update_location(
        &self,
        location: NewLocationModel,
        attributes: Vec<NewLocationAttributeModel>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError>;
}

#[cfg(feature = "postgres")]
impl<'a> LocationStoreUpdateLocationOperation
    for LocationStoreOperations<'a, diesel::pg::PgConnection>
{
    fn update_location(
        &self,
        location: NewLocationModel,
        attributes: Vec<NewLocationAttributeModel>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        self.conn
            .build_transaction()
            .read_write()
            .run::<_, LocationStoreError, _>(|| {
                let loc = location::table
                    .filter(
                        location::location_id
                            .eq(&location.location_id)
                            .and(location::service_id.eq(&location.service_id)),
                    )
                    .first::<LocationModel>(self.conn)
                    .map(Some)
                    .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
                    .map_err(|err| LocationStoreError::QueryError {
                        context: "Failed check for existing location".to_string(),
                        source: Box::new(err),
                    })?;

                if loc.is_some() {
                    update(location::table)
                        .filter(
                            location::location_id
                                .eq(&location.location_id)
                                .and(location::service_id.eq(&location.service_id))
                                .and(location::end_commit_num.eq(MAX_COMMIT_NUM)),
                        )
                        .set(location::end_commit_num.eq(current_commit_num))
                        .execute(self.conn)
                        .map(|_| ())
                        .map_err(|err| LocationStoreError::OperationError {
                            context: "Failed to update location".to_string(),
                            source: Some(Box::new(err)),
                        })?;
                }

                insert_into(location::table)
                    .values(&location)
                    .execute(self.conn)
                    .map(|_| ())
                    .map_err(|err| LocationStoreError::OperationError {
                        context: "Failed to add location".to_string(),
                        source: Some(Box::new(err)),
                    })?;

                update(location_attribute::table)
                    .filter(
                        location_attribute::location_id
                            .eq(&location.location_id)
                            .and(location_attribute::service_id.eq(&location.service_id))
                            .and(location_attribute::end_commit_num.eq(MAX_COMMIT_NUM)),
                    )
                    .set(location_attribute::end_commit_num.eq(current_commit_num))
                    .execute(self.conn)
                    .map(|_| ())
                    .map_err(|err| LocationStoreError::OperationError {
                        context: "Failed to update location".to_string(),
                        source: Some(Box::new(err)),
                    })?;

                for attribute in attributes {
                    insert_into(location_attribute::table)
                        .values(&attribute)
                        .execute(self.conn)
                        .map(|_| ())
                        .map_err(|err| LocationStoreError::OperationError {
                            context: "Failed to add location attribute".to_string(),
                            source: Some(Box::new(err)),
                        })?;
                }

                Ok(())
            })
    }
}

#[cfg(feature = "sqlite")]
impl<'a> LocationStoreUpdateLocationOperation
    for LocationStoreOperations<'a, diesel::sqlite::SqliteConnection>
{
    fn update_location(
        &self,
        location: NewLocationModel,
        attributes: Vec<NewLocationAttributeModel>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        self.conn
            .immediate_transaction::<_, LocationStoreError, _>(|| {
                let loc = location::table
                    .filter(
                        location::location_id
                            .eq(&location.location_id)
                            .and(location::service_id.eq(&location.service_id)),
                    )
                    .first::<LocationModel>(self.conn)
                    .map(Some)
                    .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
                    .map_err(|err| LocationStoreError::QueryError {
                        context: "Failed check for existing location".to_string(),
                        source: Box::new(err),
                    })?;

                if loc.is_some() {
                    update(location::table)
                        .filter(
                            location::location_id
                                .eq(&location.location_id)
                                .and(location::service_id.eq(&location.service_id))
                                .and(location::end_commit_num.eq(MAX_COMMIT_NUM)),
                        )
                        .set(location::end_commit_num.eq(current_commit_num))
                        .execute(self.conn)
                        .map(|_| ())
                        .map_err(|err| LocationStoreError::OperationError {
                            context: "Failed to update location".to_string(),
                            source: Some(Box::new(err)),
                        })?;
                }

                insert_into(location::table)
                    .values(&location)
                    .execute(self.conn)
                    .map(|_| ())
                    .map_err(|err| LocationStoreError::OperationError {
                        context: "Failed to add location".to_string(),
                        source: Some(Box::new(err)),
                    })?;

                update(location_attribute::table)
                    .filter(
                        location_attribute::location_id
                            .eq(&location.location_id)
                            .and(location_attribute::service_id.eq(&location.service_id))
                            .and(location_attribute::end_commit_num.eq(MAX_COMMIT_NUM)),
                    )
                    .set(location_attribute::end_commit_num.eq(current_commit_num))
                    .execute(self.conn)
                    .map(|_| ())
                    .map_err(|err| LocationStoreError::OperationError {
                        context: "Failed to update location".to_string(),
                        source: Some(Box::new(err)),
                    })?;

                for attribute in attributes {
                    insert_into(location_attribute::table)
                        .values(&attribute)
                        .execute(self.conn)
                        .map(|_| ())
                        .map_err(|err| LocationStoreError::OperationError {
                            context: "Failed to add location attribute".to_string(),
                            source: Some(Box::new(err)),
                        })?;
                }

                Ok(())
            })
    }
}
