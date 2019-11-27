package com.rutweet.ruclient.ipc;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

import com.rutweet.ruclient.net.CallServer;

public class User {
    public static List<String> ListFollowing(String from) {
        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);

        Answer answer = CallServer.call("me", args);

        if (answer.Code() == 0) {
            String[] f = answer.Following();

            if (f != null)
                return Arrays.asList(f);
        }

        return null;
    }

    public static int Follow(String from, String user) {
        int rc = -1;

        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);
        args.put("follow", user);

        Answer answer = CallServer.call("follow", args);

        if (answer != null)
            rc = answer.Code();

        return rc;
    }

    public static int Unfollow(String from, String user) {
        int rc = -1;

        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);
        args.put("follow", user);

        Answer answer = CallServer.call("unfollow", args);

        if (answer != null)
            rc = answer.Code();

        return rc;
    }

    public static List<String> ListBlocked(String from) {
        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);

        Answer answer = CallServer.call("me", args);

        if (answer.Code() == 0) {
            String[] f = answer.Blocked();

            if (f != null)
                return Arrays.asList(f);
        }

        return null;
    }

    public static int Block(String from, String user) {
        int rc = -1;

        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);
        args.put("block", user);

        Answer answer = CallServer.call("block", args);

        if (answer != null)
            rc = answer.Code();

        return rc;
    }

    public static int Unblock(String from, String user) {
        int rc = -1;

        HashMap<String, String> args = new HashMap<>();
        args.put("from", from);
        args.put("block", user);

        Answer answer = CallServer.call("unblock", args);

        if (answer != null)
            rc = answer.Code();

        return rc;
    }
}
