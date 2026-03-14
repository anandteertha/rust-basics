pub fn sample_input() -> String {
    String::from(
        "database primary_db connected
api payments_api degraded
cache redis_cache connected
database analytics_db disconnected
api auth_api connected
cache session_cache degraded",
    )
}
