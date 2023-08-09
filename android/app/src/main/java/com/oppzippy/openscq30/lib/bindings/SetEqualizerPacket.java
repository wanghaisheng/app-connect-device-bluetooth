// Automatically generated by flapigen
package com.oppzippy.openscq30.lib.bindings;
import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

public final class SetEqualizerPacket {

    public SetEqualizerPacket(@NonNull EqualizerConfiguration left_configuration, @Nullable EqualizerConfiguration right_configuration) {
        long a0 = left_configuration.mNativeObj;
        long a1 = 0;//TODO: use ptr::null() for corresponding constant
        if (right_configuration != null) {
            a1 = right_configuration.mNativeObj;
        }

        mNativeObj = init(a0, a1);
        JNIReachabilityFence.reachabilityFence2(left_configuration, right_configuration);
    }
    private static native long init(long left_configuration, long right_configuration);

    public final byte [] bytes() {
        byte [] ret = do_bytes(mNativeObj);

        return ret;
    }
    private static native byte [] do_bytes(long self);

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
    /*package*/ SetEqualizerPacket(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}