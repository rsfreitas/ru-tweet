package com.rutweet.ruclient.ui.main;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.MenuItem;
import android.view.View;
import android.view.ViewGroup;
import android.widget.AdapterView;
import android.widget.ArrayAdapter;
import android.widget.ListView;
import android.widget.PopupMenu;
import android.widget.Toast;

import com.rutweet.ruclient.R;

import java.util.ArrayList;
import java.util.List;

import androidx.fragment.app.Fragment;

public class FollowingFragment extends Fragment {
    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        return inflater.inflate(R.layout.following_fragment, container, false);
    }

    @Override
    public void onActivityCreated(Bundle savedInstanceState) {
        super.onActivityCreated(savedInstanceState);

        final ViewGroup viewGroup = (ViewGroup)getView();
        final ListView listView = (ListView)viewGroup.findViewById(R.id.following_listview);
        listView.setClickable(true);
        listView.setOnItemClickListener(new AdapterView.OnItemClickListener() {
            @Override
            public void onItemClick(AdapterView<?> parent, View view, int position, long id) {
                showMenu(view);
            }
        });

        List<String> dataList = new ArrayList<String>();
        dataList.add("Java");
        dataList.add("Android");
        dataList.add("JavaEE");
        dataList.add("JSP");
        dataList.add("JDBC");

        ArrayAdapter<String> arrayAdapter = new ArrayAdapter<String>(
                getActivity(), android.R.layout.simple_list_item_1, dataList
        );

        listView.setAdapter(arrayAdapter);
    }

    private void showMenu(View view) {
        PopupMenu menu = new PopupMenu(getActivity(), view);
        menu.setOnMenuItemClickListener(new PopupMenu.OnMenuItemClickListener () {
            @Override
            public boolean onMenuItemClick(MenuItem item) {
                int id = item.getItemId();

                switch (id) {
                case R.id.following_item_dm:
                    break;

                case R.id.following_item_unfollow:
                    break;
                }

                return true;
            }
        });

        menu.inflate (R.menu.following_item_menu);
        menu.show();
    }
}
