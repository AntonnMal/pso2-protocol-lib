#ifndef psopacketlib_ffi_h
#define psopacketlib_ffi_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#define API_VERSION 1

#define PROTOCOL_VERSION 1

/**
 * Packet types.
 */
typedef enum PacketType {
  NGS,
  Classic,
  NA,
  JP,
  Vita,
  Raw,
} PacketType;

/**
 * Serialized packet format
 */
typedef enum SerializedFormat {
  JSON,
  MessagePack,
  MessagePackNamed,
} SerializedFormat;

typedef struct PacketWorker PacketWorker;

/**
 * Fat pointer to data.
 */
typedef struct DataBuffer {
  const uint8_t *ptr;
  size_t size;
} DataBuffer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

uint32_t get_api_version(void);

uint32_t get_protocol_version(void);

/**
 * Creates a new packet worker.
 */
struct PacketWorker *new_worker(enum PacketType packet_type, enum SerializedFormat serde_format);

/**
 * Destroys a packet worker.
 */
void free_worker(struct PacketWorker*);

/**
 * Sets a new packet type.
 */
void set_packet_type(struct PacketWorker *worker, enum PacketType packet_type);

/**
 * Sets a new serde format.
 */
void set_serde_format(struct PacketWorker *worker, enum SerializedFormat format);

/**
 * Checks if the specified serde format is supported.
 */
bool serde_supported(enum SerializedFormat serde_format);

/**
 * Parses packet data and returns a fat pointer to the serialized packet or a null pointer if
 * an error occurred.
 *
 * # Safety
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct DataBuffer parse_packet(struct PacketWorker *worker, const uint8_t *data_ptr, size_t size);

/**
 * Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error
 * occured.
 *
 * # Safety
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct DataBuffer create_packet(struct PacketWorker *worker, const uint8_t *data_ptr, size_t size);

/**
 * Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
 * occurred.
 *
 * # Safety
 * The returned pointer is only valid until the next failable function call.
 */
const uint8_t *get_error(struct PacketWorker *worker);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* psopacketlib_ffi_h */