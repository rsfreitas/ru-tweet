package com.rutweet.ruclient.ipc;

import com.google.gson.Gson;
import com.google.gson.annotations.SerializedName;

public class Answer {
    private AnswerData data;
    private boolean empty = true;

    public void Parse(String content) {
        System.out.println("Answer: " + content);
        Gson g = new Gson();
        data = g.fromJson(content, AnswerData.class);
        this.empty = false;
    }

    public String Id() {
        if (empty)
            return null;

        return data.id;
    }

    public int Code() {
        if (empty)
            return -1;

        return data.code;
    }

    public Tweet[] Tweets() {
        if (empty)
            return null;

        return data.tweets;
    }

    private class AnswerData {
        @SerializedName("id")
        String id;

        @SerializedName("code")
        int code;

        @SerializedName("tweets")
        Tweet[] tweets;
    }
}
