// JVM binding for Uldren Loom via the Foreign Function & Memory API (JDK 22+).
// Licensed under BUSL-1.1 (see the repo LICENSE). (c) Uldren Technologies LLC.
package ai.uldren.loom;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;

/** Thin FFM wrapper over the Uldren Loom C ABI (libuldren_loom). */
public final class Loom {
    private static final Linker LINKER = Linker.nativeLinker();
    private static final SymbolLookup LOOKUP = loadLibrary();

    private static final MethodHandle LOOM_VERSION = LINKER.downcallHandle(
            LOOKUP.find("loom_version").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS));

    private static final MethodHandle LOOM_BLOB_DIGEST = LINKER.downcallHandle(
            LOOKUP.find("loom_blob_digest").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));

    private static final MethodHandle LOOM_STRING_FREE = LINKER.downcallHandle(
            LOOKUP.find("loom_string_free").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

    private Loom() {}

    private static SymbolLookup loadLibrary() {
        // For local development, set java.library.path or LD_LIBRARY_PATH to the cargo target dir;
        // packaged releases extract the per-platform native library at load.
        String name = System.mapLibraryName("uldren_loom"); // e.g. libuldren_loom.so
        return SymbolLookup.libraryLookup(name, Arena.global());
    }

    /** Library version, e.g. "0.0.0". */
    public static String version() {
        try {
            MemorySegment ptr = (MemorySegment) LOOM_VERSION.invokeExact();
            return takeOwnedString(ptr);
        } catch (Throwable t) {
            throw new RuntimeException("loom_version failed", t);
        }
    }

    /** Blob content address ("algo:hex") of the given bytes. */
    public static String blobDigest(byte[] data) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment buf = arena.allocate(Math.max(data.length, 1));
            MemorySegment.copy(data, 0, buf, ValueLayout.JAVA_BYTE, 0, data.length);
            MemorySegment ptr = (MemorySegment) LOOM_BLOB_DIGEST.invokeExact(buf, (long) data.length);
            return takeOwnedString(ptr);
        } catch (Throwable t) {
            throw new RuntimeException("loom_blob_digest failed", t);
        }
    }

    /** Read a C string returned by the library, then free it (the library owns returned pointers). */
    private static String takeOwnedString(MemorySegment ptr) throws Throwable {
        if (ptr.equals(MemorySegment.NULL)) {
            return null;
        }
        String s = ptr.reinterpret(Long.MAX_VALUE).getString(0);
        LOOM_STRING_FREE.invokeExact(ptr);
        return s;
    }
}
