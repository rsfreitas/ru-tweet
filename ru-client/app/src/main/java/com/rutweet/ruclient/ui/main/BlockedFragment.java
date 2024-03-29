package com.rutweet.ruclient.ui.main;

import java.util.List;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.MenuItem;
import android.view.View;
import android.view.ViewGroup;
import android.widget.AdapterView;
import android.widget.ArrayAdapter;
import android.widget.ListView;
import android.widget.PopupMenu;

import androidx.fragment.app.Fragment;

import com.rutweet.ruclient.MainActivity;
import com.rutweet.ruclient.R;
import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.ipc.User;

public class BlockedFragment extends Fragment {
    private Credentials credentials;
    private boolean viewCreated = false;

    private void populateListView(ListView listView) {
        List<String> dataList = User.ListBlocked(credentials.Id());

        if (dataList != null) {
            ArrayAdapter<String> arrayAdapter = new ArrayAdapter<String>(
                    getActivity(), android.R.layout.simple_list_item_1, dataList
            );

            listView.setAdapter(arrayAdapter);
        }
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        viewCreated = false;
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        viewCreated = true;
        credentials = (Credentials)getArguments().getSerializable("credentials");
        return inflater.inflate(R.layout.blocked_fragment, container, false);
    }

    @Override
    public void onActivityCreated(Bundle savedInstanceState) {
        super.onActivityCreated(savedInstanceState);

        final ViewGroup viewGroup = (ViewGroup) getView();
        final ListView listView = (ListView) viewGroup.findViewById(R.id.blocked_listview);
        listView.setClickable(true);
        listView.setOnItemClickListener(new AdapterView.OnItemClickListener() {
            @Override
            public void onItemClick(AdapterView<?> parent, View view, int position, long id) {
                showMenu(view, (String)listView.getItemAtPosition(position), listView);
            }
        });

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
            final ViewGroup viewGroup = (ViewGroup)getView();
            final ListView listView = (ListView)viewGroup.findViewById(R.id.blocked_listview);

            populateListView(listView);
            MainActivity.setActiveFragment(com.rutweet.ruclient.common.Fragment.Blocked);
        }
    }

    private void unblockUser(String user) {
        User.Unblock(credentials.Id(), user);
    }

    private void showMenu(View view, final String user, final ListView listView) {
        PopupMenu menu = new PopupMenu(getActivity(), view);
        menu.setOnMenuItemClickListener(new PopupMenu.OnMenuItemClickListener () {
            @Override
            public boolean onMenuItemClick(MenuItem item) {
                int id = item.getItemId();

                switch (id) {
                case R.id.blocked_item_unblock:
                    unblockUser(user);
                    populateListView(listView);
                    break;
                }

                return true;
            }
        });

        menu.inflate(R.menu.blocked_item_menu);
        menu.show();
    }
}
