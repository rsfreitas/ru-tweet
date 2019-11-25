package com.rutweet.ruclient.net;

import android.os.AsyncTask;

import com.rutweet.ruclient.ipc.Answer;

import java.util.HashMap;
import java.util.Locale;
import java.util.concurrent.TimeUnit;

public class CallServer {
    private static String host;
    private static int port;

    /*
     * Method responsible for initialize the server settings, host and port. It
     * must be called at least one time in order to make requests against the
     * server later.
     */
    public static void setHostname(String host, int port) {
        CallServer.host = host;
        CallServer.port = port;
    }

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
