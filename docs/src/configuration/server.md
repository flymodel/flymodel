# Server

## Keys

### `server.temp_dir`

The temporary directory used for out of memory file staging.
Required.

### `server.tls.ca_source`

The ca source used to validate mTLS requests. (future-state).

### `server.tls.certs`

If a string, it is assumed that the certificate and key are clubbed together in the respective order. \*
Else provide the below.

### `server.tls.certs.cert_file`

The cert file used for https. Optional if tls not specified. Required if specified.

### `server.tls.certs.key_file`

The pkcs8 key used for https. Optional if tls not specified. Required if specified.

## Example

```toml
[server.tls.certs]
cert_file = "./certs/my-domain.pem"
key_file = "./certs/my-domain.key"
```

## Notes

- **mTLS is not implemented**
- **Client Certificate validation is not implemented**
- **Certificate clubbing is not implemented.**
