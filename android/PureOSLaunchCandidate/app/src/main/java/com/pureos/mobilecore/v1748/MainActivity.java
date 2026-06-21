package com.pureos.mobilecore.v1748;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Bundle;
import android.view.Gravity;
import android.view.View;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.TextView;

public class MainActivity extends Activity {
    private static boolean nativeLibraryLoaded = false;
    private static String nativeLoadError = "";

    static {
        try {
            System.loadLibrary("pureos_native_runtime_bootstrap");
            nativeLibraryLoaded = true;
        } catch (UnsatisfiedLinkError error) {
            nativeLoadError = error.getMessage() == null ? error.toString() : error.getMessage();
        }
    }

    private static native int nativeContractStatus();
    private static native int nativeGoldOceanSections();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER_HORIZONTAL);
        root.setPadding(40, 56, 40, 40);
        root.setBackgroundColor(Color.rgb(5, 9, 18));

        TextView title = new TextView(this);
        title.setText("PureOS Mobile Core");
        title.setTextColor(Color.rgb(255, 215, 106));
        title.setTextSize(30);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        title.setGravity(Gravity.CENTER);
        root.addView(title);

        TextView subtitle = new TextView(this);
        subtitle.setText("v17.48 • Android ARM64 Native Runtime Bootstrap");
        subtitle.setTextColor(Color.rgb(141, 219, 255));
        subtitle.setTextSize(16);
        subtitle.setGravity(Gravity.CENTER);
        subtitle.setPadding(0, 12, 0, 28);
        root.addView(subtitle);

        TextView status = new TextView(this);
        status.setTextColor(Color.WHITE);
        status.setTextSize(16);
        status.setGravity(Gravity.CENTER);
        status.setPadding(0, 16, 0, 24);
        root.addView(status);

        Button runCheck = new Button(this);
        runCheck.setText("Run Native Contract Check");
        runCheck.setAllCaps(false);
        root.addView(runCheck);

        TextView boundary = new TextView(this);
        boundary.setText(
            "Current boundary:\n" +
            "Native Rust library packaged for ARM64\n" +
            "Gold Ocean City slice contract: 6 of 6\n" +
            "Pure Intelligence route ready\n\n" +
            "No native GPU frame, live vehicle physics, headset test, or production deployment is claimed."
        );
        boundary.setTextColor(Color.rgb(190, 199, 215));
        boundary.setTextSize(14);
        boundary.setGravity(Gravity.CENTER);
        boundary.setPadding(0, 28, 0, 0);
        root.addView(boundary);

        View.OnClickListener checkListener = view -> updateNativeStatus(status);
        runCheck.setOnClickListener(checkListener);
        updateNativeStatus(status);

        setContentView(root);
    }

    private void updateNativeStatus(TextView status) {
        if (!nativeLibraryLoaded) {
            status.setText(
                "Native ARM64 library: NOT LOADED\n\n" +
                "Reason: " + nativeLoadError
            );
            status.setTextColor(Color.rgb(255, 138, 138));
            return;
        }

        try {
            int contractStatus = nativeContractStatus();
            int mergedSections = nativeGoldOceanSections();
            boolean ready = contractStatus == 1 && mergedSections == 6;

            status.setText(
                "Native ARM64 library: LOADED\n" +
                "Rust contract: " + (contractStatus == 1 ? "PASSED" : "FAILED") + "\n" +
                "Gold Ocean City sections: " + mergedSections + " / 6\n" +
                "PureRenderIR intake: v0.8\n" +
                "Pure Intelligence runtime: v17.31R\n" +
                "Overall bootstrap: " + (ready ? "READY" : "BLOCKED")
            );
            status.setTextColor(ready ? Color.rgb(137, 255, 178) : Color.rgb(255, 196, 112));
        } catch (RuntimeException error) {
            status.setText("Native contract call failed: " + error.getMessage());
            status.setTextColor(Color.rgb(255, 138, 138));
        }
    }
}
