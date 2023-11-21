use std::collections::HashMap;

/// Ref in https://github.com/matomo-org/plugin-QueuedTracking/blob/5.x-dev/Queue/Lock.php#L13
pub const QUEUE_LOCK_NAME: &str = "QueuedTrackingLock";

/// Ref in https://github.com/matomo-org/plugin-QueuedTracking/blob/5.x-dev/Queue.php#L17
pub const QUEUE_PREFIX: &str = "trackingQueueV1";

/// Ref in https://github.com/matomo-org/plugin-QueuedTracking/blob/5.x-dev/Queue/Manager.php#L49-L66
pub const QUEUE_KEY_MAP: &[(&str, u8); 16] = &[
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("a", 10),
    ("b", 11),
    ("c", 12),
    ("d", 13),
    ("e", 14),
    ("f", 15),
];

pub fn get_queue_key(uniq_param: impl ToString) -> u8 {
    let first = uniq_param.to_string().as_str()[..1].to_lowercase();
    let map = HashMap::from(*QUEUE_KEY_MAP);
    *map.get(first.as_str()).or(Some(&0)).unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_queue_id() {
        assert_eq!(10, get_queue_key("ajdshflkajhdsflkja"));
        assert_eq!(13, get_queue_key("demo"));
        assert_eq!(2, get_queue_key("231231231231212"));
        assert_eq!(15, get_queue_key("fdlkjafaksdjhfkjads"));
    }
}
