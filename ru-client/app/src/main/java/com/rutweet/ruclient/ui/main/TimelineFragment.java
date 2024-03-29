package com.rutweet.ruclient.ui.main;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.MenuItem;
import android.view.View;
import android.view.ViewGroup;
import android.widget.AdapterView;
import android.widget.ListView;
import android.widget.PopupMenu;
import android.widget.SimpleAdapter;

import androidx.fragment.app.Fragment;

import com.rutweet.ruclient.MainActivity;
import com.rutweet.ruclient.R;
import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.ipc.Tweet;
import com.rutweet.ruclient.ipc.User;

public class TimelineFragment extends Fragment {
    private Credentials credentials;
    private boolean viewCreated = false;

    private void populateListView(ListView listView) {
        List<String> following = User.ListFollowing(credentials.Id());

        if (following == null)
            return;

        List<Tweet> tweets = new ArrayList<>();

        for (String f : following) {
            Tweet[] t = Tweet.ListTweets(credentials.Username(), f);

            if (t != null)
                tweets.addAll(Arrays.asList(t));
        }

        if (tweets.size() == 0)
            return;

        Collections.sort(tweets);
        List<Map<String, String>> itemDataList = new ArrayList<>();

        for (Tweet t : tweets) {
            SimpleDateFormat fmt = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
            String timestamp = fmt.format(t.Timestamp());

            HashMap<String, String> item = new HashMap<>();
            item.put("timestamp",
                     String.format(Locale.getDefault(), "%s - %s Likes:%d",
                                   timestamp, t.From(), t.Like()));

            item.put("content", t.Content());
            item.put("author", t.From());
            item.put("id", t.Id());

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
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setUserVisibleHint(false);
    }

    @Override
    public void setMenuVisibility(final boolean visible) {
        super.setMenuVisibility(visible);

        if (visible && viewCreated) {
            final ViewGroup viewGroup = (ViewGroup)getView();
            final ListView listView = (ListView)viewGroup.findViewById(R.id.timeline_listview);

            populateListView(listView);
            MainActivity.setActiveFragment(com.rutweet.ruclient.common.Fragment.Timeline);
        }
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        viewCreated = true;
        credentials = (Credentials)getArguments().getSerializable("credentials");
        return inflater.inflate(R.layout.timeline_fragment, container, false);
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        viewCreated = false;
    }

    @Override
    public void onActivityCreated(Bundle savedInstanceState) {
        super.onActivityCreated(savedInstanceState);

        final ViewGroup viewGroup = (ViewGroup)getView();
        final ListView listView = (ListView)viewGroup.findViewById(R.id.timeline_listview);

        listView.setClickable(true);
        listView.setOnItemClickListener(new AdapterView.OnItemClickListener() {
            @Override
            public void onItemClick(AdapterView<?> parent, View view, int position, long id) {
                HashMap<String, String> data = (HashMap<String, String>)listView.getItemAtPosition(position);
                showMenu(view, data.get("author"), data.get("id"), listView);
            }
        });

        populateListView(listView);
    }

    private void showMenu(View view, final String user, final String msgId,
                          final ListView listView)
    {
        PopupMenu menu = new PopupMenu(getActivity(), view);
        menu.setOnMenuItemClickListener(new PopupMenu.OnMenuItemClickListener () {
            @Override
            public boolean onMenuItemClick(MenuItem item) {
                int id = item.getItemId();

                switch (id) {
                case R.id.timeline_item_like:
                    Tweet.LikeTweet(credentials.Id(), msgId);
                    break;

                case R.id.timeline_item_pingback:
                    // TODO: Answer tweet
                    break;

                case R.id.timeline_item_retweet:
                    // TODO: retweet
                    break;
                }

                return true;
            }
        });

        menu.inflate(R.menu.timeline_item_menu);
        menu.show();
    }
}
