package com.rutweet.ruclient.net;

/**
 * A generic pair class. Based in the Apache Commons Pair version.
 *
 * @param <K> Key type.
 * @param <V> Value type.
 */
public class Pair<K, V> {
    private final K key;
    private final V value;

    Pair(K k, V v) {
        this.key = k;
        this.value = v;
    }

    /**
     * Creates a new Pair object using a factory method.
     *
     * @param k First element of the pair.
     * @param v Second element of the pair.
     * @param <K> The key type.
     * @param <V> The value type.
     *
     * @return A new pair.
     */
    public static <K, V> Pair<K, V> create(K k, V v) {
        return new Pair<>(k, v);
    }

    /**
     * Retrieves the key object.
     *
     * @return Returns the key object.
     */
    public K first() {
        return this.key;
    }

    /**
     * Retrieves the value object.
     *
     * @return Returns the value object.
     */
    public V second() {
        return this.value;
    }
}
