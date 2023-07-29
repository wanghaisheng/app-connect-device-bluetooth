// Automatically generated by flapigen
package com.oppzippy.openscq30.lib.bindings;
import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

public final class StateUpdatePacket {

    public StateUpdatePacket() throws Exception {
        mNativeObj = init();
    }
    private static native long init() throws Exception;

    public final int featureFlags() {
        int ret = do_featureFlags(mNativeObj);

        return ret;
    }
    private static native int do_featureFlags(long self);

    public final @NonNull java.util.Optional<SoundModes> soundModes() {
        long ret = do_soundModes(mNativeObj);
        java.util.Optional<SoundModes> convRet;
        if (ret != 0) {
            convRet = java.util.Optional.of(new SoundModes(InternalPointerMarker.RAW_PTR, ret));
        } else {
            convRet = java.util.Optional.empty();
        }

        return convRet;
    }
    private static native long do_soundModes(long self);

    public final @NonNull EqualizerConfiguration equalizerConfiguration() {
        long ret = do_equalizerConfiguration(mNativeObj);
        EqualizerConfiguration convRet = new EqualizerConfiguration(InternalPointerMarker.RAW_PTR, ret);

        return convRet;
    }
    private static native long do_equalizerConfiguration(long self);

    public final @NonNull java.util.Optional<String> firmwareVersion() {
        String ret = do_firmwareVersion(mNativeObj);
        java.util.Optional<String> convRet = java.util.Optional.ofNullable(ret);

        return convRet;
    }
    private static native @Nullable String do_firmwareVersion(long self);

    public final @NonNull java.util.Optional<String> serialNumber() {
        String ret = do_serialNumber(mNativeObj);
        java.util.Optional<String> convRet = java.util.Optional.ofNullable(ret);

        return convRet;
    }
    private static native @Nullable String do_serialNumber(long self);

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
    /*package*/ StateUpdatePacket(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}