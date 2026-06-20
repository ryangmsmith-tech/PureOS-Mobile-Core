package com.pureos.mobilecore.v1746;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Bundle;
import android.view.Gravity;
import android.widget.LinearLayout;
import android.widget.TextView;

public class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER);
        root.setPadding(32, 32, 32, 32);
        root.setBackgroundColor(Color.rgb(5, 9, 18));

        TextView title = new TextView(this);
        title.setText("PureOS Mobile Core");
        title.setTextColor(Color.rgb(255, 215, 106));
        title.setTextSize(28);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        title.setGravity(Gravity.CENTER);
        root.addView(title);

        TextView status = new TextView(this);
        status.setText("v17.46 clean Android launch test\n\nPure Intelligence route ready\nPureLang layer ready\nApproval gate active\nGitHub Actions APK artifact ready");
        status.setTextColor(Color.WHITE);
        status.setTextSize(16);
        status.setGravity(Gravity.CENTER);
        status.setPadding(0, 28, 0, 0);
        root.addView(status);

        setContentView(root);
    }
}
