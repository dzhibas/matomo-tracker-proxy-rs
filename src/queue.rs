use std::collections::HashMap;

pub const QUEUE_LOCK_NAME: &str = "QueuedTrackingLock";
pub const QUEUE_PREFIX: &str = "trackingQueueV1";
pub const QUEUE_KEY_MAP: &[(&str, u8)] = &[
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

pub fn get_queue_id(uniq_param: impl ToString) -> u8 {
    let first = uniq_param.to_string().as_str()[..1].to_lowercase();
    let m:HashMap<&str, u8> = HashMap::from(QUEUE_KEY_MAP);
    *m.get(first.as_str()).or(Some(&0)).unwrap_or(&0)
}

mod tests {
    use crate::queue::get_queue_id;
    #[test]
    fn test_get_queue_id() {
        let input = "demoemdoe";
        assert_eq!(0, get_queue_id(input));
    }
}
