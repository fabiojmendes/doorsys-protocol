---
vim: tw=80
---

# Doorsys Protocol

This repository contains the protocol messages exchanged between the firmware
and the API. I went for a very simple approach here using
[postcard](https://github.com/jamesmunns/postcard) to serialize the messages.
But not much was done in terms of backwards compatibility and schema evolution.
For now things are very simple and stable, but when the time comes perhaps
protobuf could be an option.

Current functionality includes standard ways to:

- Add and remove users
- Update user codes
- Bulk initialize a brand-new system
- Emit audit records
