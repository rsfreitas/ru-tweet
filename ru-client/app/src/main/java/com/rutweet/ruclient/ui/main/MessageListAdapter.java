package com.rutweet.ruclient.ui.main;

import java.text.SimpleDateFormat;
import java.util.List;

import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.TextView;

import androidx.recyclerview.widget.RecyclerView;

import com.rutweet.ruclient.R;
import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.ipc.DirectMessage;

public class MessageListAdapter extends RecyclerView.Adapter {
    private static final int VIEW_TYPE_MESSAGE_SENT = 1;
    private static final int VIEW_TYPE_MESSAGE_RECEIVED = 2;
    private List<DirectMessage> directMessages;
    private Credentials credentials;

    public MessageListAdapter(Credentials credentials, List<DirectMessage> messageList) {
        this.credentials = credentials;
        directMessages = messageList;
    }

    public void updateMessages(List<DirectMessage> messages) {
        this.directMessages = messages;
    }

    @Override
    public RecyclerView.ViewHolder onCreateViewHolder(ViewGroup parent, int viewType) {
        View view;

        if (viewType == VIEW_TYPE_MESSAGE_SENT) {
            view = LayoutInflater.from(parent.getContext())
                                 .inflate(R.layout.item_message_sent, parent, false);

            return new SentMessageHolder(view);
        } else if (viewType == VIEW_TYPE_MESSAGE_RECEIVED) {
            view = LayoutInflater.from(parent.getContext())
                                 .inflate(R.layout.item_message_received, parent, false);

            return new ReceivedMessageHolder(view);
        }

        return null;
    }

    @Override
    public void onBindViewHolder(RecyclerView.ViewHolder holder, int position) {
        DirectMessage message = (DirectMessage)directMessages.get(position);

        switch (holder.getItemViewType()) {
        case VIEW_TYPE_MESSAGE_SENT:
            ((SentMessageHolder)holder).bind(message);
            break;

        case VIEW_TYPE_MESSAGE_RECEIVED:
            ((ReceivedMessageHolder)holder).bind(message);
            break;
        }
    }

    @Override
    public int getItemCount() {
        return directMessages.size();
    }

    @Override
    public int getItemViewType(int position) {
        DirectMessage message = (DirectMessage)directMessages.get(position);

        if (message.From().equals(credentials.Username()))
            return VIEW_TYPE_MESSAGE_SENT;

        return VIEW_TYPE_MESSAGE_RECEIVED;
    }

    private class SentMessageHolder extends RecyclerView.ViewHolder {
        TextView messageText, timeText;

        SentMessageHolder(View itemView) {
            super(itemView);
            messageText = (TextView) itemView.findViewById(R.id.text_message_body);
            timeText = (TextView) itemView.findViewById(R.id.text_message_time);
        }

        void bind(DirectMessage message) {
            SimpleDateFormat fmt = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
            String timestamp = fmt.format(message.Timestamp());

            messageText.setText(message.Content());
            timeText.setText(timestamp);
        }
    }

    private class ReceivedMessageHolder extends RecyclerView.ViewHolder {
        TextView messageText, timeText;

        ReceivedMessageHolder(View itemView) {
            super(itemView);
            messageText = (TextView) itemView.findViewById(R.id.text_message_body);
            timeText = (TextView) itemView.findViewById(R.id.text_message_time);
        }

        void bind(DirectMessage message) {
            SimpleDateFormat fmt = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
            String timestamp = fmt.format(message.Timestamp());

            messageText.setText(message.Content());
            timeText.setText(timestamp);
        }
    }
}
