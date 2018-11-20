package com.binding;

import com.binding.rust.Binding;
import com.binding.rust.RustCallback;

import java.util.Arrays;

public class Main {
    private static final class MyCallback implements RustCallback {
        @Override
        public void callback(int a0) {
            System.out.println("Hello from callback! " + a0);
        }
    }

    public static void main(String[] args) {
        System.loadLibrary("binding_test_library");

        System.out.println("ABS(0) = " + Binding.abs(0));
        System.out.println("ABS(1) = " + Binding.abs(1));
        System.out.println("ABS(-1) = " + Binding.abs(-1));

        System.out.println("MAX(1, 3, 7) = " + Binding.max(new int[]{1, 3, 7}));
        System.out.println("MAX(1, 1, 1) = " + Binding.max(new int[]{1, 1, 1}));
        System.out.println("MAX(57, 0, 10) = " + Binding.max(new int[]{57, 0, 10}));

        byte[] vectorFromRust = Binding.createVec(10);
        System.out.println("CREATE_VEC(10) = " + Arrays.toString(vectorFromRust));

        Binding.useCallback(new MyCallback(), 10);
    }
}
