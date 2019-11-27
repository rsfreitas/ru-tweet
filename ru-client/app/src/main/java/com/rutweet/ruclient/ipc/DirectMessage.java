package com.rutweet.ruclient.ipc;

import com.google.gson.annotations.SerializedName;
import com.rutweet.ruclient.common.DateUtil;
import com.rutweet.ruclient.net.CallServer;

import java.util.ArrayList;
import java.util.Date;
import java.util.HashMap;
import java.util.List;

public class DirectMessage implements Comparable<DirectMessage> {
    @SerializedName("from")
    String from;

    @SerializedName("to")
    String to;

    @SerializedName("content")
    String content;

    @SerializedName("timestamp")
    String timestamp;

    public Date Timestamp() {
        try {
            return DateUtil.parseRFC3339(timestamp);
        } catch (Exception ignored) {
        }

        return null;
    }

    public String From() {
        return from;
    }

    public String To() {
        return to;
    }

    public String Content() {
        return content;
    }

    private DirectMessage(String from, String to, String content,
                          String timestamp)
    {
        this.from = from;
        this.to = to;
        this.content = content;
        this.timestamp = timestamp;
    }

    public static List<DirectMessage> ListDM(String from, String to) {
        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);
        args.put("to", to);

        List<DirectMessage> directMessages = new ArrayList<>();
        Answer answer = CallServer.call("listDm", args);

        if (answer.Code() != 0)
            return directMessages;

        DirectMessage[] dms = answer.DirectMessages();

        // This is not good...
        if (dms != null)
            for (DirectMessage d : dms)
                directMessages.add(new DirectMessage(d.From(), d.To(), d.Content(), d.timestamp));

        return directMessages;
    }

    public static int Send(String from, String to, String content) {
        int rc = -1;

        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);
        args.put("to", to);
        args.put("content", content);

        Answer answer = CallServer.call("dm", args);

        if (answer != null)
            rc = answer.Code();

        return rc;
    }

    @Override
    public int compareTo(DirectMessage m) {
        if ((Timestamp() == null) || (m.Timestamp() == null))
            return 0;

        return Timestamp().compareTo(m.Timestamp());
    }
}
