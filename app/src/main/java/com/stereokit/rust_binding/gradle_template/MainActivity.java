package com.stereokit.rust_binding.gradle_template;

import android.os.Bundle;
import android.util.Log;
import android.os.Process;

public class MainActivity extends android.app.NativeActivity {

    @Override
    protected void onCreate( Bundle savedInstanceState ) {
        Log.d("StereoKitJ", "!!!!onCreate");
        super.onCreate(savedInstanceState);
    }

    @Override
    protected void onResume() {
        super.onResume();
    }
	
	@Override
    protected void onPause() {
	    super.onPause();
    }

    @Override
    protected void onDestroy( ) {
        Log.d("StereoKitJ", "!!!!onDestroy");
        super.onDestroy();
        //Process.killProcess(Process.myPid());
    }
    
    static {
        System.loadLibrary("openxr_loader");
        System.loadLibrary("stereokit_rust_gradle_template");
    }
}