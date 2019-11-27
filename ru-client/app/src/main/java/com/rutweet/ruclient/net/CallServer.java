package com.rutweet.ruclient.net;

import java.util.HashMap;
import java.util.Locale;
import java.util.concurrent.TimeUnit;

import android.os.AsyncTask;

import com.rutweet.ruclient.ipc.Answer;

public class CallServer {
    // 10.0.2.2 is the (default) host IP if we're running inside an emulator
    private final static String host = "10.0.2.2";
    private final static int port = 8000;

    public static Answer call(String method, HashMap<String, String> data) {
        final Answer answer = new Answer();

        try {
            String h = String.format(Locale.getDefault(), "%s%s:%d/%s",
                                     Constants.HTTP_PROTOCOL_PREFIX, CallServer.host,
                                     CallServer.port, method);

            System.out.println(h);
            AsyncTask task = Http.AsyncPOST(h, data, new HttpListener() {
                @Override
                public void onSuccess(int code, String response) {
                    if (code == 200)
                        answer.Parse(response);
                }

                @Override
                public void onError(String error) {
                }
            });

            /*
             * We're not fully async so we need to "wait" the task finish to
             * get its result and simulate a sync call.
             */
            task.get(3, TimeUnit.SECONDS);
        } catch (Exception ignored) {
            ignored.printStackTrace();
        }

        return answer;
    }
}
