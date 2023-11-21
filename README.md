# Matomo Tracker Proxy in Rust

This script allows to track websites with Matomo without revealing the Matomo server URL

Main feature is that this lightweight, it supports Queue plugin and drops into queue compressed messages
which official queue plugin understands. It aswel splits it into amount of workers and can schedule processing
once queue size reached

This allows massive amount of traffic ingestion and slower processing

Works with Matomo plugin: https://github.com/matomo-org/plugin-QueuedTracking