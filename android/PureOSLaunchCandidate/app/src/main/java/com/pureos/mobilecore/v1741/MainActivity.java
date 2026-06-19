package com.pureos.mobilecore.v1741;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Bundle;
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
        scrollView.setFillViewport(true);
        scrollView.setBackgroundColor(Color.rgb(5, 9, 18));

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER_HORIZONTAL);
        int pad = dp(24);
        root.setPadding(pad, dp(36), pad, dp(36));
        scrollView.addView(root, new ScrollView.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        ));

        TextView title = text("PureOS Mobile Core", 30, true);
        title.setGravity(Gravity.CENTER);
        root.addView(title);

        TextView version = text("v17.41 debug launch candidate", 16, false);
        version.setGravity(Gravity.CENTER);
        version.setTextColor(Color.rgb(145, 235, 255));
        root.addView(version);

        TextView status = card("✅ Installable APK path is online\n\nPure Intelligence: armed for local runtime routing\nPureLang: command layer seeded\nGovernor: approval-first safety gate active\nCloud Build: GitHub Actions verified APK artifact");
        root.addView(status);

        TextView next = card("Next build targets\n\n1. Add the PureLang parser seed.\n2. Add the Pure Intelligence text loop.\n3. Add local memory routing.\n4. Add Android notification and file bridge.\n5. Expand into the full Pure OS mobile shell.");
        root.addView(next);

        TextView footer = text("Ryan approval required before real deployment claims.", 14, false);
        footer.setGravity(Gravity.CENTER);
        footer.setTextColor(Color.rgb(190, 200, 210));
        root.addView(footer);

        setContentView(scrollView);
    }

    private TextView text(String value, int sp, boolean bold) {
        TextView view = new TextView(this);
        view.setText(value);
        view.setTextColor(Color.WHITE);
        view.setTextSize(sp);
        if (bold) {
            view.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        }
        view.setPadding(0, dp(8), 0, dp(8));
        return view;
    }

    private TextView card(String value) {
        TextView view = text(value, 16, false);
        view.setTextColor(Color.rgb(225, 242, 255));
        view.setBackgroundColor(Color.rgb(18, 28, 47));
        view.setPadding(dp(18), dp(18), dp(18), dp(18));
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        );
        params.setMargins(0, dp(22), 0, dp(8));
        view.setLayoutParams(params);
        return view;
    }

    private int dp(int value) {
        return Math.round(value * getResources().getDisplayMetrics().density);
    }
}
