//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

// Generated by zkgroup/codegen/codegen.py - do not edit

package org.signal.zkgroup.receipts;

import java.nio.ByteBuffer;
import org.signal.zkgroup.InvalidInputException;
import org.signal.zkgroup.ZkGroupError;
import org.signal.zkgroup.internal.ByteArray;
import org.signal.zkgroup.internal.Native;

public final class ReceiptCredentialPresentation extends ByteArray {

  public static final int SIZE = 329;

  public ReceiptCredentialPresentation(byte[] contents) throws InvalidInputException {
    super(contents, SIZE);
    
    int ffi_return = Native.receiptCredentialPresentationCheckValidContentsJNI(contents);

    if (ffi_return == Native.FFI_RETURN_INPUT_ERROR) {
      throw new InvalidInputException("FFI_RETURN_INPUT_ERROR");
    }

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }
  }

  public long getReceiptExpirationTime() {
    byte[] newContents = new byte[8];

    int ffi_return = Native.receiptCredentialPresentationGetReceiptExpirationTimeJNI(contents, newContents);

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }

    return ByteBuffer.wrap(newContents).getLong();
  }

  public long getReceiptLevel() {
    byte[] newContents = new byte[8];

    int ffi_return = Native.receiptCredentialPresentationGetReceiptLevelJNI(contents, newContents);

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }

    return ByteBuffer.wrap(newContents).getLong();
  }

  public ReceiptSerial getReceiptSerial() {
    byte[] newContents = new byte[ReceiptSerial.SIZE];

    int ffi_return = Native.receiptCredentialPresentationGetReceiptSerialJNI(contents, newContents);

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }

    try {
      return new ReceiptSerial(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }

  }

  public byte[] serialize() {
    return contents.clone();
  }

}
