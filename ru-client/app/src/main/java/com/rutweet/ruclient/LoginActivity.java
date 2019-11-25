package com.rutweet.ruclient;

import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;

import com.rutweet.ruclient.common.Credentials;
import com.rutweet.ruclient.ipc.Login;
import com.rutweet.ruclient.net.CallServer;

public class LoginActivity extends AppCompatActivity {
    EditText username,password;
    Button login, create;

    private void callMainActivity() {
        Intent i = new Intent(LoginActivity.this, MainActivity.class);
        finish();
        startActivity(i);
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // 10.0.2.2 is the (default) host IP if we're running inside an emulator
        CallServer.setHostname("10.0.2.2", 8000);

        if (Login.isUserLogged(getFilesDir().getAbsolutePath())) {
            callMainActivity();
            return;
        }

        setContentView(R.layout.login_main);

        username = findViewById(R.id.username);
        password = findViewById(R.id.password);
        login = findViewById(R.id.login);
        create = findViewById(R.id.create);

        login.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                String u = username.getText().toString();
                String p = password.getText().toString();

                String id = Login.make(u, p);

                if (id != null) {
                    Credentials.save(getFilesDir().getAbsolutePath(), u, p, id);
                    callMainActivity();
                    Toast.makeText(LoginActivity.this,"You have authenticated successfully",
                                   Toast.LENGTH_LONG).show();
                } else {
                    Toast.makeText(LoginActivity.this,"Authentication failed",
                                   Toast.LENGTH_LONG).show();
                }
            }
        });

        create.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                String u = username.getText().toString();
                String p = password.getText().toString();

                switch (Login.createUser(u, p)) {
                case 0:
                    String id = Login.make(u, p);
                    Credentials.save(getFilesDir().getAbsolutePath(), u, p, id);
                    callMainActivity();
                    break;

                case 2:
                    Toast.makeText(LoginActivity.this, "User already exists",
                                   Toast.LENGTH_LONG).show();

                    break;

                default:
                    Toast.makeText(LoginActivity.this, "Unable to create user",
                                   Toast.LENGTH_LONG).show();

                    break;
                }
            }
        });
    }
}
