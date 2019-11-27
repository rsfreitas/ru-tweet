package com.rutweet.ruclient.ui.main;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ListView;
import android.widget.SimpleAdapter;

import androidx.fragment.app.Fragment;

import com.rutweet.ruclient.MainActivity;
import com.rutweet.ruclient.R;
import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.ipc.Tweet;

public class PersonalFragment extends Fragment {
    private Credentials credentials;
    private boolean viewCreated = false;

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        viewCreated = true;
        credentials = (Credentials)getArguments().getSerializable("credentials");
        return inflater.inflate(R.layout.personal_fragment, container, false);
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        viewCreated = false;
    }

    private void populateListView(ListView listView) {
        Tweet[] tweets = Tweet.ListTweets(credentials.Username());

        if (tweets == null)
            return;

        List<Map<String, String>> itemDataList = new ArrayList<>();

        for (Tweet t : tweets) {
            SimpleDateFormat fmt = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
            String timestamp = fmt.format(t.Timestamp());

            HashMap<String, String> item = new HashMap<>();

            item.put("content", t.Content());
            item.put("timestamp", String.format(Locale.getDefault(), "%s - Likes:%d",
                    timestamp, t.Like()));

            itemDataList.add(item);
        }

        SimpleAdapter simpleAdapter = new SimpleAdapter(
                getActivity(),
                itemDataList,android.R.layout.simple_list_item_2,
                new String[]{"timestamp", "content"},
                new int[]{android.R.id.text2, android.R.id.text1}
        );

        listView.setAdapter(simpleAdapter);
    }

    @Override
    public void onActivityCreated(Bundle savedInstanceState) {
        super.onActivityCreated(savedInstanceState);

        final ViewGroup viewGroup = (ViewGroup)getView();
        final ListView listView = (ListView)viewGroup.findViewById(R.id.personal_listview);
        populateListView(listView);
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setUserVisibleHint(false);
    }

    @Override
    public void setMenuVisibility(final boolean visible) {
        super.setMenuVisibility(visible);

        if (visible && viewCreated) {
            MainActivity.setActiveFragment(com.rutweet.ruclient.common.Fragment.Personal);
        }
    }
}
