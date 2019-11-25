package com.rutweet.ruclient.ui.main;

import android.content.Context;
import android.os.Bundle;

import androidx.annotation.Nullable;
import androidx.annotation.StringRes;
import androidx.fragment.app.Fragment;
import androidx.fragment.app.FragmentManager;
import androidx.fragment.app.FragmentPagerAdapter;

import com.rutweet.ruclient.R;
import com.rutweet.ruclient.common.Credentials;

/**
 * A [FragmentPagerAdapter] that returns a fragment corresponding to
 * one of the sections/tabs/pages.
 */
public class SectionsPagerAdapter extends FragmentPagerAdapter {

    @StringRes
    private static final int[] TAB_TITLES = new int[]{R.string.tab_text_2, R.string.tab_text_1, R.string.tab_text_3, R.string.tab_text_4};
    private final Context mContext;
    private Credentials credentials;

    public SectionsPagerAdapter(Context context, FragmentManager fm,
                                Credentials credentials)
    {
        super(fm);
        mContext = context;
        this.credentials = credentials;
    }

    @Override
    public Fragment getItem(int position) {
        Fragment fragment;

        Bundle bundle = new Bundle();
        bundle.putSerializable("credentials", credentials);

        // getItem is called to instantiate the fragment for the given page.
        // Return a PlaceholderFragment (defined as a static inner class below).
        switch (position) {
        case 0:
            fragment = new PersonalFragment();
            break;

        case 2:
            fragment = new FollowingFragment();
            break;

        default:
            fragment = PlaceholderFragment.newInstance(position + 1);
            break;
        }

        fragment.setArguments(bundle);

        return fragment;
    }

    @Nullable
    @Override
    public CharSequence getPageTitle(int position) {
        return mContext.getResources().getString(TAB_TITLES[position]);
    }

    @Override
    public int getCount() {
        return 4;
    }
}