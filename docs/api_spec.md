# Bosonnet API Specification

This document provides an overview of the API endpoints available in Bosonnet for interacting with its three main layers: **User Interface Layer**, **Bitcoin Core Layer**, and **AI-driven Bitmap Layer**.

## API Overview

Bosonnet’s API allows external applications and users to interact with digital assets on the network, manage transactions, and access parcel states within the Bitmap ecosystem. Each layer exposes specific functions through REST-like APIs to maintain modularity and ensure security.

## 1. User Interface Layer API

### Endpoint: `/api/ui/interact`
- **Description**: Handles user commands and interactions, including asset queries and commands to manage digital parcels.
- **Method**: POST
- **Request Parameters**:
  - `command` (string, required): The command or action requested by the user (e.g., "view_parcel", "transfer_asset").
  - `parameters` (object, optional): Additional parameters required for specific commands.
- **Response**:
  - **200 OK**: `{ "status": "success", "data": { "result": "Your command result here" } }`
  - **400 Bad Request**: `{ "status": "error", "message": "Invalid command or parameters" }`

### Endpoint: `/api/ui/status`
- **Description**: Retrieves the current status of the User Interface Layer, useful for debugging or checking system availability.
- **Method**: GET
- **Response**:
  - **200 OK**: `{ "status": "online", "uptime": "3 days" }`
  - **500 Internal Server Error**: `{ "status": "error", "message": "UI layer not available" }`

## 2. Bitcoin Core Layer API

### Endpoint: `/api/bitcoin/validate`
- **Description**: Validates a Bitcoin transaction by checking its alignment with Bitcoin's consensus rules.
- **Method**: POST
- **Request Parameters**:
  - `transaction_id` (string, required): The ID of the Bitcoin transaction to validate.
- **Response**:
  - **200 OK**: `{ "status": "valid", "transaction_id": "abc123", "details": { "confirmations": 6 } }`
  - **400 Bad Request**: `{ "status": "error", "message": "Invalid transaction ID" }`
  - **404 Not Found**: `{ "status": "error", "message": "Transaction not found" }`

### Endpoint: `/api/bitcoin/utxo`
- **Description**: Retrieves UTXO details for a given Bitcoin address.
- **Method**: GET
- **Request Parameters**:
  - `address` (string, required): The Bitcoin address to query for UTXOs.
- **Response**:
  - **200 OK**: `{ "status": "success", "utxos": [{ "txid": "abc123", "amount": 0.5 }] }`
  - **400 Bad Request**: `{ "status": "error", "message": "Invalid address" }`

## 3. AI-driven Bitmap Layer API

### Endpoint: `/api/bitmap/parcel`
- **Description**: Fetches the current state of a specified parcel within the Bitmap ecosystem.
- **Method**: GET
- **Request Parameters**:
  - `parcel_id` (string, required): The unique identifier for the parcel.
- **Response**:
  - **200 OK**: `{ "status": "active", "parcel_id": "parcel123", "owner": "user123", "state": "owned" }`
  - **404 Not Found**: `{ "status": "error", "message": "Parcel not found" }`

### Endpoint: `/api/bitmap/transfer`
- **Description**: Initiates a transfer of parcel ownership from one user to another.
- **Method**: POST
- **Request Parameters**:
  - `parcel_id` (string, required): The ID of the parcel to transfer.
  - `from_owner` (string, required): The current owner's user ID.
  - `to_owner` (string, required): The new owner's user ID.
  - `auth_token` (string, required): Authentication token to authorize the transfer.
- **Response**:
  - **200 OK**: `{ "status": "success", "message": "Transfer complete", "parcel_id": "parcel123" }`
  - **401 Unauthorized**: `{ "status": "error", "message": "Invalid authentication token" }`
  - **404 Not Found**: `{ "status": "error", "message": "Parcel or owner not found" }`

### Endpoint: `/api/bitmap/ai_decision`
- **Description**: Queries the AI-driven Bitmap Layer for its current decision or action status for a specific parcel.
- **Method**: GET
- **Request Parameters**:
  - `parcel_id` (string, required): The parcel in question.
- **Response**:
  - **200 OK**: `{ "status": "active", "action": "wait_for_confirmation", "parcel_id": "parcel123" }`
  - **404 Not Found**: `{ "status": "error", "message": "Parcel not found" }`

## Authentication and Security

Some endpoints require an `auth_token` parameter for security and authorization purposes. This token should be obtained via a secure authentication method and passed in the request headers or parameters where specified.

- **Example Header with Auth Token**:

- 
## Error Handling

Each API endpoint returns a standardized error response with an HTTP status code and message:
- **400 Bad Request**: Invalid parameters or request format.
- **401 Unauthorized**: Missing or invalid authentication token.
- **404 Not Found**: Resource does not exist.
- **500 Internal Server Error**: System errors or failures.

## Notes and Future Expansion

- **New Endpoints**: Additional endpoints may be added in future releases to extend functionality.
- **Rate Limiting**: Consider implementing rate limiting for high-traffic endpoints to ensure stable performance.

---

This API specification provides the foundational information needed to interact with Bosonnet’s layers programmatically. For further clarification, consult the development team or review the source code for each layer.

