use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;
use sqlx::Row;

use abi::{convert_to_utc_time, Reservation, ReservationQuery};

use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }
        let start = convert_to_utc_time(rsvp.start.clone().unwrap());
        let end = convert_to_utc_time(rsvp.end.clone().unwrap());
        if start <= end {
            return Err(ReservationError::InvalidTime);
        }
        let timespan: PgRange<DateTime<Utc>> = (start..end).into();
        // generate a insert sql for reservation
        let id: String = sqlx::query("INSERT INTO reservation (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5)")
            .bind(rsvp.user_id.clone())
            .bind(rsvp.resource_id.clone())
            .bind(timespan)
            .bind(rsvp.note.clone())
            .bind(rsvp.status)
            .fetch_one(&self.pool)
            .await?.get(0);
        rsvp.id = id;

        Ok(rsvp)
    }

    async fn change_status(&self, _id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, _id: ReservationId) -> Result<(), ReservationError> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn query(&self, _query: ReservationQuery) -> Result<Vec<Reservation>, ReservationError> {
        todo!()
    }
}
