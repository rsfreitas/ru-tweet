package com.rutweet.ruclient.net;

public interface HttpListener {
    void onSuccess(int code, String response);
    void onError(String error);
}
