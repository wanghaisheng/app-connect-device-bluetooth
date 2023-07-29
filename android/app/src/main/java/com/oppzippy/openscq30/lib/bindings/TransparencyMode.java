// Automatically generated by flapigen
package com.oppzippy.openscq30.lib.bindings;


public enum TransparencyMode {
    FullyTransparent(0),
    VocalMode(1);

    private final int value;
    TransparencyMode(int value) {
        this.value = value;
    }
    public final int getValue() { return value; }
    /*package*/ static TransparencyMode fromInt(int x) {
        switch (x) {
            case 0: return FullyTransparent;
            case 1: return VocalMode;
            default: throw new Error("Invalid value for enum TransparencyMode: " + x);
        }
    }
}
