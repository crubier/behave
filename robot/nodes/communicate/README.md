# Node: Communicate

GCS communication link. Runs a UDP server on port 9000 that receives FlatBuffers serialized ActionRequest messages from the ground control station, validates them, and forwards them to the Behave node over iceoryx2.

## Topics

- Publishes to: `behave/ActionRequest`
