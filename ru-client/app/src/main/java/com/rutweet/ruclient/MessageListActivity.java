package com.rutweet.ruclient;

import java.util.Collections;
import java.util.List;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;

import androidx.appcompat.app.AppCompatActivity;
import androidx.localbroadcastmanager.content.LocalBroadcastManager;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import com.rutweet.ruclient.common.Constants;
import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.common.Fragment;
import com.rutweet.ruclient.ipc.DirectMessage;
import com.rutweet.ruclient.ui.main.MessageListAdapter;

public class MessageListActivity extends AppCompatActivity {
    private RecyclerView recycler;
    private MessageListAdapter adapter;
    private Credentials credentials;
    private String contactName;

    private void reloadList() {
        List<DirectMessage> messages = DirectMessage.ListDM(credentials.Username(), contactName);
        Collections.sort(messages);

        adapter.notifyDataSetChanged();
        adapter.updateMessages(messages);
        recycler.scrollToPosition(messages.size() - 1);
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_message_list);

        Intent intent = getIntent();
        credentials = (Credentials)intent.getSerializableExtra("credentials");
        contactName = intent.getStringExtra("to");
        final List<DirectMessage> directMessages = DirectMessage.ListDM(credentials.Username(), contactName);
        Collections.sort(directMessages);

        final EditText editText = (EditText)findViewById(R.id.edittext_chatbox);
        final Button send = (Button)findViewById(R.id.button_chatbox_send);

        send.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                DirectMessage.Send(credentials.Username(), contactName,
                                   editText.getText().toString());

                editText.setText("");
                reloadList();
            }
        });

        recycler = (RecyclerView)findViewById(R.id.recylerview_message_list);
        adapter = new MessageListAdapter(credentials, directMessages);
        recycler.setLayoutManager(new LinearLayoutManager(this));
        recycler.setAdapter(adapter);
        recycler.scrollToPosition(directMessages.size() - 1);

        /* Handles a FCM notification when a new message is received */
        BroadcastReceiver broadcastReceiver = new BroadcastReceiver() {
            @Override
            public void onReceive(Context context, Intent intent) {
                boolean reload = intent.getBooleanExtra("reload", false);

                if (reload)
                    reloadList();
            }
        };

        LocalBroadcastManager.getInstance(getApplicationContext())
                             .registerReceiver(broadcastReceiver,
                                               new IntentFilter(Constants.DM_BROADCAST));

        MainActivity.setActiveFragment(Fragment.DirectMessage);
    }
}
