//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//
// Generated by zkgroup/codegen/codegen.py - do not edit

import Foundation
import libzkgroup

public class ReceiptCredential : ByteArray {

  public static let SIZE: Int = 129

  public init(contents: [UInt8]) throws  {
    try super.init(newContents: contents, expectedLength: ReceiptCredential.SIZE)

    
    let ffi_return = FFI_ReceiptCredential_checkValidContents(self.contents, UInt32(self.contents.count))

    if (ffi_return == Native.FFI_RETURN_INPUT_ERROR) {
      throw ZkGroupException.InvalidInput
    }

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw ZkGroupException.ZkGroupError
    }
  }

  public func getReceiptExpirationTime() throws  -> UInt64 {
    var newContents: [UInt8] = Array(repeating: 0, count: Int(8))

    let ffi_return = FFI_ReceiptCredential_getReceiptExpirationTime(self.contents, UInt32(self.contents.count), &newContents, UInt32(newContents.count))

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw ZkGroupException.ZkGroupError
     }

    let data = Data(bytes: newContents)
    let value = UInt64(bigEndian: data.withUnsafeBytes { $0.pointee })
    return value
  }

  public func getReceiptLevel() throws  -> UInt64 {
    var newContents: [UInt8] = Array(repeating: 0, count: Int(8))

    let ffi_return = FFI_ReceiptCredential_getReceiptLevel(self.contents, UInt32(self.contents.count), &newContents, UInt32(newContents.count))

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw ZkGroupException.ZkGroupError
     }

    let data = Data(bytes: newContents)
    let value = UInt64(bigEndian: data.withUnsafeBytes { $0.pointee })
    return value
  }

  public func serialize() -> [UInt8] {
    return contents
  }

}
