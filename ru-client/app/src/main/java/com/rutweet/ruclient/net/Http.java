package com.rutweet.ruclient.net;

import com.google.gson.Gson;

import java.io.BufferedReader;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.UnsupportedEncodingException;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLEncoder;
import java.util.HashMap;
import java.util.Map;

import javax.net.ssl.HttpsURLConnection;

import android.os.AsyncTask;

public class Http {
    private static String toURLEncoded(HashMap<String, String> data) throws UnsupportedEncodingException {
        StringBuilder d = new StringBuilder();
        boolean first = true;

        for (Map.Entry<String, String> entry : data.entrySet()) {
            if (first)
                first = false;
            else
                d.append(Constants.HTTP_URL_ENCODED_DELIMITER);

            d.append(URLEncoder.encode(entry.getKey(), Constants.UTF8));
            d.append(Constants.EQUAL);
            d.append(URLEncoder.encode(entry.getValue(), Constants.UTF8));
        }

        return d.toString();
    }

    private static void sendData(HttpURLConnection conn,
                                 HashMap<String, String> data, boolean json)
            throws IOException
    {
        String d;

        if (json) {
            d = new Gson().toJson(data);
        } else
            d = toURLEncoded(data);

        System.out.println("Body: " + d);

        DataOutputStream wr = new DataOutputStream(conn.getOutputStream());
        wr.writeBytes(d);
        wr.flush();
        wr.close();
    }

    private static Pair<Integer, String> request(String method, String host,
                                                 HashMap<String, String> data,
                                                 boolean jsonContent)
            throws IOException
    {
        if (method.equals(Constants.HTTP_GET))
            host += "?" + toURLEncoded(data);

        URL url = new URL(host);
        HttpURLConnection conn;

        if (url.getProtocol().equals(Constants.HTTPS_PROTOCOL_NAME))
            conn = (HttpsURLConnection)url.openConnection();
        else
            conn = (HttpURLConnection)url.openConnection();

        conn.setConnectTimeout(2000);
        conn.setReadTimeout(2000);
        conn.setRequestMethod(method);

        if (jsonContent)
            conn.setRequestProperty(Constants.HTTP_CONTENT_TYPE, Constants.HTTP_CONTENT_TYPE_JSON);
        else
            conn.setRequestProperty(Constants.HTTP_CONTENT_TYPE, Constants.HTTP_CONTENT_TYPE_URL_ENCODED);

        if (method.equals(Constants.HTTP_POST)) {
            conn.setDoOutput(true);
            sendData(conn, data, jsonContent);
        } else
            conn.setDoInput(true);

        int code = conn.getResponseCode();
        StringBuffer out = new StringBuffer();
        String content;
        BufferedReader in = new BufferedReader(
                new InputStreamReader(conn.getInputStream())
        );

        while ((content = in.readLine()) != null)
            out.append(content);

        in.close();
        conn.disconnect();

        return Pair.create(code, out.toString());
    }

    private static Pair<Integer, String> POST(String host, HashMap<String, String> data,
                                             boolean jsonContent)
            throws IOException
    {
        return request(Constants.HTTP_POST, host, data, jsonContent);
    }

    private static class AsyncRequest extends AsyncTask<Void, Void, Void> {
        private HttpListener listener;
        private String host;
        private HashMap<String, String> data;

        AsyncRequest(String host, HashMap<String, String> data,
                     HttpListener listener)
        {
            this.host = host;
            this.data = data;
            this.listener = listener;
        }

        @Override
        protected Void doInBackground(Void... params) {
            Pair<Integer, String> response;

            try {
                response = Http.POST(host, data, true);
            } catch (IOException e) {
                listener.onError(e.getMessage());
                return null;
            }

            listener.onSuccess(response.first(), response.second());
            return null;
        }
    }

    static AsyncTask<Void, Void, Void> AsyncPOST(String host, HashMap<String, String> data,
                                                 HttpListener listener)
    {
        return new AsyncRequest(host, data, listener).execute();
    }
}
