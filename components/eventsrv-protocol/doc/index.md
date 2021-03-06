# Protocol Documentation
<a name="top"/>

## Table of Contents
* [event.proto](#event.proto)
 * [CensusEntry](#habitat.eventsrv.CensusEntry)
 * [EventEnvelope](#habitat.eventsrv.EventEnvelope)
 * [PackageIdent](#habitat.eventsrv.PackageIdent)
 * [SysInfo](#habitat.eventsrv.SysInfo)
 * [EventEnvelope.Type](#habitat.eventsrv.EventEnvelope.Type)
* [Scalar Value Types](#scalar-value-types)

<a name="event.proto"/>
<p align="right"><a href="#top">Top</a></p>

## event.proto

EventSrv related messages

<a name="habitat.eventsrv.CensusEntry"/>
### CensusEntry
Generated by gossip information from Supervisors representing a single member of the Census.

| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| member_id | [string](#string) | optional | Member-ID of originating Supervisor |
| service | [string](#string) | optional | Service name |
| group | [string](#string) | optional | Service group name |
| org | [string](#string) | optional | Service group organization name |
| cfg | [bytes](#bytes) | optional | Gossiped configuration of service |
| sys | [SysInfo](#habitat.eventsrv.SysInfo) | optional | System information of Supervisor |
| pkg | [PackageIdent](#habitat.eventsrv.PackageIdent) | optional | Package information of service |
| leader | [bool](#bool) | optional | `true` if this service instance is the leader when in a leader topology |
| follower | [bool](#bool) | optional | `true` if this service instance is a follower when in a leader topology |
| update_leader | [bool](#bool) | optional | `true` if this service instance is the update leader when in a coordinated update topology |
| update_follower | [bool](#bool) | optional | `true` if this service instance is an update leader when in a coordinated update topology |
| election_is_running | [bool](#bool) | optional | `true` if this service instance is part of a topology and an election is currently under way |
| election_is_no_quorum | [bool](#bool) | optional | `true` if this service instance is part of a topology and an election is currently under way/ but has come to a stop because a quorum cannot be met |
| election_is_finished | [bool](#bool) | optional | `true` if this service instance is part of a topology and an election is finished |
| update_election_is_running | [bool](#bool) | optional | `true` if this service instance is part of an update topology and an election is currently/ under way |
| update_election_is_no_quorum | [bool](#bool) | optional | `true` if this service instance is part of an update topology and an election is currently/ under way but has come to a stop because a quorum cannot be met |
| update_election_is_finished | [bool](#bool) | optional | `true` if this service instance is part of an update topology and an election is finished |
| initialized | [bool](#bool) | optional | `true` if the service has successfully initialized |
| alive | [bool](#bool) | optional | `true` if the service is alive |
| suspect | [bool](#bool) | optional | `true` if the service is suspected to be dead |
| confirmed | [bool](#bool) | optional | `true` if the service is confirmed to be dead |
| persistent | [bool](#bool) | optional | `true` if the originating Supervisor is a permanent peer |
| departed | [bool](#bool) | optional | `true` if the service is marked as departed |


<a name="habitat.eventsrv.EventEnvelope"/>
### EventEnvelope
The base for all messages generated by an EventSrv. This message contains framing
/ to hint to a consumer how to encode/decode the message's payload and information for
/ how to route or index the message.

| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [EventEnvelope.Type](#habitat.eventsrv.EventEnvelope.Type) | optional | Message payload hint to a decoder |
| payload | [bytes](#bytes) | optional | Contents of message |
| timestamp | [uint64](#uint64) | optional | Time of message origination |
| member_id | [string](#string) | optional | Member-ID of originating Supervisor |
| service | [string](#string) | optional | Service name of originating Supervisor service |
| incarnation | [uint64](#uint64) | optional | Supervisor's incarnation at message origination |
| sequence_id | [uint64](#uint64) | optional | Message's sequence ID |


<a name="habitat.eventsrv.PackageIdent"/>
### PackageIdent
Information describing the package a service is running.

| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| origin | [string](#string) | optional | Origin name of package |
| name | [string](#string) | optional | Software name of package |
| version | [string](#string) | optional | Software version of package |
| release | [string](#string) | optional | Build release timestamp of package |


<a name="habitat.eventsrv.SysInfo"/>
### SysInfo
System information generated by the Supervisor of the machine it is running on.

| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ip | [string](#string) | optional | Public facing IP address of Supervisor |
| hostname | [string](#string) | optional | Network hostname of Supervisor |
| gossip_ip | [string](#string) | optional | Listening address for Supervisor's gossip connection |
| gossip_port | [string](#string) | optional | Listening port for Supervisor's gossip connection |
| http_gateway_ip | [string](#string) | optional | Listening address for Supervisor's http gateway |
| http_gateway_port | [string](#string) | optional | Listening port for Supervisor's http gateway |



<a name="habitat.eventsrv.EventEnvelope.Type"/>
### EventEnvelope.Type
Enumerator of potential encoding types for the Envelope's payload

| Name | Number | Description |
| ---- | ------ | ----------- |
| ProtoBuf | 1 | Encoded with a Google Protobuf |
| JSON | 2 | Encoded with JSON |
| TOML | 3 | Encoded with TOML |





<a name="scalar-value-types"/>
## Scalar Value Types

| .proto Type | Notes | C++ Type | Java Type | Python Type |
| ----------- | ----- | -------- | --------- | ----------- |
| <a name="double"/> double |  | double | double | float |
| <a name="float"/> float |  | float | float | float |
| <a name="int32"/> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int |
| <a name="int64"/> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long |
| <a name="uint32"/> uint32 | Uses variable-length encoding. | uint32 | int | int/long |
| <a name="uint64"/> uint64 | Uses variable-length encoding. | uint64 | long | int/long |
| <a name="sint32"/> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int |
| <a name="sint64"/> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long |
| <a name="fixed32"/> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int |
| <a name="fixed64"/> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long |
| <a name="sfixed32"/> sfixed32 | Always four bytes. | int32 | int | int |
| <a name="sfixed64"/> sfixed64 | Always eight bytes. | int64 | long | int/long |
| <a name="bool"/> bool |  | bool | boolean | boolean |
| <a name="string"/> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode |
| <a name="bytes"/> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str |
