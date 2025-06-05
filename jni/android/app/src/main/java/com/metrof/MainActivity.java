package com.metrof;

import co.realfit.nawinitwgpu.R;
import android.os.Build;
import android.os.Bundle;
import android.widget.Button;
import android.view.View;
import android.widget.EditText;
import android.widget.Toast;
import android.app.NativeActivity;
import android.app.NotificationChannel;
import android.app.NotificationManager;

public class MainActivity extends NativeActivity {

static {
        System.loadLibrary("android_rust_example_apk_jni");

}
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
    }
}
