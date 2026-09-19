#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use super::super::domain::{session_expiry, Entitlement, LabSession, ProviderKind, SessionStatus};
    #[test] fn entitlement_expires_after_thirty_days() { let now = Utc::now(); let e = Entitlement { id:uuid::Uuid::new_v4(), user_id:uuid::Uuid::new_v4(), lab_id:"aws-s3-101".into(), starts_at:now, expires_at:now + Duration::days(30) }; assert!(e.active(now + Duration::days(29))); assert!(!e.active(now + Duration::days(30))); }
    #[test] fn session_is_forty_five_minutes() { let now = Utc::now(); assert_eq!(session_expiry(now) - now, Duration::minutes(45)); }
    #[test] fn expired_session_is_detected_server_side() { let now = Utc::now(); let s = LabSession { id:uuid::Uuid::new_v4(), user_id:uuid::Uuid::new_v4(), lab_id:"x".into(), provider:ProviderKind::Aws, environment_id:None, status:SessionStatus::Running, started_at:now - Duration::minutes(46), expires_at:now - Duration::minutes(1), completed_at:None, score:0 }; assert!(s.expired(now)); }
}
