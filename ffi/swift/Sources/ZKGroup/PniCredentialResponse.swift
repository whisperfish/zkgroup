//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//
// Generated by zkgroup/codegen/codegen.py - do not edit

import Foundation
import libzkgroup

public class PniCredentialResponse : ByteArray {

  public static let SIZE: Int = 521

  public init(contents: [UInt8]) throws  {
    try super.init(newContents: contents, expectedLength: PniCredentialResponse.SIZE)

    
    let ffi_return = FFI_PniCredentialResponse_checkValidContents(self.contents, UInt32(self.contents.count))

    if (ffi_return == Native.FFI_RETURN_INPUT_ERROR) {
      throw ZkGroupException.InvalidInput
    }

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw ZkGroupException.ZkGroupError
    }
  }

  public func serialize() -> [UInt8] {
    return contents
  }

}
