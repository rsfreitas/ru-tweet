package com.rutweet.ruclient;

import android.content.DialogInterface;
import android.os.Bundle;

import com.google.android.material.floatingactionbutton.FloatingActionButton;
import com.google.android.material.tabs.TabLayout;

import androidx.appcompat.app.AlertDialog;
import androidx.viewpager.widget.ViewPager;
import androidx.appcompat.app.AppCompatActivity;

import android.text.InputType;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;
import android.view.ViewGroup;
import android.widget.EditText;
import android.widget.ListView;
import android.widget.Toast;

import com.rutweet.ruclient.common.Constants;
import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.common.Fragment;
import com.rutweet.ruclient.ipc.Login;
import com.rutweet.ruclient.ipc.Tweet;
import com.rutweet.ruclient.ipc.User;
import com.rutweet.ruclient.ui.main.PersonalFragment;
import com.rutweet.ruclient.ui.main.SectionsPagerAdapter;

public class MainActivity extends AppCompatActivity {
    Credentials credentials;
    private static Fragment activeFragment = Fragment.Personal;

    public static void setActiveFragment(Fragment active) {
        activeFragment = active;
    }

    public static Fragment getActiveFragment() {
        return activeFragment;
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        /* Gets credentials */
        final String path = getFilesDir().getAbsolutePath();
        credentials = Credentials.load(path);
        SectionsPagerAdapter sectionsPagerAdapter = new SectionsPagerAdapter(this, getSupportFragmentManager(),
                                                                             credentials);

        ViewPager viewPager = findViewById(R.id.view_pager);
        viewPager.setAdapter(sectionsPagerAdapter);
        TabLayout tabs = findViewById(R.id.tabs);
        tabs.setupWithViewPager(viewPager);
        FloatingActionButton fab = findViewById(R.id.fab);

        fab.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                createNewTweetDialog(credentials);
            }
        });

        /* Does the token file exist? */
        String token = NotificationService.loadToken(path);

        if (token != null) {
            Login.token(credentials.Id(), token);
        }
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        getMenuInflater().inflate(R.menu.main_menu, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        switch (item.getItemId()) {
        case R.id.action_follow:
            followUser(credentials);
            return true;

        case R.id.action_block:
            blockUser(credentials);
            return true;

        default:
            return super.onOptionsItemSelected(item);
        }
    }

    private void blockUser(final Credentials credentials) {
        AlertDialog.Builder builder = new AlertDialog.Builder(this);
        builder.setTitle("Block user");

        final EditText input = new EditText(this);
        input.setInputType(InputType.TYPE_CLASS_TEXT);
        builder.setView(input);

        builder.setPositiveButton("OK", new DialogInterface.OnClickListener() {
            @Override
            public void onClick(DialogInterface dialog, int which) {
                switch (User.Block(credentials.Id(), input.getText().toString())) {
                case 3:
                    Toast.makeText(MainActivity.this, "User does not exist",
                            Toast.LENGTH_LONG).show();

                    break;

                case 0:
                    // TODO: Since we blocked the user we need to reload the
                    //       timeline from his messages.
                    dialog.cancel();
                    break;

                default:
                    Toast.makeText(MainActivity.this, "Error blocking user",
                            Toast.LENGTH_LONG).show();

                    break;
                }
            }
        });

        builder.setNegativeButton("Cancel", new DialogInterface.OnClickListener() {
            @Override
            public void onClick(DialogInterface dialog, int which) {
                dialog.cancel();
            }
        });

        builder.show();
    }

    private void followUser(final Credentials credentials) {
        AlertDialog.Builder builder = new AlertDialog.Builder(this);
        builder.setTitle("Follow user");

        final EditText input = new EditText(this);
        input.setInputType(InputType.TYPE_CLASS_TEXT);
        builder.setView(input);

        builder.setPositiveButton("OK", new DialogInterface.OnClickListener() {
            @Override
            public void onClick(DialogInterface dialog, int which) {
                switch (User.Follow(credentials.Id(), input.getText().toString())) {
                case 3:
                    Toast.makeText(MainActivity.this, "User does not exist",
                                   Toast.LENGTH_LONG).show();

                    break;

                case 0:
                    // TODO: A new user is being followed, we need to update
                    //       the timeline with his messages.
                    dialog.cancel();
                    break;

                default:
                    Toast.makeText(MainActivity.this, "Error following user",
                                   Toast.LENGTH_LONG).show();

                    break;
                }
            }
        });

        builder.setNegativeButton("Cancel", new DialogInterface.OnClickListener() {
            @Override
            public void onClick(DialogInterface dialog, int which) {
                dialog.cancel();
            }
        });

        builder.show();
    }

    private void createNewTweetDialog(final Credentials credentials) {
        AlertDialog.Builder builder = new AlertDialog.Builder(this);
        builder.setTitle("New tweet");

        final EditText input = new EditText(this);
        input.setInputType(InputType.TYPE_CLASS_TEXT);
        builder.setView(input);

        builder.setPositiveButton("OK", new DialogInterface.OnClickListener() {
            @Override
            public void onClick(DialogInterface dialog, int which) {
                Tweet.New(credentials.Username(), credentials.Id(),
                          input.getText().toString());

                // TODO: A new tweet has been created. Update the personal
                //       list.

                dialog.cancel();
            }
        });

        builder.setNegativeButton("Cancel", new DialogInterface.OnClickListener() {
            @Override
            public void onClick(DialogInterface dialog, int which) {
                dialog.cancel();
            }
        });

        builder.show();
    }
}