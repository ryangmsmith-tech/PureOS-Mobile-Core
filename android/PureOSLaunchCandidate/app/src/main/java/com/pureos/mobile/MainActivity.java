package com.pureos.mobile;

import android.app.Activity;
import android.os.Bundle;
import android.graphics.Color;
import android.graphics.Typeface;
import android.view.Gravity;
import android.view.ViewGroup;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

public class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        ScrollView scrollView = new ScrollView(this);
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER_HORIZONTAL);
        root.setPadding(48, 72, 48, 72);
        root.setBackgroundColor(Color.rgb(8, 10, 18));
        scrollView.addView(root, new ScrollView.LayoutParams(
                ScrollView.LayoutParams.MATCH_PARENT,
                ScrollView.LayoutParams.WRAP_CONTENT
        ));

        TextView title = makeText("PureOS Mobile Core", 30, true);
        title.setGravity(Gravity.CENTER);
        root.addView(title);

        TextView version = makeText("v17.40 • Phone Cloud Build Candidate", 16, false);
        version.setGravity(Gravity.CENTER);
        root.addView(version);

        addDivider(root);
        root.addView(makeText("Pure Intelligence", 22, true));
        root.addView(makeText("Local-first assistant layer placeholder. This screen proves the Android launch candidate opens and can present the PureOS status shell.", 16, false));

        addDivider(root);
        root.addView(makeText("PureLang", 22, true));
        root.addView(makeText("Command layer placeholder for geometry-native, shape-validated actions. Real command execution stays behind Ryan approval and security gates.", 16, false));

        addDivider(root);
        root.addView(makeText("Build Receipt", 22, true));
        root.addView(makeText("If you are reading this on the phone, the APK installed and launched. Next step: send the Actions result or launch screenshot back for the v17.41 pass/fix seal.", 16, false));

        setContentView(scrollView);
    }

    private TextView makeText(String text, int sp, boolean bold) {
        TextView view = new TextView(this);
        view.setText(text);
        view.setTextColor(Color.WHITE);
        view.setTextSize(sp);
        view.setLineSpacing(6f, 1.05f);
        view.setPadding(0, 16, 0, 16);
        if (bold) {
            view.setTypeface(Typeface.DEFAULT_BOLD);
        }
        view.setLayoutParams(new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        ));
        return view;
    }

    private void addDivider(LinearLayout root) {
        TextView divider = new TextView(this);
        divider.setText("◆ ◆ ◆");
        divider.setTextColor(Color.rgb(255, 220, 120));
        divider.setGravity(Gravity.CENTER);
        divider.setTextSize(18);
        divider.setPadding(0, 22, 0, 22);
        root.addView(divider);
    }
}
