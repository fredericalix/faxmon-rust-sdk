# Faxmon Backend SDK Documentation (for Probes/Agents)

This document provides guidance for developing external probes or agents that interact with the Faxmon backend API to report event statuses.

## 1. Authentication

All API requests must be authenticated using a **Bearer Token** provided in the `Authorization` header.

```http
Authorization: Bearer <YOUR_PROBE_API_TOKEN>
```

### Obtaining a Probe Token

Tokens specifically for probes (`TokenTypeProbe`) must be generated by a user with Administrator privileges using the Faxmon backend API:

*   **Endpoint:** `POST /admin/tokens/probe`
*   **Authentication:** Requires an `Admin` or `SuperAdmin` token.
*   **Request Body:** 
    ```json
    {
      "description": "Token for My Custom Probe"
    }
    ```
*   **Response:** Contains the generated token value.

**Important:** Securely store and manage this token within your probe/agent configuration. Do not expose it unnecessarily.

## 2. Reporting Event Status

The primary function of a probe is to update the status of a specific event it monitors.

*   **Endpoint:** `POST /events/{eventId}/update`
*   **Method:** `POST`
*   **Authentication:** Requires a `Probe` token (as described above).

### Path Parameters

*   `eventId` (string, required): The unique identifier of the event whose status is being updated. This ID is typically obtained when the event is initially created in Faxmon.

### Request Body

The request body must be a JSON object conforming to the `EventStatusUpdatePayload` structure:

```json
{
  "status": "ok",
  "message": "System is operating normally.",
  "acknowledged": false,
  "timestamp": "2025-05-04T10:30:00Z",
  "notification_listener_id": "optional-listener-id"
}
```

**Fields:**

*   `status` (string, **required**): The current status of the event. Must be one of:
    *   `"ok"`
    *   `"warning"`
    *   `"critical"`
    *   `"unknown"`
*   `message` (string, *optional*): A descriptive message associated with the current status. Defaults to empty if omitted.
*   `acknowledged` (boolean, *optional*): Indicates if the current status should be considered acknowledged. Defaults to `false` if omitted. Usually managed via the UI or rules, but can be set by probes if needed.
*   `timestamp` (string, *optional*, ISO 8601 format): The timestamp when the status was recorded. If omitted, the backend uses the time the request was received.
*   `notification_listener_id` (string, *optional*): Specifies a particular notification listener to use for this update. Typically, this is determined by the event's configuration or triggered by rules, so it's often omitted in probe updates.

### Example Request (using curl)

```bash
curl -X POST \
  http://<FAXMON_HOST>:<PORT>/events/b0e42fe7-31a5-4f89-9d89-cc3b690219b9/update \
  -H 'Authorization: Bearer <YOUR_PROBE_API_TOKEN>' \
  -H 'Content-Type: application/json' \
  -d '{
        "status": "critical",
        "message": "Service unreachable."
      }'
```

### Success Response

*   **Status Code:** `200 OK`
*   **Body:** A JSON object representing the updated consolidated state of the event (`models.EventConsolidatedState`).

    ```json
    {
        "id": "b0e42fe7-31a5-4f89-9d89-cc3b690219b9",
        "name": "My Critical Service Check",
        "type": "service",
        "designation": "critical",
        "status": "critical",
        "acknowledged": false,
        "last_updated": "2025-05-04T08:58:33Z", 
        "message": "Service unreachable.",
        "notification_listener_id": "f7e42fe7-31a5-4f89-9d89-cc3b690219b9"
    }
    ```

### Error Responses

Common error status codes include:

*   `400 Bad Request`: Invalid request body (e.g., missing `status`, invalid `status` value).
*   `401 Unauthorized`: Invalid or missing API token.
*   `403 Forbidden`: Token provided does not have `Probe` (or higher) privileges.
*   `404 Not Found`: The specified `eventId` does not exist.
*   `500 Internal Server Error`: An unexpected error occurred on the backend.

The error response body typically contains a JSON object with an `error` field describing the issue:

```json
{
  "error": "Invalid status value. Must be one of: ok, warning, critical, unknown"
}
```
