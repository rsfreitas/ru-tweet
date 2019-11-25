package com.rutweet.ruclient.ui.main;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ListView;
import android.widget.SimpleAdapter;

import androidx.fragment.app.Fragment;

import com.rutweet.ruclient.R;
import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.ipc.Tweet;

public class PersonalFragment extends Fragment {
    private Credentials credentials;

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        credentials = (Credentials)getArguments().getSerializable("credentials");

        if (credentials != null)
            System.out.println("fragment:" + credentials.Id());

        return inflater.inflate(R.layout.personal_fragment, container, false);
    }

    @Override
    public void onActivityCreated(Bundle savedInstanceState) {
        super.onActivityCreated(savedInstanceState);

        Tweet[] tweets = Tweet.ListTweets(credentials.Id(), credentials.Username());

        if (tweets == null)
            return;

        final ViewGroup viewGroup = (ViewGroup)getView();
        final ListView listView = (ListView)viewGroup.findViewById(R.id.personal_listview);
        List<Map<String, String>> itemDataList = new ArrayList<>();

        for (Tweet t : tweets) {
            HashMap<String, String> item = new HashMap<>();
            // Timestamp: %s Like: %d
            item.put("timestamp", t.Timestamp());
            item.put("content", t.Content());

            itemDataList.add(item);
        }

        SimpleAdapter simpleAdapter = new SimpleAdapter(
                getActivity(),
                itemDataList,android.R.layout.simple_list_item_2,
                new String[]{"timestamp", "content"},
                new int[]{android.R.id.text1, android.R.id.text2}
         );

        listView.setAdapter(simpleAdapter);
    }
}
