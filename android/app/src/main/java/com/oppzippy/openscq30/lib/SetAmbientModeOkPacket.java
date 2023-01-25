// Automatically generated by flapigen
package com.oppzippy.openscq30.lib;
import androidx.annotation.NonNull;

public final class SetAmbientModeOkPacket {

    public SetAmbientModeOkPacket() throws Exception {
        mNativeObj = init();
    }
    private static native long init() throws Exception;

    public static @NonNull java.util.Optional<SetAmbientModeOkPacket> fromBytes(@NonNull byte [] bytes) throws Exception {
        long ret = do_fromBytes(bytes);
        java.util.Optional<SetAmbientModeOkPacket> convRet;
        if (ret != 0) {
            convRet = java.util.Optional.of(new SetAmbientModeOkPacket(InternalPointerMarker.RAW_PTR, ret));
        } else {
            convRet = java.util.Optional.empty();
        }

        return convRet;
    }
    private static native long do_fromBytes(byte [] bytes) throws Exception;

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ SetAmbientModeOkPacket(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}