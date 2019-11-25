package com.rutweet.ruclient.common;

import com.google.gson.Gson;
import com.google.gson.annotations.SerializedName;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Serializable;

public class Credentials implements Serializable {
    private String username;
    private String password;
    private String id;

    public String Username() {
        return username;
    }

    public String Id() {
        return id;
    }

    private Credentials(String username, String password, String id) {
        this.username = username;
        this.password = password;
        this.id = id;
    }

    private static class CredentialsData {
        @SerializedName("id")
        String id;

        @SerializedName("password")
        String password;

        @SerializedName("username")
        String username;

        CredentialsData(String id, String username, String password) {
            this.id = id;
            this.username = username;
            this.password = password;
        }
    }

    /*
     * Saves the user into a credentials file.
     */
    public static void save(String dirname, String username, String password,
                            String id)
    {
        String filename = dirname + "/" + Constants.CREDENTIALS_FILENAME;
        CredentialsData credentialsData = new CredentialsData(id, username, password);
        Gson g = new Gson();
        String content = g.toJson(credentialsData);

        try (FileWriter file = new FileWriter(filename)) {
            file.write(content);
        } catch (IOException ignored) {
        }
    }

    public static Credentials load(String dirname) {
        String filename = dirname + "/" + Constants.CREDENTIALS_FILENAME;
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

        if (s == null)
            return null;

        Gson g = new Gson();
        Credentials.CredentialsData data = g.fromJson(s, CredentialsData.class);

        return new Credentials(data.username, data.password, data.id);
    }
}
