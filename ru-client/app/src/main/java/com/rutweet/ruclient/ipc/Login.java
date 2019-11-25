package com.rutweet.ruclient.ipc;

import java.util.HashMap;

import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.net.CallServer;

public class Login {
    /*
     * Checks if current user is already logged inside the server.
     */
    public static boolean isUserLogged(String dirname) {
        try {
            Credentials credentials = Credentials.load(dirname);

            HashMap<String, String> args = new HashMap<>();
            args.put("id", credentials.Id());

            Answer answer = CallServer.call("isLogged", args);

            if (answer.Code() == 0)
                return true;
        } catch (Exception ignored) {
            ignored.printStackTrace();
        }

        return false;
    }

    /*
     * Tries to make the user login inside the server.
     *
     * @return On success returns the user session ID or null otherwise.
     */
    public static String make(String username, String password) {
        HashMap<String, String> args = new HashMap<>();
        args.put("name", username);
        args.put("password", password);

        Answer answer = CallServer.call("login", args);

        if ((answer != null) && (answer.Code() == 0))
            return answer.Id();

        return null;
    }

    /*
     * Tries to create a new user inside the server.
     */
    public static int createUser(String username, String password) {
        int rc = -1;

        HashMap<String, String> args = new HashMap<>();
        args.put("name", username);
        args.put("password", password);

        Answer answer = CallServer.call("addUser", args);

        if (answer != null) {
            rc = answer.Code();
        }

        return rc;
    }
}
