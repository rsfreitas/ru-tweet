package com.rutweet.ruclient.ipc;

import com.google.gson.annotations.SerializedName;
import com.rutweet.ruclient.net.CallServer;

import java.util.HashMap;

public class Tweet {
    @SerializedName("from")
    String from;

    @SerializedName("content")
    String content;

    @SerializedName("id")
    String id;

    @SerializedName("timestamp")
    String timestamp;

    public static Tweet[] ListTweets(String id, String username) {
        HashMap<String, String> args = new HashMap<>();
        args.put("from", id);
        args.put("name", username);

        Answer answer = CallServer.call("listTweet", args);

        if (answer.Code() == 0) {
            return answer.Tweets();
        }

        return null;
    }

    public static void New(String from, String content) {
        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);
        args.put("content", content);

        Answer answer = CallServer.call("addTweet", args);
    }

    public String Timestamp() {
        return timestamp;
    }

    public String Id() {
        return id;
    }

    public String From() {
        return from;
    }

    public String Content() {
        return content;
    }
}
