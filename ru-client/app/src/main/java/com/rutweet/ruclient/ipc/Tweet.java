package com.rutweet.ruclient.ipc;

import java.util.HashMap;

import com.google.gson.annotations.SerializedName;

import com.rutweet.ruclient.net.CallServer;

public class Tweet {
    @SerializedName("from")
    String from;

    @SerializedName("content")
    String content;

    @SerializedName("id")
    String id;

    @SerializedName("timestamp")
    String timestamp;

    @SerializedName("like")
    int like;

    public static Tweet[] ListTweets(String username) {
        HashMap<String, String> args = new HashMap<>();
        args.put("name", username);

        Answer answer = CallServer.call("listTweet", args);

        if (answer.Code() == 0) {
            return answer.Tweets();
        }

        return null;
    }

    public static Tweet[] ListTweets(String username, String from) {
        HashMap<String, String> args = new HashMap<>();
        args.put("name", username);
        args.put("from", from);

        Answer answer = CallServer.call("listTweet", args);

        if (answer.Code() == 0) {
            return answer.Tweets();
        }

        return null;
    }

    public static void New(String username, String id, String content) {
        HashMap<String, String> args = new HashMap<>();
        args.put("name", username);
        args.put("from", id);
        args.put("content", content);

        Answer answer = CallServer.call("addTweet", args);
    }

    public static void LikeTweet(String sessionId, String id) {
        HashMap<String, String> args = new HashMap<>();
        args.put("from", sessionId);
        args.put("id", id);

        Answer answer = CallServer.call("like", args);
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

    public int Like() {
        return like;
    }
}
