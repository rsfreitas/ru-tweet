package com.rutweet.ruclient;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Map;

import android.content.Intent;
import androidx.localbroadcastmanager.content.LocalBroadcastManager;

import com.google.firebase.messaging.FirebaseMessagingService;
import com.google.firebase.messaging.RemoteMessage;

import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.common.Fragment;
import com.rutweet.ruclient.ipc.Login;

public class NotificationService extends FirebaseMessagingService {
    private LocalBroadcastManager broadcaster;
    private Credentials credentials;
    private static final String FILENAME = "token";

    @Override
    public void onCreate() {
        super.onCreate();
        broadcaster = LocalBroadcastManager.getInstance(this);
        credentials = Credentials.load(getFilesDir().getAbsolutePath());
    }

    @Override
    public void onMessageReceived(RemoteMessage message) {
        super.onMessageReceived(message);

        Map<String, String> data = message.getData();
        String command = data.get("command");

        if ((command != null) && command.equals("dm") &&
            (MainActivity.getActiveFragment() == Fragment.Following))
        {
            Intent intent = new Intent();
            intent.putExtra("reload", true);
            broadcaster.sendBroadcast(intent);
        }
    }

    public void save(String dirname, String token) {
        String filename = dirname + "/" + FILENAME;

        try (FileWriter file = new FileWriter(filename)) {
            file.write(token);
        } catch (IOException ignored) {
        }
    }

    public static String loadToken(String dirname) {
        String filename = dirname + "/" + FILENAME;
        StringBuilder builder = new StringBuilder();
        String s;

        try (BufferedReader br = new BufferedReader(new FileReader(filename))) {
            String l;

            while ((l = br.readLine()) != null)
                builder.append(l);

            s = builder.toString();
        } catch (Exception e) {
            s = null;
        }

        return s;
    }

    @Override
    public void onNewToken(String token) {
        super.onNewToken(token);
        String path = getFilesDir().getAbsolutePath();

        /*
         * Always saves the token so that when the user logs in it will be
         * available.
         */
        save(path, token);

        if (Login.isUserLogged(path)) {
            if (credentials == null)
                credentials = Credentials.load(path);

            Login.token(credentials.Id(), token);
        }
    }
}
